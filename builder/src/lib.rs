use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Ident, PathArguments,
    PathSegment, Type,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let vis = input.vis;

    let builder_name = format_ident!("{}Builder", ident);
    let (idents, types): (Vec<Ident>, Vec<Type>) = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields
                .named
                .into_iter()
                .map(|field| {
                    let ident = field.ident;
                    let ty = field.ty;
                    (ident.unwrap(), ty)
                })
                .unzip(),
            _ => panic!("no unnamed fields are allowed"),
        },
        _ => panic!("expects struct"),
    };

    let builder_fields = idents.iter().zip(&types).map(|(ident, ty)| {
        let t = unwrap_option(ty).unwrap_or(ty);
        quote! {
            #ident: Option<#t>
        }
    });

    let checks = idents
        .iter()
        .zip(&types)
        .filter(|(_, ty)| !is_option(ty))
        .map(|(ident, _)| {
            let err = format!("Required field '{}' is missing", ident.to_string());
            quote! {
                if self.#ident.is_none() {
                    return Err(#err.into())
                }
            }
        });

    let setters = idents.iter().zip(&types).map(|(ident, ty)| {
        let t = unwrap_option(ty).unwrap_or(ty);
        quote! {
            pub fn #ident(&mut self, #ident: #t) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let struct_fields = idents.iter().zip(&types).map(|(ident, ty)| {
        if is_option(ty) {
            quote! {
                #ident: self.#ident.clone()
            }
        } else {
            quote! {
                #ident: self.#ident.clone().unwrap()
            }
        }
    });

    let expand = quote! {
        #vis struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#setters)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                #(#checks)*
                Ok(#ident {
                    #(#struct_fields),*
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#idents: None),*
                }
            }
        }
    };
    proc_macro::TokenStream::from(expand)
}

fn is_option(ty: &Type) -> bool {
    match get_last_path_segment(ty) {
        Some(seg) => seg.ident == "Option",
        _ => false,
    }
}

fn unwrap_option(ty: &Type) -> Option<&Type> {
    if !is_option(ty) {
        return None;
    }
    match get_last_path_segment(ty) {
        Some(seg) => match seg.arguments {
            PathArguments::AngleBracketed(ref args) => {
                args.args.first().and_then(|arg| match arg {
                    &GenericArgument::Type(ref ty) => Some(ty),
                    _ => None,
                })
            }
            _ => None,
        },
        None => None,
    }
}

fn get_last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(path) => path.path.segments.last(),
        _ => None,
    }
}

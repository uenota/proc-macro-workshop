use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let vis = input.vis;

    let builder_name = format_ident!("{}Builder", ident);
    let data = input.data.clone();
    let fields = match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named.into_iter().map(|field| {
                let ident = field.ident;
                let ty = field.ty;
                quote! { #ident: Option<#ty> }
            }),
            _ => panic!("no unnamed fields are allowed"),
        },
        _ => panic!("expected struct"),
    };
    let initial_fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named.into_iter().map(|field| {
                let ident = field.ident;
                quote! { #ident: None }
            }),
            _ => panic!("no unnamed fields are allowed"),
        },
        _ => panic!("expected struct"),
    };

    let expand = quote! {
        #vis struct #builder_name {
           #(#fields),*
        }

        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#initial_fields),*
                }
            }
        }
    };
    proc_macro::TokenStream::from(expand)
}

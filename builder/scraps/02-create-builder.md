derive マクロの入力としては syn::DeriveInput が入ってくる

> ```
> pub struct DeriveInput {
>    pub attrs: Vec<Attribute>,
>    pub vis: Visibility,
>    pub ident: Ident,
>    pub generics: Generics,
>    pub data: Data,
> }
> ```

- attrs は構造体のフィールド名
- vis は構造体の可視性。pub とか pub(crate)とか
- ident は identifier。Rust のコードの識別子。変数名とか。ここでは derive マクロが付与された構造体/列挙体の名前
- generics, data はこの回では触ってないのでよくわからない

特に出力のチェックとかはされてないので何もしなくてもテストは通る。はず。なのでまずは以下のコードが出力されるようにするところから始める。

```
impl Command {
    pub fn builder() {}
}
```

最終的には以下のコードが生成されるようにするのが目的。

```
    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }
```

```
    impl Command {
        pub fn builder() -> CommandBuilder {
            CommandBuilder {
                executable: None,
                args: None,
                env: None,
                current_dir: None,
            }
        }
    }
```

以下の二つの方法でベクタを値としてもつフィールドを更新できるようにする

- ベクタを与える（一括で更新する）
- 要素を与える（一つずつ追加する）

一つずつ追加するための関数名として一括で更新するための関数名と同じ関数名が与えられた場合は一つずつ追加するための関数を優先して生成する

attributes は DeriveInput::attrs に格納されている
-> 違う。DeriveInput の中に入っている attributes は Struct に紐づけられたもの。Field にも attrs があるのでそっちが各フィールドに紐づけられたもの。

```
DeriveInput {
    attrs: [
        Attributes
    ],
    ...
}
```

```
DeriveInput {
    ...
    data: Data::Struct {
        ...
        fields: Fields {
            Fields::Named {
                ...
                named: [
                    Field {
                        ...
                        attrs: [
                            Attribute {
                                ...
                            }
                        ]
                    }
                ]
            }
        }
    }
}
```

Attributes.parse_meta で syn::Meta が出てくる。syn::Meta は以下の構造になっている。

```
Meta {
    ...
    Meta::NameValue {
        ...
        lit: Lit {
            ...
        }
    }
}
```

Optional かどうかの判定には syn::Path::is_ident()は使えないっぽい。以下の説明が関係ある？要調査

> For them to compare equal, it must be the case that:
>
> - the path has no leading colon,
> - the number of path segments is 1,
> - the first path segment has no angle bracketed or parenthesized path arguments, and
> - the ident of the first path segment is equal to the given one.
>
> https://docs.rs/syn/1.0.86/syn/struct.Path.html#method.is_ident

## debug tips

syn クレートの extra-traits フィーチャーを enable にすると Debug トレイトが実装されて dbg!マクロが使えるようになるのでデバッグしやすくなる

```
syn = { version = "1.0.86", features = ["extra-traits"] }
```

cargo expand を使うとマクロによって生成されたコードを出力できるのでこれもデバッグに便利。ただし-Z フラグは nightly でしか使えないので rustup default nightly でツールチェインを変更しておく必要がある
https://github.com/uenota/proc-macro-workshop#debugging-tips

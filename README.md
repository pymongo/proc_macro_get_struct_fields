Answer for https://stackoverflow.com/questions/62837767/how-to-get-the-struct-name-attribute-name-attribute-type-and-traits-data

Use proc_macro_derive(procedural macro)

src/main.rs:

```rust
use print_struct_trait::PrintStruct;

#[derive(PrintStruct)]
struct Point {
    name: String,
    x: i32,
    y: i32,
}

fn main() {
    let point = Point {
        name: "origin".to_string(),
        x: 2,
        y: 3,
    };
    point.print();
}
```

Output:

```
key=name, value=origin, type=String
key=x, value=2, type=i32
key=y, value=3, type=i32
```

print_struct_derive/src/lib.rs:

```rust
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PrintStruct, attributes(serde))]
pub fn derive_signature(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut keys = Vec::new();
    let mut idents = Vec::new();
    let mut types = Vec::new();

    for field in fields.named.iter() {
        let field_name: &syn::Ident = field.ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_key_str = syn::LitStr::new(&name, field.span());
        let type_name = &field.ty;
        keys.push(quote! { #literal_key_str });
        idents.push(&field.ident);
        types.push(type_name.to_token_stream());
    }

    let expanded = quote! {
        impl PrintStruct for #struct_name {
            fn print(&self) {
                #(
                    println!(
                        "key={key}, value={value}, type={type_name}",
                        key = #keys,
                        value = self.#idents,
                        type_name = stringify!(#types)
                    );
                )*
            }
        }
    };
    expanded.into()
}
```

PrintStruct's source code on [github](https://github.com/pymongo/proc_macro_get_struct_fields)

Reference:

- [syn/examples/heapsize/](https://github.com/dtolnay/syn/tree/master/examples/heapsize)
- [Procedural Macros in Rust (part 1) - YouTube](https://www.youtube.com/watch?v=geovSK3wMB8&t=7661s)

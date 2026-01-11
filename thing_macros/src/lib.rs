extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute, Lit, Meta, NestedMeta};

#[proc_macro_derive(Thing, attributes(thing))]
pub fn derive_thing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    // Find #[thing(kind="...")]
    let mut kind_lit = None;
    for attr in input.attrs {
        if attr.path.is_ident("thing") {
            if let Ok(Meta::List(list)) = attr.parse_meta() {
                for nested in list.nested {
                    if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                        if nv.path.is_ident("kind") {
                            if let Lit::Str(s) = nv.lit {
                                kind_lit = Some(s.value());
                            }
                        }
                    }
                }
            }
        }
    }
    
    let kind_const = match kind_lit.as_deref() {
        Some("bytespace.buffer") => quote! { stem::thing::ThingKind::BYTESPACE_BUFFER },
        Some("stream.watch") => quote! { stem::thing::ThingKind::STREAM_WATCH },
        Some("test.node") => quote! { stem::thing::ThingKind::TEST_NODE },
         _ => {
            return syn::Error::new_spanned(name, "Missing or unknown kind in #[thing(kind=\"...\")]").to_compile_error().into();
         }
    };
    
    let expanded = quote! {
        impl stem::thing::Thing for #name {
            const KIND: stem::thing::ThingKind = #kind_const;
        }
    };
    
    TokenStream::from(expanded)
}

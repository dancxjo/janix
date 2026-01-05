extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields,
};

#[proc_macro_derive(Thing, attributes(thing))]
pub fn derive_thing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut kind = None;
    let mut version = 1u32;
    let mut explicit_schema = None;
    let mut _format = String::from("postcard");

    for attr in &input.attrs {
        if attr.path().is_ident("thing") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("kind") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    kind = Some(s.value());
                } else if meta.path.is_ident("version") {
                    let value = meta.value()?;
                    let v: syn::LitInt = value.parse()?;
                    version = v.base10_parse()?;
                } else if meta.path.is_ident("schema") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    explicit_schema = Some(s.value());
                } else if meta.path.is_ident("format") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    _format = s.value();
                }
                Ok(())
            });
        }
    }

    let kind_str = kind.expect("#[thing(kind = \"...\")] is required");
    let kind_hash = fnv1a_64(kind_str.as_bytes());

    let mut fingerprint = format!("{}|{}|v{}|", kind_str, struct_name, version);
    if let Data::Struct(s) = &input.data {
        if let Fields::Named(fields) = &s.fields {
            for field in &fields.named {
                let name = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                let ty_str = quote!(#ty).to_string().replace(" ", "");
                fingerprint.push_str(&format!("{}:{}|", name, ty_str));
            }
        }
    }
    
    let schema_str = explicit_schema.unwrap_or(fingerprint);
    let schema_hash = fnv1a_64(schema_str.as_bytes());

    let expanded = quote! {
        impl thing_codec::Thing for #struct_name {
            const KIND: abi::ids::SymbolId = abi::ids::SymbolId(#kind_hash);
            const SCHEMA_HASH: u64 = #schema_hash;
            const SCHEMA_VERSION: u32 = #version;
            const SCHEMA_STR: &'static str = #schema_str;
        }

        impl #struct_name {
            pub fn create<G: thing_codec::GraphClient>(g: &mut G, value: &Self) -> Result<abi::ids::ThingId, i32> {
                use thing_codec::Thing;
                value.create(g)
            }

            pub fn read<G: thing_codec::GraphClient>(g: &G, id: abi::ids::ThingId) -> Result<Self, i32> {
                <Self as thing_codec::Thing>::read(g, id)
            }

            pub fn write<G: thing_codec::GraphClient>(g: &mut G, id: abi::ids::ThingId, value: &Self) -> Result<(), i32> {
                use thing_codec::Thing;
                value.write(g, id)
            }

            pub fn try_read<G: thing_codec::GraphClient>(g: &G, id: abi::ids::ThingId) -> Result<Option<Self>, i32> {
                <Self as thing_codec::Thing>::try_read(g, id)
            }

            // Compatibility methods
            pub fn encode(&self) -> alloc::vec::Vec<u8> {
                use thing_codec::Thing;
                <Self as Thing>::encode(self)
            }

            pub fn encode_full(&self) -> alloc::vec::Vec<u8> {
                use thing_codec::Thing;
                <Self as Thing>::encode_full(self)
            }
        }
    };

    TokenStream::from(expanded)
}

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ThingType, attributes(thing))]
pub fn derive_thing_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_str = name.to_string();

    // Generate code to construct the TypeDesc
    let desc_construction = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_constructions = fields.named.iter().map(|f| {
                        let fname = f.ident.as_ref().unwrap();
                        let fname_str = fname.to_string();
                        let ftype = &f.ty;
                        quote! {
                            abi::wire::typed::Field {
                                name: abi::symbols::sym(#fname_str),
                                type_desc: <#ftype as ThingType>::type_desc(),
                                optional: false, // TODO: detect Option
                            }
                        }
                    });

                    quote! {
                        abi::wire::typed::TypeDesc::Struct {
                            fields: alloc::vec![#(#field_constructions),*]
                        }
                    }
                }
                Fields::Unnamed(_) => {
                    quote! { abi::wire::typed::TypeDesc::Struct { fields: alloc::vec![] } }
                }
                Fields::Unit => {
                    quote! { abi::wire::typed::TypeDesc::Struct { fields: alloc::vec![] } }
                }
            }
        }
        Data::Enum(_) => {
            // TODO: Enum support
            quote! { abi::wire::typed::TypeDesc::Enum { variants: alloc::vec![] } }
        }
        Data::Union(_) => {
            quote! { abi::wire::typed::TypeDesc::Struct { fields: alloc::vec![] } }
        }
    };

    let expanded = quote! {
        impl ThingType for #name {
            fn type_id() -> abi::wire::typed::TypeId {
                // Compute hash at runtime for v0.2
                // We could optimize this by caching it, but for now just call compute_hash via typedef
                // or replicate logic. Calling typedef() is safer but slower.
                Self::typedef().type_id
            }

            fn type_desc() -> abi::wire::typed::TypeDesc {
                #desc_construction
            }

            fn typedef() -> abi::wire::typed::TypeDef {
                let desc = Self::type_desc();
                let constraints = abi::wire::typed::Constraints::default();
                let codec_id = abi::wire::typed::CodecId::POSTCARD;
                let version = 1;
                let name_sym = abi::symbols::sym(#name_str);

                let type_id = abi::wire::typed::TypeDef::compute_hash(
                    name_sym,
                    version,
                    codec_id,
                    &desc,
                    &constraints
                );

                abi::wire::typed::TypeDef {
                    type_id,
                    name: name_sym,
                    codec_id,
                    version,
                    desc,
                    constraints,
                }
            }

            fn encode(&self) -> Result<alloc::vec::Vec<u8>, postcard::Error> {
                 postcard::to_allocvec(self)
            }

            fn decode(bytes: &[u8]) -> Result<Self, postcard::Error> {
                postcard::from_bytes(bytes)
            }

            fn validate(&self) -> Result<(), ()> {
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

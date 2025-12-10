use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, Lit, Meta, MetaNameValue, Type, parse_macro_input,
};

fn extract_description(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("thing") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the tokens properly using syn
                if let Ok(meta_name_value) = syn::parse2::<MetaNameValue>(meta_list.tokens.clone())
                {
                    if meta_name_value.path.is_ident("description") {
                        if let Expr::Lit(expr_lit) = &meta_name_value.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
            }
        }
    }

    // No description found
    None
}

fn normalize_type_name(ty: &Type) -> String {
    let mut ty_str = quote!(#ty).to_string();
    ty_str.retain(|c| !c.is_whitespace());
    while ty_str.starts_with("::") {
        ty_str = ty_str[2..].to_string();
    }
    ty_str
}

#[proc_macro_derive(Thing, attributes(thing))]
pub fn derive_thing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let kind_str = name.to_string();

    // Extract description and generate compile error if missing
    let description = match extract_description(&input.attrs) {
        Some(desc) => desc,
        None => {
            return TokenStream::from(quote! {
                compile_error!("Thing derive requires a #[thing(description = \"...\")] attribute");
            });
        }
    };

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Thing derive only supports named fields"),
        },
        _ => panic!("Thing derive only supports structs"),
    };

    let to_props_arms: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let name = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            if name == "id" {
                return None;
            }
            let ty = &f.ty;
            let ty_str = normalize_type_name(ty);

            let val_expr = match ty_str.as_str() {
                "u64" => quote! { ::abi::PropValue::U64(self.#name as u64) },
                "u32" => quote! { ::abi::PropValue::U64(self.#name as u64) },
                "u16" => quote! { ::abi::PropValue::U64(self.#name as u64) },
                "u8" => quote! { ::abi::PropValue::U64(self.#name as u64) },
                "i64" => quote! { ::abi::PropValue::I64(self.#name as i64) },
                "i32" => quote! { ::abi::PropValue::I64(self.#name as i64) },
                "i16" => quote! { ::abi::PropValue::I64(self.#name as i64) },
                "i8" => quote! { ::abi::PropValue::I64(self.#name as i64) },
                "bool" => quote! { ::abi::PropValue::Bool(self.#name as bool) },
                "alloc::string::String" | "String" => {
                    quote! { ::abi::PropValue::Str(self.#name.clone()) }
                }
                "&'staticstr" => {
                    quote! { ::abi::PropValue::Str(::alloc::string::String::from(self.#name)) }
                }
                "char" => {
                    quote! { ::abi::PropValue::Str(::alloc::string::String::from(self.#name)) }
                }
                "ThingId" | "abi::ThingId" => {
                    quote! { ::abi::PropValue::U64(self.#name.0) }
                }
                other => panic!("Unsupported type for Thing derive: {}", other),
            };

            Some(quote! {
                out.push((stringify!(#name), #val_expr));
            })
        })
        .collect();

    let from_props_arms: Vec<_> = fields
        .iter()
        .map(|f| {
            let name = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            let ty = &f.ty;
            let ty_str = normalize_type_name(ty);

            if name == "id" {
                return quote! { #name: id };
            }

            let match_arm = match ty_str.as_str() {
                "u64" => quote! {
                    if let ::abi::PropValue::U64(val) = *v { val } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "u32" => quote! {
                    if let ::abi::PropValue::U64(val) = *v { val as u32 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "u16" => quote! {
                    if let ::abi::PropValue::U64(val) = *v { val as u16 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "u8" => quote! {
                    if let ::abi::PropValue::U64(val) = *v { val as u8 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "i64" => quote! {
                    if let ::abi::PropValue::I64(val) = *v { val } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "i32" => quote! {
                    if let ::abi::PropValue::I64(val) = *v { val as i32 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "i16" => quote! {
                    if let ::abi::PropValue::I64(val) = *v { val as i16 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "i8" => quote! {
                    if let ::abi::PropValue::I64(val) = *v { val as i8 } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "bool" => quote! {
                    if let ::abi::PropValue::Bool(val) = *v { val } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "alloc::string::String" | "String" => quote! {
                    if let ::abi::PropValue::Str(ref val) = *v {
                        val.clone()
                    } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "&'staticstr" => {
                    quote! {
                        if let ::abi::PropValue::Str(ref val) = *v {
                            ::alloc::string::String::from(val.as_str())
                        } else {
                            panic!("Type mismatch for {}", stringify!(#name))
                        }
                    }
                }
                "ThingId" | "abi::ThingId" => quote! {
                if let ::abi::PropValue::U64(val) = *v {
                    ::abi::ThingId(val)
                    } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                "char" => quote! {
                    if let ::abi::PropValue::Str(ref val) = *v {
                        val.chars().next().unwrap_or('\0')
                    } else {
                        panic!("Type mismatch for {}", stringify!(#name))
                    }
                },
                other => panic!("Unsupported type for Thing derive: {}", other),
            };

            quote! {
                #name: {
                    let mut found = None;
                    for prop in props {
                        if let Some((k, v)) = prop {
                            if *k == stringify!(#name) {
                                found = Some(#match_arm);
                                break;
                            }
                        }
                    }
                    found.expect(concat!("Missing property: ", stringify!(#name)))
                }
            }
        })
        .collect();

    let schema_entries: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let name = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            if name == "id" {
                return None;
            }
            let ty = &f.ty;
            let ty_str = normalize_type_name(ty);

            let prop_ty_expr = match ty_str.as_str() {
                "u64" | "u32" | "u16" | "u8" | "ThingId" | "abi::ThingId" => {
                    quote! { ::abi::PropType::U64 }
                }
                "i64" | "i32" | "i16" | "i8" => quote! { ::abi::PropType::I64 },
                "bool" => quote! { ::abi::PropType::Bool },
                "alloc::string::String" | "String" => quote! { ::abi::PropType::Str },
                "char" => quote! { ::abi::PropType::Str },
                "&'staticstr" => quote! { ::abi::PropType::Str },
                other => {
                    let error_msg = format!(
                        "Unsupported type '{}' for Thing derive schema. Only numeric primitives, bool, and String-based fields are supported.",
                        other
                    );
                    quote! { compile_error!(#error_msg) }
                }
            };

            Some(quote! {
                ( stringify!(#name), #prop_ty_expr )
            })
        })
        .collect();

    let expanded = quote! {
        impl ::abi::Thing for #name {
            const KIND: &'static str = #kind_str;
            const DESCRIPTION: &'static str = #description;

            fn to_props(&self, out: &mut ::alloc::vec::Vec<(::abi::PropKey, ::abi::PropValue)>) {
                #(#to_props_arms)*
            }

            fn from_props(id: ::abi::ThingId, props: &[Option<(::abi::PropKey, ::abi::PropValue)>]) -> Self {
                let _ = id;
                Self {
                    #(#from_props_arms),*
                }
            }

            fn schema() -> &'static [(&'static str, ::abi::PropType)] {
                static SCHEMA: &[(&'static str, ::abi::PropType)] = &[
                    #(#schema_entries),*
                ];
                SCHEMA
            }
        }
    };

    TokenStream::from(expanded)
}

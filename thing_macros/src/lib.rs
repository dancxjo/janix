use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, Attribute, Meta, Lit, MetaNameValue, Expr};

fn extract_description(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("thing") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the tokens properly using syn
                if let Ok(meta_name_value) = syn::parse2::<MetaNameValue>(meta_list.tokens.clone()) {
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

    let to_props_arms = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ty_str = quote!(#ty).to_string();

        let val_expr = if ty_str == "u64" {
            quote! { ::abi::PropValue::U64(self.#name as u64) }
        } else if ty_str == "i64" {
            quote! { ::abi::PropValue::I64(self.#name as i64) }
        } else if ty_str == "bool" {
            quote! { ::abi::PropValue::Bool(self.#name as bool) }
        } else if ty_str == "alloc :: string :: String" || ty_str == "String" {
            quote! { ::abi::PropValue::Str(self.#name.clone()) }
        } else if ty_str == "& 'static str" || ty_str == "&'static str" {
            quote! { ::abi::PropValue::Str(self.#name.to_string()) }
        } else {
            quote! { panic!("Unsupported type for Thing derive: {}", #ty_str) }
        };

        quote! {
            out.push((stringify!(#name), #val_expr));
        }
    });

    let from_props_arms = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ty_str = quote!(#ty).to_string();

        let match_arm = if ty_str == "u64" {
            quote! {
                if let ::abi::PropValue::U64(val) = *v { val } else {
                    panic!("Type mismatch for {}", stringify!(#name))
                }
            }
        } else if ty_str == "i64" {
            quote! {
                if let ::abi::PropValue::I64(val) = *v { val } else {
                    panic!("Type mismatch for {}", stringify!(#name))
                }
            }
        } else if ty_str == "bool" {
            quote! {
                if let ::abi::PropValue::Bool(val) = *v { val } else {
                    panic!("Type mismatch for {}", stringify!(#name))
                }
            }
        } else if ty_str == "alloc :: string :: String" || ty_str == "String" {
            quote! {
                if let ::abi::PropValue::Str(ref val) = *v {
                    val.clone()
                } else {
                    panic!("Type mismatch for {}", stringify!(#name))
                }
            }
        } else if ty_str == "& 'static str" || ty_str == "&'static str" {
            quote! { compile_error!("from_props does not support &'static str fields") }
        } else {
            quote! { panic!("Unsupported type for Thing derive: {}", #ty_str) }
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
    });

    let schema_entries = fields.iter().map(|f| {
        let name = &f.ident;
        let name_str = name.as_ref().unwrap().to_string();
        let ty = &f.ty;
        let ty_str = quote!(#ty).to_string();

        let prop_ty_expr = if ty_str == "u64" {
            quote! { ::abi::PropType::U64 }
        } else if ty_str == "i64" {
            quote! { ::abi::PropType::I64 }
        } else if ty_str == "bool" {
            quote! { ::abi::PropType::Bool }
        } else if ty_str == "alloc :: string :: String" || ty_str == "String" {
            quote! { ::abi::PropType::Str }
        } else {
            let error_msg = format!(
                "Unsupported type '{}' for Thing derive schema. Only u64, i64, bool, and String are supported.",
                ty_str
            );
            quote! { compile_error!(#error_msg) }
        };

        quote! {
            ( #name_str, #prop_ty_expr )
        }
    });

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

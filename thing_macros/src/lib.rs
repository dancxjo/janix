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
    None
}

fn extract_kind(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("thing") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(meta_name_value) = syn::parse2::<MetaNameValue>(meta_list.tokens.clone())
                {
                    if meta_name_value.path.is_ident("kind") {
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
    None
}

fn extract_field_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("thing") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(meta_name_value) = syn::parse2::<MetaNameValue>(meta_list.tokens.clone())
                {
                    if meta_name_value.path.is_ident("rename") {
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
    None
}

fn extract_field_via(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("thing") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(meta_name_value) = syn::parse2::<MetaNameValue>(meta_list.tokens.clone())
                {
                    if meta_name_value.path.is_ident("via") {
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

fn extract_inner_type(ty_str: &str) -> Option<String> {
    if ty_str.starts_with("Option<") && ty_str.ends_with(">") {
        Some(ty_str[7..ty_str.len() - 1].to_string())
    } else {
        None
    }
}

#[proc_macro_derive(Thing, attributes(thing))]
pub fn derive_thing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut kind_str = name.to_string();
    if let Some(k) = extract_kind(&input.attrs) {
        kind_str = k;
    }

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
            let ident = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            if ident == "id" {
                return None;
            }

            let prop_name = extract_field_rename(&f.attrs).unwrap_or(ident.to_string());
            let via_type = extract_field_via(&f.attrs);

            let ty = &f.ty;
            let full_ty_str = normalize_type_name(ty);
            let (ty_str, is_option) = match extract_inner_type(&full_ty_str) {
                Some(inner) => (inner, true),
                None => (full_ty_str, false),
            };

            let effective_type = if let Some(via) = &via_type {
                via.clone()
            } else {
                ty_str.clone()
            };

            let val_conversion = match effective_type.as_str() {
                "u64" => quote! { ::thing_models::PropValue::U64(*val as u64) },
                "u32" => quote! { ::thing_models::PropValue::U64(*val as u64) },
                "u16" => quote! { ::thing_models::PropValue::U64(*val as u64) },
                "u8" => quote! { ::thing_models::PropValue::U64(*val as u64) },
                "i64" => quote! { ::thing_models::PropValue::I64(*val as i64) },
                "i32" => quote! { ::thing_models::PropValue::I64(*val as i64) },
                "i16" => quote! { ::thing_models::PropValue::I64(*val as i64) },
                "i8" => quote! { ::thing_models::PropValue::I64(*val as i64) },
                "bool" => quote! { ::thing_models::PropValue::Bool(*val as bool) },
                "alloc::string::String" | "String" => {
                    if via_type.is_some() {
                         quote! { ::thing_models::PropValue::Str(val.to_string()) }
                    } else {
                         quote! { ::thing_models::PropValue::Str(val.clone()) }
                    }
                }
                "&'staticstr" => {
                    quote! { ::thing_models::PropValue::Str(::alloc::string::String::from(*val)) }
                }
                "char" => {
                    quote! { ::thing_models::PropValue::Str(::alloc::string::String::from(*val)) }
                }
                "ThingId" | "abi::ThingId" => {
                    quote! { ::thing_models::PropValue::U64(val.0) }
                }
                other => panic!("Unsupported type for Thing derive: {}", other),
            };

            if is_option {
                Some(quote! {
                    if let Some(ref val) = self.#ident {
                        out.push((::alloc::string::String::from(#prop_name), #val_conversion));
                    }
                })
            } else {
                Some(quote! {
                    let val = &self.#ident;
                    out.push((::alloc::string::String::from(#prop_name), #val_conversion));
                })
            }
        })
        .collect();

    let from_props_arms: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            let ty = &f.ty;
            let full_ty_str = normalize_type_name(ty);
            let (ty_str, is_option) = match extract_inner_type(&full_ty_str) {
                Some(inner) => (inner, true),
                None => (full_ty_str, false),
            };

            if ident == "id" {
                return quote! { #ident: id };
            }

            let prop_name = extract_field_rename(&f.attrs).unwrap_or(ident.to_string());
            let via_type = extract_field_via(&f.attrs);
             let effective_type = if let Some(via) = &via_type {
                via.clone()
            } else {
                ty_str.clone()
            };

            let mut match_arm = match effective_type.as_str() {
                "u64" => quote! {
                    if let ::thing_models::PropValue::U64(val) = *v { val } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "u32" => quote! {
                    if let ::thing_models::PropValue::U64(val) = *v { val as u32 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "u16" => quote! {
                    if let ::thing_models::PropValue::U64(val) = *v { val as u16 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "u8" => quote! {
                    if let ::thing_models::PropValue::U64(val) = *v { val as u8 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "i64" => quote! {
                    if let ::thing_models::PropValue::I64(val) = *v { val } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "i32" => quote! {
                    if let ::thing_models::PropValue::I64(val) = *v { val as i32 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "i16" => quote! {
                    if let ::thing_models::PropValue::I64(val) = *v { val as i16 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "i8" => quote! {
                    if let ::thing_models::PropValue::I64(val) = *v { val as i8 } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "bool" => quote! {
                    if let ::thing_models::PropValue::Bool(val) = *v { val } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "alloc::string::String" | "String" => quote! {
                    if let ::thing_models::PropValue::Str(ref val) = *v {
                        val.clone()
                    } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "&'staticstr" => {
                    quote! {
                        if let ::thing_models::PropValue::Str(ref val) = *v {
                            ::alloc::string::String::from(val.as_str())
                        } else {
                            panic!("Type mismatch for {}", #prop_name)
                        }
                    }
                }
                "ThingId" | "abi::ThingId" => quote! {
                if let ::thing_models::PropValue::U64(val) = *v {
                    ::abi::ThingId(val)
                    } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                "char" => quote! {
                    if let ::thing_models::PropValue::Str(ref val) = *v {
                        val.chars().next().unwrap_or('\0')
                    } else {
                        panic!("Type mismatch for {}", #prop_name)
                    }
                },
                other => panic!("Unsupported type for Thing derive: {}", other),
            };

            if via_type.is_some() {
                 match_arm = quote! {
                     {
                         let s: ::alloc::string::String = #match_arm;
                         s.parse().expect("Failed to parse via type")
                     }
                 }
            }

            if is_option {
                quote! {
                    #ident: {
                        let mut found = None;
                        for prop in props {
                            if let Some((k, v)) = prop {
                                if *k == #prop_name {
                                    found = Some(Some(#match_arm));
                                    break;
                                }
                            }
                        }
                        found.unwrap_or(None)
                    }
                }
            } else {
                quote! {
                    #ident: {
                        let mut found = None;
                        for prop in props {
                            if let Some((k, v)) = prop {
                                if *k == #prop_name {
                                    found = Some(#match_arm);
                                    break;
                                }
                            }
                        }
                        found.expect(concat!("Missing property: ", #prop_name))
                    }
                }
            }
        })
        .collect();

    let schema_entries: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let ident = f
                .ident
                .as_ref()
                .expect("Thing derive only supports named fields");
            if ident == "id" {
                return None;
            }

            let prop_name = extract_field_rename(&f.attrs).unwrap_or(ident.to_string());
             let via_type = extract_field_via(&f.attrs);

            let ty = &f.ty;
            let full_ty_str = normalize_type_name(ty);
            let (ty_str, _is_option) = match extract_inner_type(&full_ty_str) {
                Some(inner) => (inner, true),
                None => (full_ty_str, false),
            };

             let effective_type = if let Some(via) = &via_type {
                via.clone()
            } else {
                ty_str.clone()
            };

            let prop_ty_expr = match effective_type.as_str() {
                "u64" | "u32" | "u16" | "u8" | "ThingId" | "abi::ThingId" => {
                    quote! { ::thing_models::PropType::U64 }
                }
                "i64" | "i32" | "i16" | "i8" => quote! { ::thing_models::PropType::I64 },
                "bool" => quote! { ::thing_models::PropType::Bool },
                "alloc::string::String" | "String" => quote! { ::thing_models::PropType::Str },
                "char" => quote! { ::thing_models::PropType::Str },
                "&'staticstr" => quote! { ::thing_models::PropType::Str },
                other => {
                    let error_msg = format!(
                        "Unsupported type '{}' for Thing derive schema. Only numeric primitives, bool, and String-based fields are supported.",
                        other
                    );
                    quote! { compile_error!(#error_msg) }
                }
            };

            Some(quote! {
                ( #prop_name, #prop_ty_expr )
            })
        })
        .collect();

    let expanded = quote! {
        impl ::thing_models::Thing for #name {
            const KIND: &'static str = #kind_str;
            const DESCRIPTION: &'static str = #description;

            fn to_props(&self, out: &mut ::alloc::vec::Vec<(::thing_models::PropKey, ::thing_models::PropValue)>) {
                #(#to_props_arms)*
            }

            fn from_props(id: ::abi::ThingId, props: &[Option<(::thing_models::PropKey, ::thing_models::PropValue)>]) -> Self {
                let _ = id;
                Self {
                    #(#from_props_arms),*
                }
            }

            fn schema() -> &'static [(&'static str, ::thing_models::PropType)] {
                static SCHEMA: &[(&'static str, ::thing_models::PropType)] = &[
                    #(#schema_entries),*
                ];
                SCHEMA
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let original_main = input.block;
    let original_sig = input.sig;

    // Rename user's main to avoid conflict
    let mut modified_sig = original_sig.clone();
    modified_sig.ident = syn::Ident::new("thing_os_app_main", modified_sig.ident.span());

    let expanded = quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn main() -> ! {
            ::thing_os::heap::init_user_heap();
            thing_os_app_main();
            unsafe {
                 ::thing_os::syscalls::syscall(::thing_os::abi::KernelRequest::ExitThread);
            }
            loop {}
        }

        #modified_sig #original_main
    };
    TokenStream::from(expanded)
}

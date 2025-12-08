use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Thing)]
pub fn derive_thing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let kind_str = name.to_string();

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
        
        let val_expr = match ty_str.as_str() {
            "u64" => quote! { ::abi::PropValue::U64(self.#name as u64) },
            "i64" => quote! { ::abi::PropValue::I64(self.#name as i64) },
            "bool" => quote! { ::abi::PropValue::Bool(self.#name as bool) },
            _ => quote! { panic!("Unsupported type for Thing derive") },
        };

        quote! {
            out.push((stringify!(#name), #val_expr));
        }
    });

    let from_props_arms = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let ty_str = quote!(#ty).to_string();

        let match_arm = match ty_str.as_str() {
            "u64" => quote! {
                if let ::abi::PropValue::U64(val) = *v { val } else { panic!("Type mismatch for {}", stringify!(#name)) }
            },
            "i64" => quote! {
                if let ::abi::PropValue::I64(val) = *v { val } else { panic!("Type mismatch for {}", stringify!(#name)) }
            },
            "bool" => quote! {
                if let ::abi::PropValue::Bool(val) = *v { val } else { panic!("Type mismatch for {}", stringify!(#name)) }
            },
            _ => quote! { panic!("Unsupported type for Thing derive") },
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

        let prop_ty_expr = match ty_str.as_str() {
            "u64" => quote! { ::abi::PropType::U64 },
            "i64" => quote! { ::abi::PropType::I64 },
            "bool" => quote! { ::abi::PropType::Bool },
            _ => {
                let error_msg = format!("Unsupported type '{}' for Thing derive schema. Only u64, i64, and bool are supported.", ty_str);
                quote! { compile_error!(#error_msg) }
            }
        };

        quote! {
            ( #name_str, #prop_ty_expr )
        }
    });

    let expanded = quote! {
        impl ::userland_std::Thing for #name {
            const KIND: &'static str = #kind_str;

            fn to_props(&self, out: &mut ::std::vec::Vec<(::abi::PropKey, ::abi::PropValue)>) {
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

#![doc = include_str!("../README.md")]

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Expr, Ident, Token, Type,
};

struct CvarDef {
    name: Ident,
    ty: Type,
    value: Expr,
}

impl Parse for CvarDef {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(CvarDef { name, ty, value })
    }
}

/// TODO downside - can't specify attributes
#[proc_macro]
pub fn cvars(input: TokenStream) -> TokenStream {
    // TODO what happens to doc comments above cvars?

    let parser = Punctuated::<CvarDef, Token![,]>::parse_terminated;
    let punctuated = parser.parse(input).unwrap();
    let cvar_defs: Vec<_> = punctuated.iter().collect();

    let names: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.name).collect();
    let tys: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.ty).collect();
    let values: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.value).collect();

    let expanded = quote! {
        #[derive(Debug, Clone, Default)]
        pub struct Cvars {
            #(
                pub #names: #tys,
            )*
        }

        impl Cvars {
            fn new() -> Self {
                Self {
                    #(
                        #names: #values,
                    )*
                }

            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(Cvars)]
pub fn derive(input: TokenStream) -> TokenStream {
    // TODO public API?
    //  put trait in private mod?
    //  underscored named like serde?
    //  check what's in docs (of this crate and of generated code)
    //  getters/setters should return Result

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // TODO error msg
    let named_fields = match input.data {
        syn::Data::Struct(struct_data) => match struct_data.fields {
            syn::Fields::Named(named_fields) => named_fields,
            syn::Fields::Unnamed(_) => unimplemented!(),
            syn::Fields::Unit => unimplemented!(),
        },
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    let mut fields = Vec::new();
    let mut tys = Vec::new();
    for field in &named_fields.named {
        let ident = field.ident.as_ref().unwrap();
        fields.push(ident);
        tys.push(&field.ty);
    }

    let unique_tys: HashSet<_> = tys.iter().collect();
    let mut trait_impls = Vec::new();
    for unique_ty in unique_tys {
        let mut getter_arms = Vec::new();
        let mut setter_arms = Vec::new();

        for i in 0..fields.len() {
            let field = fields[i];
            let ty = tys[i];
            if ty == *unique_ty {
                let getter_arm = quote! {
                    stringify!(#field) => cvars.#field,
                };
                getter_arms.push(getter_arm);

                let setter_arm = quote! {
                    stringify!(#field) => cvars.#field = value,
                };
                setter_arms.push(setter_arm);
            }
        }

        // TODO Better error messages (report type of value and of field if both exist)
        // TODO Is there a sane way to automatically convert?
        //      e.g. integers default to i32 even though cvar type is usize
        let trait_impl = quote! {
            impl CvarValue for #unique_ty {
                fn get(cvars: &Cvars, cvar_name: &str) -> Self {
                    match cvar_name {
                        #( #getter_arms )*
                        _ => panic!("Cvar named {} with type {} not found", cvar_name, stringify!(#unique_ty)),
                    }
                }

                fn set(cvars: &mut Cvars, cvar_name: &str, value: Self) {
                    match cvar_name {
                        #( #setter_arms )*
                        _ => panic!("Cvar named {} with type {} not found", cvar_name, stringify!(#unique_ty)),
                    }
                }
            }
        };
        trait_impls.push(trait_impl);
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn get<T: CvarValue>(&self, cvar_name: &str) -> T {
                CvarValue::get(self, cvar_name)
            }

            pub fn get_string(&self, cvar_name: &str) -> String {
                match cvar_name {
                    // This doesn't need to be dispatched via CvarValue, it uses Display instead.
                    #( stringify!(#fields) => self.#fields.to_string(), )*
                    _ => panic!("Cvar named {} not found", cvar_name),
                }
            }

            pub fn set<T: CvarValue>(&mut self, cvar_name: &str, value: T) {
                CvarValue::set(self, cvar_name, value);
            }

            pub fn set_str(&mut self, cvar_name: &str, str_value: &str) {
                match cvar_name {
                    // This doesn't need to be dispatched via CvarValue, it uses FromStr instead.
                    #( stringify!(#fields) => self.#fields = str_value.parse().unwrap(), )*
                    _ => panic!("Cvar named {} not found", cvar_name),
                }
            }
        }

        /// This trait is needed to dispatch cvar get/set based on its type.
        /// You're not meant to impl it yourself, it's done automatically
        /// for all types used as cvars.
        pub trait CvarValue {
            fn get(cvars: &Cvars, cvar_name: &str) -> Self;
            fn set(cvars: &mut Cvars, cvar_name: &str, value: Self);
        }

        #( #trait_impls )*
    };
    TokenStream::from(expanded)
}

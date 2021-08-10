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

/// **Experimental**. Generate the whole `Cvars` struct.
///
/// This allows both the field types and their initial values to be defined on one line.
/// The downside is that users can't specify attributes on the struct.
///
/// The generated code contains the struct definition and an impl block
/// with a `new` function which sets the initial values.
///
/// **Open question**: Should the generated `Default` use the specified initial values
/// or default values of the field types?
///
/// **Currently will crash on doc comments**
///
/// # Example
///
/// ```rust
/// use cvars::cvars;
///
/// cvars! {
///     g_rocket_launcher_ammo_max: i32 = 20,
///     g_rocket_launcher_damage: f32 = 75.0,
/// }
/// ```
#[proc_macro]
pub fn cvars(input: TokenStream) -> TokenStream {
    // LATER doc comments above cvars

    let parser = Punctuated::<CvarDef, Token![,]>::parse_terminated;
    let punctuated = parser.parse(input).unwrap();
    let cvar_defs: Vec<_> = punctuated.iter().collect();

    let names: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.name).collect();
    let tys: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.ty).collect();
    let values: Vec<_> = cvar_defs.iter().map(|cvar_def| &cvar_def.value).collect();

    let expanded = quote! {
        #[derive(Debug, Clone, Default, ::cvars::SetGet)]
        pub struct Cvars {
            #(
                pub #names: #tys,
            )*
        }

        impl Cvars {
            pub fn new() -> Self {
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

/// Generate setters and getters that take cvar names as string.
///
/// This requires all types used as cvars to impl `FromStr` and `Display`.
///
/// The generated methods:
/// - `get` - take cvar name as string and return its value as the correct type
/// - `get_string` - take cvar name as string and return its value as a `String`
/// - `set` - take cvar name as string and its new value as the correct type
/// - `set_str` - take cvar name as string and its new value as a `&str`
///
/// Since all 4 functions can fail, all return `Result`s.
///
/// # Example
///
/// ```rust
/// use cvars::SetGet;
///
/// #[derive(SetGet)]
/// pub struct Cvars {
///     g_rocket_launcher_ammo_max: i32,
///     g_rocket_launcher_damage: f32,
/// }
///
/// impl Cvars {
///     pub fn new() -> Self {
///         Self {
///             g_rocket_launcher_ammo_max: 20,
///             g_rocket_launcher_damage: 75.0,
///         }
///     }
/// }
/// ```
#[proc_macro_derive(SetGet)]
pub fn derive(input: TokenStream) -> TokenStream {
    // TODO public API?
    //  put trait in private mod?
    //  underscored named like serde?
    //  check what's in docs (of this crate and of generated code)

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
        let ident = field.ident.as_ref().expect("ident was None");
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
                    stringify!(#field) => ::core::result::Result::Ok(cvars.#field),
                };
                getter_arms.push(getter_arm);

                let setter_arm = quote! {
                    stringify!(#field) => ::core::result::Result::Ok(cvars.#field = value),
                };
                setter_arms.push(setter_arm);
            }
        }

        // TODO Is there a sane way to automatically convert?
        //      e.g. integers default to i32 even though cvar type is usize
        let trait_impl = quote! {
            impl CvarValue for #unique_ty {
                fn get(cvars: &Cvars, cvar_name: &str) -> ::core::result::Result<Self, String> {
                    match cvar_name {
                        #( #getter_arms )*
                        _ => ::core::result::Result::Err(format!(
                            "Cvar named {} with type {} not found",
                            cvar_name,
                            stringify!(#unique_ty)
                        )),
                    }
                }

                fn set(cvars: &mut Cvars, cvar_name: &str, value: Self) -> ::core::result::Result<(), String> {
                    match cvar_name {
                        #( #setter_arms )*
                        _ => ::core::result::Result::Err(format!(
                            "Cvar named {} with type {} not found",
                            cvar_name,
                            stringify!(#unique_ty),
                        )),
                    }
                }
            }
        };
        trait_impls.push(trait_impl);
    }

    let expanded = quote! {
        impl #struct_name {
            /// Finds the cvar whose name matches `cvar_name` and returns it's value.
            ///
            /// Returns `Err` if the cvar doesn't exist.
            pub fn get<T: CvarValue>(&self, cvar_name: &str) -> ::core::result::Result<T, String> {
                CvarValue::get(self, cvar_name)
            }

            /// Finds the cvar whose name matches `cvar_name` and returns it's value as a `String`.
            ///
            /// Returns `Err` if the cvar doesn't exist.
            pub fn get_string(&self, cvar_name: &str) -> ::core::result::Result<String, String> {
                match cvar_name {
                    // This doesn't need to be dispatched via CvarValue, it uses Display instead.
                    #( stringify!(#fields) => ::core::result::Result::Ok(self.#fields.to_string()), )*
                    _ => ::core::result::Result::Err(format!(
                        "Cvar named {} not found",
                        cvar_name,
                    )),
                }
            }

            /// Finds the cvar whose name matches `cvar_name` and sets it to `value`.
            ///
            /// Returns `Err` if the cvar doesn't exist or its type doesn't match that of `value`.
            pub fn set<T: CvarValue>(&mut self, cvar_name: &str, value: T) -> ::core::result::Result<(), String> {
                CvarValue::set(self, cvar_name, value)
            }

            /// Finds the cvar whose name matches `cvar_name`, tries to parse `str_value` to its type and sets it to the parsed value.
            ///
            /// Returns `Err` if the cvar doesn't exist or if `str_value` fails to parse to its type.
            pub fn set_str(&mut self, cvar_name: &str, str_value: &str) -> ::core::result::Result<(), String> {
                match cvar_name {
                    // This doesn't need to be dispatched via CvarValue, it uses FromStr instead.
                    #( stringify!(#fields) => match str_value.parse() {
                        ::core::result::Result::Ok(val) => ::core::result::Result::Ok(self.#fields = val),
                        ::core::result::Result::Err(err) => ::core::result::Result::Err(format!("failed to parse {} as type {}: {}",
                            str_value,
                            stringify!(#tys),
                            err,
                        ))
                    }, )*
                    _ => ::core::result::Result::Err(format!(
                        "Cvar named {} not found",
                        cvar_name
                    )),
                }
            }
        }

        /// This trait is needed to dispatch cvar get/set based on its type.
        /// You're not meant to impl it yourself, it's done automatically
        /// for all types used as cvars.
        pub trait CvarValue {
            fn get(cvars: &Cvars, cvar_name: &str) -> ::core::result::Result<Self, String>
                where Self: Sized;
            fn set(cvars: &mut Cvars, cvar_name: &str, value: Self) -> ::core::result::Result<(), String>;
        }

        #( #trait_impls )*
    };
    TokenStream::from(expanded)
}

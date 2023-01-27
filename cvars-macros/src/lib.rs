//! Proc macros for generating methods on your Cvars struct.
//!
//! This crate is not meant to be used directly,
//! its macros are reexported by the `cvars` crate
//! and might not work if used directly due to language limitations.

// LATER The macros don't need to depend on each other but should call a common function to generate the code.

#![warn(missing_docs)]

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Expr, Field, Ident, Meta, MetaList, NestedMeta, Token, Type,
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
/// **Currently will crash on doc comments and (other) attributes**
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
    // TODO doc comments above cvars
    // TODO cvars(skip)
    // LATER proper error reporting (no unwraps, expect only for infallible)

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

        #[automatically_derived]
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
#[proc_macro_derive(SetGet, attributes(cvars))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_name = input.ident;

    let named_fields = match input.data {
        syn::Data::Struct(struct_data) => match struct_data.fields {
            syn::Fields::Named(named_fields) => named_fields,
            syn::Fields::Unnamed(_) => panic!("tuple structs are not supported, use named fields"),
            syn::Fields::Unit => panic!("unit structs are not supported, use curly braces"),
        },
        syn::Data::Enum(_) => panic!("enums are not supported, use a struct"),
        syn::Data::Union(_) => panic!("unions are not supported, use a struct"),
    };

    // Get the list of all cvars and their types
    let mut fields = Vec::new();
    let mut tys = Vec::new();
    for field in &named_fields.named {
        if skip_field(field) {
            continue;
        }

        let ident = field.ident.as_ref().expect("ident was None");
        fields.push(ident);
        tys.push(&field.ty);
    }

    // Get the set of types used as cvars.
    // We need to impl SetGetType for them and it needs to be done
    // once per type, not once per cvar.
    let unique_tys: HashSet<_> = tys.iter().collect();
    let mut trait_impls = Vec::new();
    for unique_ty in unique_tys {
        let mut getter_arms = Vec::new();
        let mut setter_arms = Vec::new();

        for i in 0..fields.len() {
            let field = fields[i];
            let ty = tys[i];
            // Each `impl SetGetType for X` block only generates match arms for cvars of type X
            // so that the getters and setters typecheck.
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

        // LATER Is there a sane way to automatically convert? (even fallibly)
        //       e.g. integers default to i32 even though cvar type is usize
        //       At the very least, it should suggest specifying the type.
        let trait_impl = quote! {
            impl SetGetType for #unique_ty {
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
        #[automatically_derived]
        impl #struct_name {
            /// Finds the cvar whose name matches `cvar_name` and returns its value.
            ///
            /// Returns `Err` if the cvar doesn't exist.
            pub fn get<T: SetGetType>(&self, cvar_name: &str) -> ::core::result::Result<T, String> {
                // We can't generate all the match arms here because we don't know what concrete type T is.
                // Instead, we statically dispatch it through the SetGetType trait and then only look up
                // fields of the correct type in each impl block.
                SetGetType::get(self, cvar_name)
            }

            /// Finds the cvar whose name matches `cvar_name` and returns its value as a `String`.
            ///
            /// Returns `Err` if the cvar doesn't exist.
            pub fn get_string(&self, cvar_name: &str) -> ::core::result::Result<String, String> {
                match cvar_name {
                    // This doesn't need to be dispatched via SetGetType, it uses Display instead.
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
            ///
            /// **Note**: Rust can't infer the type of `value` based on the type of the cvar
            /// because `cvar_name` is resolved to the right struct field only at runtime.
            /// This means integer literals default to `i32` and float literals to `f64`.
            /// This in turn means if the cvar's type is e.g. usize and you try
            /// `cvars.set("sometihng_with_type_usize", 123);`, it will fail
            /// because at compile time, `123` is inferred to be `i32`.
            /// Use `123_usize` to specify the correct type.
            ///
            /// This limitation doesn't apply to `set_str` since it determines which type to parse to
            /// *after* looking up the right field at runtime.
            pub fn set<T: SetGetType>(&mut self, cvar_name: &str, value: T) -> ::core::result::Result<(), String> {
                SetGetType::set(self, cvar_name, value)
            }

            /// Finds the cvar whose name matches `cvar_name`, tries to parse `str_value` to its type and sets it to the parsed value.
            ///
            /// Returns `Err` if the cvar doesn't exist or if `str_value` fails to parse to its type.
            pub fn set_str(&mut self, cvar_name: &str, str_value: &str) -> ::core::result::Result<(), String> {
                match cvar_name {
                    // This doesn't need to be dispatched via SetGetType, it uses FromStr instead.
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

        impl ::cvars::SetGet for #struct_name {
            fn get_string(&self, cvar_name: &str) -> ::core::result::Result<String, String> {
                self.get_string(cvar_name)
            }

            fn set_str(&mut self, cvar_name: &str, cvar_value: &str) -> ::core::result::Result<(), String> {
                self.set_str(cvar_name, cvar_value)
            }
        }

        /// This trait is needed to dispatch cvar get/set based on its type.
        /// You're not meant to impl it yourself, it's done automatically
        /// for all types used as cvars.
        pub trait SetGetType {
            fn get(cvars: &Cvars, cvar_name: &str) -> ::core::result::Result<Self, String>
                where Self: Sized;
            fn set(cvars: &mut Cvars, cvar_name: &str, value: Self) -> ::core::result::Result<(), String>;
        }

        #( #trait_impls )*
    };
    TokenStream::from(expanded)
}

/// Check whether the field has the `#[cvars(skip)]` attribute
fn skip_field(field: &Field) -> bool {
    for attr in &field.attrs {
        // Only attempt to parse the attribute if it's ours.
        // Not all attributes are parseable by `parse_meta`.
        if !attr.path.is_ident("cvars") {
            continue;
        }

        let meta = attr
            .parse_meta()
            .expect("expected #[cvars(skip)], failed to parse");
        if let Meta::List(MetaList { nested, .. }) = meta {
            if nested.len() != 1 {
                panic!("expected #[cvars(skip)]");
            }
            let nested_meta = nested.first().expect("len != 1");
            if let NestedMeta::Meta(Meta::Path(path)) = nested_meta {
                if path.is_ident("skip") {
                    return true;
                } else {
                    panic!("expected #[cvars(skip)]");
                }
            } else {
                panic!("expected #[cvars(skip)]");
            }
        } else {
            panic!("expected #[cvars(skip)]");
        }
    }

    false
}

/// Dummy version of SetGet for debugging how much cvars add to _incremental_ compile times of your project.
///
/// Generates the 4 setters and getters like SetGet but they contain only `unimplemented!()`,
/// therefore the code to dispatch from string to struct field is not generated.
/// This exists only to test how using SetGet affects the total compile time of downstream crates.
/// Simply replace SetGet with SetGetDummy and compare how long `cargo build` takes.
/// The resulting code should compile but will crash if the generated methods are used.
///
/// TODO Generate trait impls, also add those to dummy test
#[doc(hidden)]
#[proc_macro_derive(SetGetDummy, attributes(cvars))]
pub fn derive_dummy(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_name = input.ident;

    let expanded = quote! {
        impl #struct_name {
            pub fn get<T>(&self, cvar_name: &str) -> ::core::result::Result<T, String> {
                unimplemented!("SetGetDummy is only for compile time testing.");
            }
            pub fn get_string(&self, cvar_name: &str) -> ::core::result::Result<String, String> {
                unimplemented!("SetGetDummy is only for compile time testing.");
            }
            pub fn set<T>(&mut self, cvar_name: &str, value: T) -> ::core::result::Result<(), String> {
                unimplemented!("SetGetDummy is only for compile time testing.");
            }
            pub fn set_str(&mut self, cvar_name: &str, str_value: &str) -> ::core::result::Result<(), String> {
                unimplemented!("SetGetDummy is only for compile time testing.");
            }
        }
    };
    TokenStream::from(expanded)
}

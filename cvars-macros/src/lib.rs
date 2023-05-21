#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::collections::HashSet;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    Attribute, Data, DeriveInput, Expr, Fields, Ident, Meta, MetaList, Token, Type,
};

// LATER Optional feature to generate cvars from build.rs to avoid running the macro every build?
//  Format with prettyplease - https://docs.rs/quote/latest/quote/#non-macro-code-generators ?
// LATER How to profile? https://users.rust-lang.org/t/profiling-a-proc-macro/64274

struct CvarDef {
    attrs: Vec<Attribute>,
    skip: bool,
    name: Ident,
    ty: Type,
    value: Expr,
}

impl Parse for CvarDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs_raw = input.call(Attribute::parse_outer)?;
        let mut attrs = Vec::new();
        let mut skip = false;
        for attr in attrs_raw {
            if is_skip(&attr) {
                skip = true;
            } else {
                attrs.push(attr);
            }
        }
        let name = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty = input.parse()?;
        let _: Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(CvarDef {
            attrs,
            skip,
            name,
            ty,
            value,
        })
    }
}

/// **Experimental**. Generate the whole `Cvars` struct.
///
/// This allows both the field types and their initial values to be defined on one line.
/// The downside is that users currently can't specify attributes on the struct.
///
/// The generated code contains the struct definition and an `impl Default for Cvars` block
/// which sets the initial values.
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
    // LATER proper error reporting (no unwraps, expect only for infallible)

    let parser = Punctuated::<CvarDef, Token![,]>::parse_terminated;
    let punctuated = parser.parse(input).unwrap();

    let mut attrss = Vec::new();
    let mut skips = Vec::new();
    let mut names = Vec::new();
    let mut tys = Vec::new();
    let mut values = Vec::new();
    for cvar_def in punctuated {
        attrss.push(cvar_def.attrs);
        skips.push(cvar_def.skip);
        names.push(cvar_def.name);
        tys.push(cvar_def.ty);
        values.push(cvar_def.value);
    }

    let struct_name = Ident::new("Cvars", Span::call_site());
    let generated = generate(struct_name, &skips, &names, &tys);

    let expanded = quote! {
        pub struct Cvars {
            #(
                #( #attrss )*
                pub #names: #tys,
            )*
        }

        #[automatically_derived]
        impl ::std::default::Default for Cvars {
            fn default() -> Self {
                Self {
                    #(
                        #names: #values,
                    )*
                }
            }
        }

        #generated
    };

    expanded.into()
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
        Data::Struct(struct_data) => match struct_data.fields {
            Fields::Named(named_fields) => named_fields,
            Fields::Unnamed(_) => panic!("tuple structs are not supported, use named fields"),
            Fields::Unit => panic!("unit structs are not supported, use curly braces"),
        },
        Data::Enum(_) => panic!("enums are not supported, use a struct"),
        Data::Union(_) => panic!("unions are not supported, use a struct"),
    };

    // Get the list of all cvars and their types
    let mut skips = Vec::new();
    let mut names = Vec::new();
    let mut tys = Vec::new();
    for field in named_fields.named {
        skips.push(contains_skip(&field.attrs));
        names.push(field.ident.expect("ident was None"));
        tys.push(field.ty);
    }

    let expanded = generate(struct_name, &skips, &names, &tys);
    expanded.into()
}

fn generate(
    struct_name: Ident,
    skips: &[bool],
    names_all: &[Ident],
    tys_all: &[Type],
) -> proc_macro2::TokenStream {
    let mut names = Vec::new();
    let mut tys = Vec::new();
    for i in 0..skips.len() {
        if skips[i] {
            continue;
        }

        names.push(&names_all[i]);
        tys.push(&tys_all[i]);
    }

    let set_get_impl = impl_set_get(&struct_name);

    // Get the set of types used as cvars.
    // We need to impl SetGetType for them and it needs to be done
    // once per type, not once per cvar.
    // Note: I benchmarked this against FnvHashSet and it doesn't make a difference.
    let unique_tys: HashSet<_> = tys.iter().collect();
    let mut trait_impls = Vec::new();
    for unique_ty in unique_tys {
        let mut getter_arms = Vec::new();
        let mut setter_arms = Vec::new();

        for i in 0..names.len() {
            let field = names[i];
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
            #[automatically_derived]
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

    quote! {
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
                    #( stringify!(#names) => ::core::result::Result::Ok(self.#names.to_string()), )*
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
                    #( stringify!(#names) => match str_value.parse() {
                        ::core::result::Result::Ok(val) => ::core::result::Result::Ok(self.#names = val),
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

        #set_get_impl

        /// This trait is needed to dispatch cvar get/set based on its type.
        /// You're not meant to impl it yourself, it's done automatically
        /// for all types used as cvars.
        pub trait SetGetType {
            fn get(cvars: &Cvars, cvar_name: &str) -> ::core::result::Result<Self, String>
                where Self: Sized;
            fn set(cvars: &mut Cvars, cvar_name: &str, value: Self) -> ::core::result::Result<(), String>;
        }

        #( #trait_impls )*
    }
}

/// Check whether the field has the `#[cvars(skip)]` attribute
fn contains_skip(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| is_skip(attr))
}

fn is_skip(attr: &Attribute) -> bool {
    if let Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
        if !path.is_ident("cvars") {
            return false;
        }

        // Is it #[cvars(skip)]?
        let mut tokens = tokens.clone().into_iter();
        let nested = tokens.next().expect("expected #[cvars(skip)]");
        assert!(tokens.next().is_none(), "expected #[cvars(skip)]");
        if let TokenTree::Ident(ident) = nested {
            if ident == "skip" {
                return true;
            } else {
                panic!("expected #[cvars(skip)]");
            }
        } else {
            panic!("expected #[cvars(skip)]");
        }
    } else {
        false
    }
}

/// Dummy version of SetGet for debugging how much cvars add to _incremental_ compile times of your project.
///
/// Generates the 4 setters and getters like SetGet but they contain only `unimplemented!()`,
/// therefore the code to dispatch from string to struct field is not generated.
/// This exists only to test how using SetGet affects the total compile time of downstream crates.
/// Simply replace SetGet with SetGetDummy and compare how long `cargo build` takes.
/// The resulting code should compile but will crash if the generated methods are used.
#[doc(hidden)]
#[proc_macro_derive(SetGetDummy, attributes(cvars))]
pub fn derive_dummy(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_name = input.ident;
    let set_get_impl = impl_set_get(&struct_name);

    let expanded = quote! {
        #[automatically_derived]
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

        #set_get_impl
    };
    TokenStream::from(expanded)
}

fn impl_set_get(struct_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #[automatically_derived]
        impl ::cvars::SetGet for #struct_name {
            fn get_string(&self, cvar_name: &str) -> ::core::result::Result<String, String> {
                self.get_string(cvar_name)
            }

            fn set_str(&mut self, cvar_name: &str, cvar_value: &str) -> ::core::result::Result<(), String> {
                self.set_str(cvar_name, cvar_value)
            }
        }
    }
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Attributes: #[default], #[halt]
#[proc_macro_derive(State, attributes(default, halt))]
pub fn derive_state(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut default_variant = None;

    let is_halt_impl = match ast.data {
        Data::Enum(ref data) => {
            let mut arms = Vec::with_capacity(data.variants.len());
            let mut has_halt = false;

            // process each variant
            for variant in &data.variants {
                let variant_name = &variant.ident;

                let is_halt = variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("halt"));

                let is_default = variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("default"));

                if is_halt && is_default {
                    panic!("A variant cannot be marked as both #[halt] and #[default]");
                }

                if is_halt {
                    has_halt = true;
                    arms.push(quote! { Self::#variant_name => true });
                } else {
                    arms.push(quote! { Self::#variant_name => false });
                }

                if is_default {
                    if default_variant.is_some() {
                        panic!("Only one variant can be marked as default");
                    }
                    default_variant = Some(variant_name);
                }
            }

            if !has_halt {
                panic!("At least one variant must be marked with #[halt]");
            }

            if default_variant.is_none() {
                panic!("One variant must be marked with #[default]");
            }

            quote! {
                match self {
                    #(#arms,)*
                }
            }
        }

        _ => panic!("State can only be derived for enums"),
    };

    let default_impl = match default_variant {
        Some(variant) => quote! {
            impl std::default::Default for #name {
                fn default() -> Self {
                    Self::#variant
                }
            }
        },

        None => quote! {},
    };

    let expanded = quote! {
        impl bb_machine::State for #name {
            fn is_halt(&self) -> bool {
                #is_halt_impl
            }
        }

        #default_impl
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Color, attributes(default))]
pub fn derive_color(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let mut default_variant = None;

    match ast.data {
        Data::Enum(ref data_enum) => {
            // Process each variant
            for variant in &data_enum.variants {
                let variant_name = &variant.ident;

                let is_default = variant
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("default"));

                if is_default {
                    if default_variant.is_some() {
                        panic!("Only one variant can be marked as default");
                    }
                    default_variant = Some(variant_name);
                }
            }

            if default_variant.is_none() {
                panic!("One variant must be marked with #[default]");
            }
        }
        _ => panic!("Color can only be derived for enums"),
    };

    let default_impl = match default_variant {
        Some(variant) => quote! {
            impl std::default::Default for #name {
                fn default() -> Self {
                    Self::#variant
                }
            }
        },
        None => quote! {},
    };

    let expanded = quote! {
        impl bb_machine::Color for #name {}

        #default_impl
    };

    TokenStream::from(expanded)
}

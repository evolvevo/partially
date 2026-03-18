use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use self::derive_receiver::DeriveReceiver;

mod derive_receiver;
mod field_receiver;
mod impl_partial;
mod meta_attribute;
mod token_vec;

pub fn expand_derive_partial(item: &mut DeriveInput) -> TokenStream {
    let maybe_receiver = DeriveReceiver::from_derive_input(item);

    match maybe_receiver {
        Ok(receiver) => quote!(#receiver),
        Err(e) => e.write_errors(),
    }
}

#[cfg(test)]
mod test {
    use darling::FromDeriveInput;
    use proc_macro2::TokenStream;
    use syn::{parse_quote, DeriveInput};

    use super::{derive_receiver::DeriveReceiver, expand_derive_partial};

    #[test]
    fn basic_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(derive(Default, Debug))]
            #[partially(attribute(serde(default)))]
            #[some_attr]
            struct Data {
                /// A documented field.
                #[some_attr]
                str_field: String,
                #[partially(omit)]
                skipped_field: String,
                #[partially(as_type = "Option<f32>")]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(Default, Debug)]
            #[some_attr]
            #[serde(default)]
            struct PartialData {
                /// A documented field.
                #[some_attr]
                str_field: Option<String>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>,
            }

            impl partially::Partial for Data {
                type Item = PartialData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }

            impl partially::Partial for PartialData {
                type Item = PartialData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn basic_e2e_named() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(rename = "OptData")]
            #[partially(derive(Default, Debug))]
            #[some_attr]
            struct Data {
                /// A documented field.
                #[some_attr]
                str_field: String,
                #[partially(omit)]
                skipped_field: String,
                #[partially(as_type = "Option<f32>")]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(Default, Debug)]
            #[some_attr]
            struct OptData {
                /// A documented field.
                #[some_attr]
                str_field: Option<String>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>,
            }

            impl partially::Partial for Data {
                type Item = OptData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }

            impl partially::Partial for OptData {
                type Item = OptData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn generic_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(rename = "PartialData")]
            #[partially(derive(Default, Debug))]
            #[some_attr]
            struct Data<T> {
                /// A documented field.
                #[some_attr]
                type_field: T,
                #[partially(omit)]
                skipped_field: String,
                #[partially(as_type = "Option<f32>")]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(Default, Debug)]
            #[some_attr]
            struct PartialData<T> {
                /// A documented field.
                #[some_attr]
                type_field: Option<T>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>,
            }

            impl<T> partially::Partial for Data<T> {
                type Item = PartialData<T>;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.type_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(type_field) = partial.type_field {
                        self.type_field = type_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }

            impl<T> partially::Partial for PartialData<T> {
                type Item = PartialData<T>;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.type_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(type_field) = partial.type_field {
                        self.type_field = type_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn generic_constrained_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(rename = "PartialData")]
            #[partially(derive(Default, Debug))]
            #[partially(crate = "custom_partially")]
            #[some_attr]
            struct Data<T> where T: Sized {
                /// A documented field.
                #[some_attr]
                type_field: T,
                #[partially(omit)]
                skipped_field: String,
                #[partially(as_type = "Option<f32>")]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(Default, Debug)]
            #[some_attr]
            struct PartialData<T> where T: Sized {
                /// A documented field.
                #[some_attr]
                type_field: Option<T>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>,
            }

            impl<T> custom_partially::Partial for Data<T> where T : Sized {
                type Item = PartialData<T>;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.type_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(type_field) = partial.type_field {
                        self.type_field = type_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }

            impl<T> custom_partially::Partial for PartialData<T> where T : Sized {
                type Item = PartialData<T>;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.type_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(type_field) = partial.type_field {
                        self.type_field = type_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn extensive_attr_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial, Default, Debug)]
            #[partially(derive(PartialEq))]
            #[partially(attribute(serde(default)))]
            #[partially(attribute(serde(rename = "PascalCase")))]
            #[partially(skip_attributes)]
            #[some_attr]
            struct Data {
                /// A documented field.
                #[some_attr]
                str_field: String,
                #[partially(omit)]
                skipped_field: String,
                #[partially(as_type = "Option<f32>")]
                #[some_attr]
                number_field: i32,
                #[partially(transparent)]
                transparent_field: Option<String>,
                #[partially(rename = "new_field")]
                old_field: String
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(PartialEq)]
            #[serde(default)]
            #[serde(rename = "PascalCase")]
            struct PartialData {
                /// A documented field.
                #[some_attr]
                str_field: Option<String>,
                #[some_attr]
                number_field: Option<f32>,
                transparent_field: Option<String>,
                new_field: Option<String>,
            }

            impl partially::Partial for Data {
                type Item = PartialData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }

            impl partially::Partial for PartialData {
                type Item = PartialData;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false ||
                        partial.str_field.is_some() ||
                        partial.number_field.is_some() ||
                        partial.transparent_field.is_some() ||
                        partial.new_field.is_some();

                    if let Some(str_field) = partial.str_field {
                        self.str_field = str_field.into();
                    }

                    if let Some(number_field) = partial.number_field {
                        self.number_field = number_field.into();
                    }

                    if let Some(transparent_field) = partial.transparent_field {
                        self.transparent_field = transparent_field.into();
                    }

                    if let Some(new_field) = partial.new_field {
                        self.old_field = new_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn apply_with_e2e() {
        let mut input: DeriveInput = parse_quote! {
            #[derive(partially::Partial)]
            #[partially(derive(Default))]
            struct Config {
                /// Field with custom applicator closure
                #[partially(as_type = "CustomPatch<String>", apply_with = "|p, t| p.apply(t)")]
                patched_field: String,
                /// Field with custom applicator function
                #[partially(apply_with = "custom_apply")]
                func_field: i32,
                /// Normal field
                normal_field: bool,
            }
        };

        let expanded = expand_derive_partial(&mut input);

        let expected: TokenStream = parse_quote! {
            #[derive(Default)]
            struct PartialConfig {
                /// Field with custom applicator closure
                patched_field: CustomPatch<String>,
                /// Field with custom applicator function
                func_field: Option<i32>,
                /// Normal field
                normal_field: Option<bool>,
            }

            impl partially::Partial for Config {
                type Item = PartialConfig;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    // Only normal_field is in is_some check (apply_with fields excluded)
                    let mut will_apply_some = false || partial.normal_field.is_some();

                    // patched_field uses custom closure
                    will_apply_some = (|p, t| p.apply(t))(
                        partial.patched_field,
                        &mut self.patched_field
                    ) || will_apply_some;

                    // func_field uses custom function
                    will_apply_some = (custom_apply)(
                        partial.func_field,
                        &mut self.func_field
                    ) || will_apply_some;

                    // normal_field uses default
                    if let Some(normal_field) = partial.normal_field {
                        self.normal_field = normal_field.into();
                    }

                    will_apply_some
                }
            }

            impl partially::Partial for PartialConfig {
                type Item = PartialConfig;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = false || partial.normal_field.is_some();

                    // Partial→Partial: apply_with fields use Partial::apply_some
                    will_apply_some = partially::Partial::apply_some(
                        &mut self.patched_field,
                        partial.patched_field
                    ) || will_apply_some;

                    will_apply_some = partially::Partial::apply_some(
                        &mut self.func_field,
                        partial.func_field
                    ) || will_apply_some;

                    if let Some(normal_field) = partial.normal_field {
                        self.normal_field = normal_field.into();
                    }

                    will_apply_some
                }
            }
        };

        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn apply_with_nested_mutex() {
        let input: DeriveInput = parse_quote! {
            #[derive(partially::Partial)]
            struct Config {
                #[partially(nested, apply_with = "|p, t| true")]
                field: String,
            }
        };

        let result = DeriveReceiver::from_derive_input(&input);
        assert!(
            result.is_err(),
            "apply_with and nested should be mutually exclusive"
        );
    }
}

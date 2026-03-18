use quote::{quote, ToTokens};
use std::iter;
use syn::{Generics, Ident, Path};

use super::{
    field_receiver::FieldReceiver,
    token_vec::{Separator, TokenVec},
};

pub struct ImplPartial<'a> {
    pub krate: &'a Path,
    pub generics: &'a Generics,
    pub from_ident: &'a Ident,
    pub to_ident: &'a Ident,

    /// Note: assumed to already be filtered (such that `omit`-ted entries are removed)
    pub fields: &'a Vec<&'a FieldReceiver>,
}

impl<'a> ToTokens for ImplPartial<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            krate,
            from_ident,
            to_ident,
            generics,
            fields,
        } = self;

        let (imp, ty, wher) = generics.split_for_impl();

        // For Partial→Partial impl, apply_with fields use Partial::apply_some
        // (the closure is designed for Original→Partial, not Partial→Partial)
        let is_partial_to_partial = from_ident == to_ident;

        // Exclude nested and apply_with fields from is_some check
        // (they don't necessarily have Option type)
        let field_is_somes = iter::once(quote!(false))
            .chain(
                fields
                    .iter()
                    .filter(|f| !f.nested.is_present() && f.apply_with.is_none())
                    .map(|f| {
                        // this is enforced with a better error by [`FieldReceiver::validate`].
                        let from_ident = f.ident.as_ref().unwrap();

                        let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                        quote!(partial.#to_ident.is_some())
                    }),
            )
            .collect();
        let field_is_somes = TokenVec::new_with_vec_and_sep(field_is_somes, Separator::Or);

        let field_applicators = fields
            .iter()
            .map(|f| {
                // this is enforced with a better error by [`FieldReceiver::validate`].
                let from_ident = f.ident.as_ref().unwrap();

                let to_ident = f.rename.as_ref().unwrap_or(from_ident);

                if f.nested.is_present() {
                    quote! {
                        will_apply_some = #krate::Partial::apply_some(
                            &mut self.#from_ident,
                            partial.#to_ident
                        ) || will_apply_some;
                    }
                } else if let Some(apply_with_expr) = &f.apply_with {
                    if is_partial_to_partial {
                        // Partial→Partial: use Partial::apply_some
                        // (requires the partial field type to impl Partial)
                        quote! {
                            will_apply_some = #krate::Partial::apply_some(
                                &mut self.#from_ident,
                                partial.#to_ident
                            ) || will_apply_some;
                        }
                    } else {
                        // Original→Partial: use custom closure/function
                        quote! {
                            will_apply_some = (#apply_with_expr)(
                                partial.#to_ident,
                                &mut self.#from_ident
                            ) || will_apply_some;
                        }
                    }
                } else {
                    quote! {
                        if let Some(#to_ident) = partial.#to_ident {
                            self.#from_ident = #to_ident.into();
                        }
                    }
                }
            })
            .collect();
        let field_applicators =
            TokenVec::new_with_vec_and_sep(field_applicators, Separator::Newline);

        tokens.extend(quote! {
            impl #imp #krate::Partial for #from_ident #ty #wher {
                type Item = #to_ident #ty;

                fn apply_some(&mut self, partial: Self::Item) -> bool {
                    let mut will_apply_some = #field_is_somes;

                    #field_applicators

                    will_apply_some
                }
            }
        })
    }
}

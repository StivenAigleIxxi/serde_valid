use crate::abort::{
    abort_unexpected_list_argument, abort_unexpected_lit_argument,
    abort_unexpected_name_value_argument,
};
use crate::types::{NamedField, SingleIdentPath};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "custom";

pub fn extract_generic_custom_validator(
    field: &NamedField,
    attribute: &syn::Attribute,
    meta_list: &syn::MetaList,
) -> Validator {
    let mut custom_validation_fn = None;
    let mut custom_validation_args = None;
    let field_ident = field.ident();
    for meta_item in &meta_list.nested {
        match meta_item {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => {
                    update_custom_validator_from_meta_path(&mut custom_validation_fn, path)
                }
                syn::Meta::List(list) => update_custom_validator_from_meta_list(
                    &mut custom_validation_fn,
                    &mut custom_validation_args,
                    list,
                ),
                syn::Meta::NameValue(name_value) => abort_unexpected_name_value_argument(
                    VALIDATION_LABEL,
                    field_ident,
                    attribute.span(),
                    name_value,
                ),
            },
            syn::NestedMeta::Lit(lit) => {
                abort_unexpected_lit_argument(VALIDATION_LABEL, field_ident, attribute.span(), &lit)
            }
        }
    }

    let custom_validation_fn = custom_validation_fn.unwrap_or_else(|| {
        abort!(
            attribute.span(),
            &format!(
                "'{}' literal meta items size must be greater than 0.",
                VALIDATION_LABEL
            )
        )
    });

    let field_string = field_ident.to_string();
    return Validator::Normal(quote!(
        if let Err(error) = #custom_validation_fn(#field_ident, #custom_validation_args) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(error);
        };
    ));
}

fn update_custom_validator_from_meta_path(
    custom_validation_fn: &mut Option<TokenStream>,
    path: &syn::Path,
) {
    check_duplicated_custom_validation(custom_validation_fn, path);
    *custom_validation_fn = Some(quote!(#path));
}

fn update_custom_validator_from_meta_list(
    custom_validation_fn: &mut Option<TokenStream>,
    custom_validation_args: &mut Option<TokenStream>,
    meta_list: &syn::MetaList,
) {
    let path = &meta_list.path;
    let path_ident = SingleIdentPath::new(path).ident();
    check_duplicated_custom_validation(custom_validation_fn, path);
    check_duplicated_custom_validation(custom_validation_args, path);

    let nested: syn::punctuated::Punctuated<TokenStream, syn::Token![,]> = meta_list
        .nested
        .iter()
        .map(|nested_meta| match nested_meta {
            syn::NestedMeta::Lit(lit) => quote!(#lit),
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => quote!(&self.#path),
                syn::Meta::List(list) => abort_unexpected_list_argument(
                    VALIDATION_LABEL,
                    path_ident,
                    meta_list.span(),
                    &list,
                ),
                syn::Meta::NameValue(name_value) => abort_unexpected_name_value_argument(
                    VALIDATION_LABEL,
                    path_ident,
                    meta_list.span(),
                    &name_value,
                ),
            },
        })
        .collect();

    *custom_validation_fn = Some(quote!(#path));
    *custom_validation_args = Some(quote!(#nested));
}

fn check_duplicated_custom_validation(
    custom_validation_fn: &Option<TokenStream>,
    path: &syn::Path,
) {
    if custom_validation_fn.is_some() {
        abort!(
            path.span(),
            &format!("'{}' validator allow 1 custom function.", VALIDATION_LABEL)
        )
    }
}

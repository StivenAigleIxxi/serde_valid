use std::collections::HashMap;

use crate::types::{Field, NamedField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

pub type RenameMap = HashMap<String, TokenStream>;

pub fn collect_serde_rename_map(fields: &syn::FieldsNamed) -> RenameMap {
    let mut renames = RenameMap::new();
    for field in fields.named.iter() {
        let named_field = NamedField::new(field);
        for attribute in named_field.attrs() {
            if attribute.path == parse_quote!(serde) {
                if let Some(rename) = find_rename_from_serde_attributes(attribute) {
                    renames.insert(
                        field.ident.to_token_stream().to_string(),
                        quote!(#rename.to_string()),
                    );
                }
            }
        }
    }
    renames
}

fn find_rename_from_serde_attributes(attribute: &syn::Attribute) -> Option<TokenStream> {
    if let Ok(syn::Meta::List(serde_list)) = attribute.parse_meta() {
        for serde_nested_meta in serde_list.nested {
            if let syn::NestedMeta::Meta(serde_meta) = &serde_nested_meta {
                if let Some(rename) = find_rename_from_serde_rename_attributes(serde_meta) {
                    return Some(rename);
                }
            }
        }
    }
    None
}

fn find_rename_from_serde_rename_attributes(serde_meta: &syn::Meta) -> Option<TokenStream> {
    match serde_meta {
        syn::Meta::NameValue(rename_name_value) => {
            if let syn::Lit::Str(lit_str) = &rename_name_value.lit {
                Some(lit_str.to_token_stream())
            } else {
                None
            }
        }
        syn::Meta::List(rename_list) => {
            for rename_nested_meta in &rename_list.nested {
                if let syn::NestedMeta::Meta(rename_meta) = rename_nested_meta {
                    if *rename_meta.path() != parse_quote!(deserialize) {
                        continue;
                    }
                    if let syn::Meta::NameValue(deserialize_name_value) = rename_meta {
                        if let syn::Lit::Str(lit_str) = &deserialize_name_value.lit {
                            return Some(lit_str.to_token_stream());
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

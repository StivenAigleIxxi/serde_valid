use crate::field_validate::{MetaPathStructValidation, Validator};

#[inline]
pub fn extract_struct_validator_from_nested_meta_path(
    validation_type: MetaPathStructValidation,
    _validation: &syn::Path,
) -> Result<Validator, crate::Errors> {
    match validation_type {}
}

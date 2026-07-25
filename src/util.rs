pub mod token_signer;

/// Update SeaORM ActiveModel with struct-patch struct.
/// Prerequirement: DB Schema and struct-patch should
/// share same key-value pair type.
///
/// Usage example:
/// ```
/// update_am!(active_model, patch, keys_to_apply...)
/// ```
#[macro_export]
macro_rules! update_am {
    ($active_model:expr, $patch:expr, $($field:ident),* $(,)?) => {
        $(
            if let Some(v) = $patch.$field {
                $active_model.$field = sea_orm::ActiveValue::Set(v);
            }
        )*
    };
}

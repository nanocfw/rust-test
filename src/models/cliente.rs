use derive_macro::default_model;

#[default_model]
#[derive(Debug, Clone, Default)]
pub struct Cliente {
    pub name: String,
}

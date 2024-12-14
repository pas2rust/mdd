use syn::{Data, DeriveInput, Fields, FieldsNamed};

pub fn get_named_fields(input: &DeriveInput) -> Result<&FieldsNamed, &str> {
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => Ok(named_fields),
            _ => Err("Fields are not named"),
        },
        _ => Err("Data is not a struct"),
    }
}

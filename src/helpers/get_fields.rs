use syn::{Data, DeriveInput, Fields};

pub fn get_fields(input: &DeriveInput) -> Result<&Fields, &str> {
    match &input.data {
        Data::Struct(data_struct) => Ok(&data_struct.fields),
        _ => Err("from_json can only be derived for structs"),
    }
}

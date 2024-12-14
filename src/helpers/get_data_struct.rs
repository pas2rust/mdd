use syn::{Data, DataStruct, DeriveInput};

pub fn get_data_struct(input: &DeriveInput) -> Result<&DataStruct, &str> {
    match &input.data {
        Data::Struct(data_struct) => Ok(data_struct),
        _ => Err("Data is not a struct"),
    }
}

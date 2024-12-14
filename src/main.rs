#![cfg(all(feature = "builder", feature = "math", feature = "logging"))]

use mdd::{Builder, Logging, Math};

#[derive(Debug, Builder, Math, Clone, Default, Logging)]
#[transporter(async fn procedure() {
    println!("message: {}", message);
})]
pub struct CalcStruct {
    pub usize: usize,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub u128: u128,
    pub isize: isize,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
    pub f64: f64,
    pub f32: f32,
}

#[tokio::main]
async fn main() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(100)
        .u8::<u8>(100)
        .u16::<u16>(100)
        .u32::<u32>(100)
        .u64::<u64>(100)
        .u128::<u128>(100)
        .isize::<isize>(100)
        .i8::<i8>(100)
        .i16::<i16>(100)
        .i32(100)
        .i64(100)
        .i128(100)
        .f64(100.0)
        .f32(100.0)
        .build()
        .unwrap();

    calc_struct.inflate_usize(20);
    calc_struct.inflate_u8(20);
    calc_struct.inflate_u16(20);
    calc_struct.inflate_u32(20);
    calc_struct.inflate_u64(20);
    calc_struct.inflate_u128(20);
    calc_struct.inflate_isize(20);
    calc_struct.inflate_i8(20);
    calc_struct.inflate_i16(20);
    calc_struct.inflate_i32(20);
    calc_struct.inflate_i64(20);
    calc_struct.inflate_i128(20);
    calc_struct.inflate_f32(20.0);
    calc_struct.inflate_f64(20.0);
    calc_struct.print_info("ok").await;
}

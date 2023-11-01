use serde::{Deserialize, Serialize};

pub const fn default_bool<const V: bool>() -> bool {
    V
}
pub const fn default_u32<const V: u32>() -> u32 {
    V
}
pub const fn default_u64<const V: u64>() -> u64 {
    V
}
pub const fn default_i32<const V: i32>() -> i32 {
    V
}
pub const fn default_i64<const V: i64>() -> i64 {
    V
}



pub const SPV_MAGIC_NUMBER_LE: u32 = 0x07230203;
pub const SPV_MAGIC_NUMBER_BE: u32 = SPV_MAGIC_NUMBER_LE.swap_bytes();

enum B8 {
    I(i8),
    U(u8)
}
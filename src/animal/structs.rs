/**
* str -> primitive
* i8, i16, i32... -> signed integers
* u8, u16, u32...  -> unsigned integers
* bool -> booleans (on, off) or (yes, no) or (1, 0)
* f32, f64 -> floating point nums
* char -> single char
*/
pub struct Cat {
    pub(super) name: String,
    pub(super) age: i32
}

pub struct Dog {
    pub(super) name: String,
    pub(super) age: i32
}
/*
************ RUST TUTORIAL ************
TITLE: DATA TYPES

A scalar type represents a single value. For example, 10,3.14,'c'.
Rust has four primary scalar types:

Integer
Floating-point
Booleans
Characters
*/

#[derive(Debug)]

pub struct DataFields {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub first_letter_of_name: char,
    pub age: u32,
    pub national_code: String,
    pub weight: f32,
    pub height: u32,
    pub married: bool,
    pub is_alive: bool,
}

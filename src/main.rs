extern crate byteorder;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quick_error;
extern crate uuid;

mod nbt;
mod binary;
mod text;
mod proto;

fn main() {
    println!("Hello world");
}

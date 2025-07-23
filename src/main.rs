pub mod standard_names;

// use crate::standard_names::raw_aliases::RAW_ALIASES;
// use crate::standard_names::aliases::parse_aliases;
use crate::standard_names::load_standards;

fn main() {
    println!("Hello, world!");
    // println!("{:?}", parse_aliases());
    println!("{:?}", load_standards());
}

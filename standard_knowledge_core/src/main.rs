use standard_knowledge::standard_names;

fn main() {
    println!("Hello, world!");
    let standards = standard_names::cf_standards();
    println!("{:?}", standards);
}

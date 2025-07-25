use standard_knowledge::standards_library::StandardsLibrary;

fn main() {
    let mut standards = StandardsLibrary::default();
    standards.load_cf_standards();
    println!(
        "By name: {:?}",
        &standards.get("air_pressure_at_mean_sea_level")
    );
    println!(
        "By alias: {:?}",
        &standards.get("air_pressure_at_sea_level")
    )
}

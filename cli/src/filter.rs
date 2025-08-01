use clap::{Parser, ValueEnum};
use standard_knowledge::{Standard, StandardsLibrary};
use std::process;

#[derive(Parser)]
pub struct FilterArgs {
    /// Filter by common variable names
    #[arg(short, long)]
    var: Option<String>,
    /// Filter by IOOS category
    #[arg(short, long)]
    ioos_category: Option<String>,
    /// Filter by unit
    #[arg(short, long)]
    unit: Option<String>,
    /// Search by string across multiple fields
    #[arg(short, long)]
    search: Option<String>,

    /// Format to display in
    #[arg(short, long, value_enum, default_value_t = ListFormat::Short)]
    format: ListFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ListFormat {
    /// Shorthand display,
    Short,
    /// Xarray attributes
    Xarray,
}

impl ListFormat {
    fn format_standards(&self, standards: Vec<&Standard>) -> String {
        match self {
            Self::Short => standards
                .iter()
                .map(|standard| format!("- {}", standard.display_short()))
                .collect::<Vec<String>>()
                .join("\n"),

            Self::Xarray => standards
                .iter()
                .map(|standard| standard.display_xarray_attrs())
                .collect::<Vec<String>>()
                .join(",\n"),
        }
    }
}

pub fn execute(filter_args: &FilterArgs, library: &StandardsLibrary) {
    let mut standards_filter = library.filter();

    if let Some(var) = &filter_args.var {
        standards_filter = standards_filter.by_variable_name(var);
    }
    if let Some(ioos_category) = &filter_args.ioos_category {
        standards_filter = standards_filter.by_ioos_category(ioos_category);
    }
    if let Some(unit) = &filter_args.unit {
        standards_filter = standards_filter.by_unit(unit);
    }
    if let Some(search_str) = &filter_args.search {
        standards_filter = standards_filter.search(search_str);
    }

    let filtered_standards: Vec<_> = standards_filter.standards.into_iter().collect();

    if filtered_standards.is_empty() {
        eprintln!("No standards found matching the criteria.");
        process::exit(2);
    } else {
        let format = filter_args.format;
        println!("{}", format.format_standards(filtered_standards));
    }
}

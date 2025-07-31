use std::process;

use clap::{Parser, Subcommand, ValueEnum};
use standard_knowledge::{Standard, StandardsLibrary};

pub mod qc;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get standard by name or alias
    Get {
        /// Standard name or alias
        name: String,

        /// Format to display in
        #[arg(short, long, value_enum, default_value_t = GetFormat::Full)]
        format: GetFormat,
    },

    /// Find standards by common variable names
    ByVariable {
        // Variable name
        name: String,

        /// Format to display in
        #[arg(short, long, value_enum, default_value_t = ListFormat::Short)]
        format: ListFormat,
    },

    /// Search through all standard fields
    Search {
        /// String to search by
        search_str: String,

        /// Format to display in
        #[arg(short, long, value_enum, default_value_t = ListFormat::Short)]
        format: ListFormat,
    },

    /// QARTOD test suites
    Qc(qc::QcArgs),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GetFormat {
    /// Shorthand display
    Short,
    /// All info for standard
    Full,
    /// Xarray attributes
    Xarray,
    // /// ERDDAP datasets.xml <addAttributes>
    // Erddap,
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

fn main() {
    let cli = Cli::parse();

    let mut library = StandardsLibrary::default();
    library.load_cf_standards();
    library.load_knowledge();
    library.load_test_suites();

    match &cli.command {
        Commands::Get { name, format } => {
            if let Ok(standard) = library.get(name) {
                match format {
                    GetFormat::Short => {
                        println!("{standard}")
                    }
                    GetFormat::Full => {
                        println!("{}", standard.display_all())
                    }
                    GetFormat::Xarray => {
                        println!("{}", standard.display_xarray_attrs());
                    }
                }
            } else {
                eprintln!("Didn't find a standard matching: {name}");
                process::exit(2)
            }
        }
        Commands::ByVariable { name, format } => {
            let filter = library.filter().by_variable_name(name);
            if filter.standards.is_empty() {
                eprintln!("No standards with a variable for: {name}");
                process::exit(2)
            } else {
                println!("{}", format.format_standards(filter.standards))
            }
        }
        Commands::Search { search_str, format } => {
            let filtered = library.filter().search(search_str);
            if filtered.standards.is_empty() {
                eprintln!("No standards match: {search_str}");
                process::exit(2)
            } else {
                println!("{}", format.format_standards(filtered.standards))
            }
        }
        Commands::Qc(qc_args) => {
            qc::execute(qc_args, &library);
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

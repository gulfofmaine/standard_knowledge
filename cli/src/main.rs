use std::process;

use clap::{Parser, Subcommand, ValueEnum};
use standard_knowledge::StandardsLibrary;

pub mod filter;
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

    /// Filter standards
    Filter(filter::FilterArgs),

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
                        println!("{}", standard.display_short())
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
        Commands::Filter(filter_args) => {
            filter::execute(filter_args, &library);
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

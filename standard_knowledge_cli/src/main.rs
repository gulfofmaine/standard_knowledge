use clap::{Parser, Subcommand, ValueEnum};
use standard_knowledge::StandardsLibrary;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get standard by name or alias
    Get {
        name: String,
        #[arg(short, long, value_enum, default_value_t = GetFormat::Full)]
        format: GetFormat,
    },
    ByVariable {
        name: String,
    },
    Search {
        search_str: String,
    },
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
                        println!("{{");
                        for (key, value) in standard.xarray_attrs() {
                            println!("  \"{key}\": \"{value}\",")
                        }
                        println!("}}")
                    }
                }
            } else {
                eprintln!("Didn't find a standard matching: {name}");
            }
        }
        Commands::ByVariable { name } => {
            let standards = library.by_variable_name(name);
            if standards.is_empty() {
                eprintln!("No standards with a variable for: {name}");
            } else {
                println!("Found standards:");
                for standard in standards {
                    println!("- {standard}");
                }
            }
        }
        Commands::Search { search_str } => {
            let standards = library.search(search_str);
            if standards.is_empty() {
                eprintln!("No standards with a variable for: {search_str}");
            } else {
                println!("Found standards:");
                for standard in standards {
                    println!("- {standard}");
                }
            }
        }
    }
}

# Standard Knowledge

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

Standard Knowledge is a multi-language library for programmatically augmenting CF Standards with operational knowledge. It provides cross-language access (Rust, Python, JavaScript) to shared learnings from the CF Standards community.

## Working Effectively

### Prerequisites
- Install uv Python package manager: `pip3 install uv`
- Install pre-commit: `pip3 install pre-commit`

### Bootstrap, build, and test the repository
- `cargo build --verbose` -- builds all Rust components. Takes ~55 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `cargo test --verbose` -- runs all Rust tests. Takes ~25 seconds. NEVER CANCEL. Set timeout to 60+ seconds.
- `cd py && uv run pytest` -- runs Python tests. Takes ~25 seconds. NEVER CANCEL. Set timeout to 60+ seconds.

### Install CLI tool
- `cargo install --path cli` -- installs the standard_knowledge CLI tool. Takes ~32 seconds. NEVER CANCEL. Set timeout to 90+ seconds.

### Run applications
Python library (interactive testing):
- `cd py && uv run python`
```python
import standard_knowledge
library = standard_knowledge.StandardsLibrary()
library.load_cf_standards()
library.load_knowledge()
standard = library.get("air_pressure_at_mean_sea_level")
print(standard.attrs())  # Note: use standard.unit, not standard.units
```

CLI usage:
- `standard_knowledge --help` -- show CLI help
- `standard_knowledge get air_pressure_at_mean_sea_level` -- get standard details
- `standard_knowledge get -f xarray air_pressure_at_mean_sea_level` -- get xarray format
- `standard_knowledge filter --search temp` -- search for standards
- `standard_knowledge filter --ioos-category Meteorology` -- filter by category

### Linting and formatting
- `cargo fmt --check` -- check Rust formatting (~0.6 seconds)
- `cargo fmt --all` -- apply Rust formatting
- `cargo clippy --all-targets --all-features` -- run Rust linting. Takes ~12 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
- `pre-commit install` -- install pre-commit hooks
- `pre-commit run --all-files` -- run all pre-commit checks

## Validation

Always manually validate changes by:
1. **Build and test**: Run both Rust and Python test suites after making changes
2. **CLI functionality**: Test CLI commands with real examples from the README
3. **Python library**: Test Python library functionality interactively
4. **Standards data**: Verify standards can be loaded and queried correctly

### Test CLI changes
- Update CLI tests: `TRYCMD=overwrite cargo test --package standard_knowledge_cli --test cmd` -- Takes ~11 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
- For new CLI features, copy existing test files in `cli/tests/cmd/` and run the TRYCMD command

### Update standards
- Run `uv run --script utils/update_standards.py` to refresh CF Standards from official sources. Takes ~10 seconds.

## Repository Structure

- **core/**: Rust library containing standards data and core functionality
- **cli/**: Command-line interface (standard_knowledge binary)
- **py/**: Python bindings using PyO3/Maturin
- **utils/**: Python utility scripts for maintaining standards data

### Key files and directories
- `core/standards/`: YAML files containing community knowledge for each standard
- `core/src/lib.rs`: Main Rust library entry point
- `cli/src/main.rs`: CLI application entry point
- `py/src/lib.rs`: Python bindings entry point
- `.pre-commit-config.yaml`: Pre-commit hooks configuration

### Standards Knowledge Structure
Each standard has a YAML file in `core/standards/<standard_name>.yaml` with:
- `ioos_category`: IOOS measurement category
- `long_name`: Human readable name
- `common_variable_names`: Alternative column/variable names
- `related_standards`: Similar standards worth investigating
- `sibling_standards`: Standards typically used together
- `extra_attrs`: Additional xarray/ERDDAP attributes
- `other_units`: Alternative units commonly used
- `comments`: Implementation notes and usage guidance

## Common Tasks

### Build times (with 50% buffer for timeout recommendations)
- Rust build: ~55 seconds → Use 120+ second timeout
- Rust tests: ~25 seconds → Use 60+ second timeout  
- CLI install: ~32 seconds → Use 90+ second timeout
- Python tests: ~25 seconds → Use 60+ second timeout
- Clippy linting: ~12 seconds → Use 30+ second timeout
- CLI test updates: ~11 seconds → Use 30+ second timeout

### Project Workspace Structure
This is a Cargo workspace with three members:
- `core` (standard_knowledge): Core Rust library
- `cli` (standard_knowledge_cli): CLI application  
- `py` (standard_knowledge_py): Python bindings

### Common CI/CD commands that must pass
- All pre-commit hooks (rustfmt, clippy, ruff, codespell, actionlint)
- `cargo build --verbose` 
- `cargo test --verbose`
- `cd py && uv run pytest`

### Example repository root listing
```
.
├── .github/
│   └── workflows/          # CI/CD workflows (rust.yml, python.yml, pre-commit.yml)
├── cli/                    # CLI application
├── core/                   # Core Rust library  
│   └── standards/          # Standards knowledge YAML files
├── py/                     # Python bindings
├── utils/                  # Utility scripts
├── Cargo.toml              # Workspace configuration
├── README.md
└── .pre-commit-config.yaml
```

### Standards Library Usage Patterns
**Rust:**
```rust
use standard_knowledge::StandardsLibrary;
let mut library = StandardsLibrary::new();
library.load_cf_standards();
library.load_knowledge();
```

**Python:**
```python
import standard_knowledge
library = standard_knowledge.StandardsLibrary()
library.load_cf_standards()
library.load_knowledge()
```

**CLI:**
```bash
standard_knowledge get <standard_name>
standard_knowledge filter --var <variable_name>
standard_knowledge qc config <standard> <region> <params>
```

### Adding new standards knowledge
1. Create or edit YAML file in `core/standards/<standard_name>.yaml`
2. Run tests to validate: `cargo test`
3. Update CLI tests if needed: `TRYCMD=overwrite cargo test --package standard_knowledge_cli --test cmd`
4. Always run linting before committing: `cargo fmt --all && cargo clippy`

### Important Notes
- Python bindings automatically rebuild when Rust code changes
- CLI tests use trycmd for snapshot testing - update with `TRYCMD=overwrite`
- Standards data is embedded into binaries at compile time via `build.rs`
- Network access required only for `utils/update_standards.py` to fetch CF Standards
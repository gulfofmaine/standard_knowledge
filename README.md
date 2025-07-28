# standard_knowledge
Programmatically augmenting CF Standards with operational knowledge.

```py
# uv run python
# in standard_knowledge_py

import standard_knowledge

library = standard_knowledge.StandardsLibrary()

# Load all CF standards
library.load_cf_standards()

# Apply community knowledge to the standards
library.load_knowledge()

# Get a standard by name or alias
standard = library.get("air_pressure_at_mean_sea_level")

# Xarray compatible attributes for a standard
attrs = standard.attrs()

# find standards by variable names
standards = library.by_variable_name("pressure")

# Search for standards across multiple fields (name, aliases, common variable names, related standards)
under_pressure = library.search("pressure")
```

A CLI can also be installed for interacting with the standards.

`cargo install --path standard_knowledge_cli`

```sh
❯ standard_knowledge --help
Usage: standard_knowledge <COMMAND>

Commands:
  get          Get standard by name or alias
  by-variable
  search
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

❯ standard_knowledge get -f xarray air_pressure_at_mean_sea_level
{
  "ioos_category": "Meteorology",
  "long_name": "Atmospheric Pressure at Sea Level",
  "standard_name": "air_pressure_at_mean_sea_level",
  "units": "Pa",
}

❯ standard_knowledge get air_pressure_at_mean_sea_level
air_pressure_at_mean_sea_level - Atmospheric Pressure at Sea Level - Pa
  Aliases: air_pressure_at_sea_level
  IOOS Category: Meteorology
  Related standards: air_pressure

Air pressure at sea level is the quantity often abbreviated as MSLP or PMSL. Air pressure is the force per unit area which would be exerted when the moving gas molecules of which the air is composed strike a theoretical surface of any orientation. "Mean sea level" means the time mean of sea surface elevation at a given location over an arbitrary period sufficient to eliminate the tidal signals.
```

## Goals

Provide a cross language way (by packaging Rust into Python, Javascript, and other languages) of sharing learnings from users of CF Standards.

- _QARTOD QC test suite suggestions_ (in progress)
- Translations from CF-ese
- Common column/variable names

## Contributing Knowledge

The core of the library isn't the code, but the knowledge that we have gained as a community in implementing the CF Standards in our work.

The knowledge is stored as YAML files in [standard_knowledge_core/standards/](./standard_knowledge_core/standards/) by `<standard_name>.yaml`.

```yaml
# standard_knowledge_core/standards/air_pressure_at_mean_sea_level.yaml
ioos_category: Meteorology
long_name: Atmospheric Pressure at Sea Level
common_variable_names:
- pressure
- atmospheric_pressure
- sea_level_pressure
related_standards:
- air_pressure
other_units:
- kPa
- bar
comments: |
  Raw pressure sensor values on buoys may need to be adjusted based on sensor tower height.
```

> [!NOTE]
>
> - IOOS categories are not (_currently_) validated, but the set of known values (derived from ERDDAP's internal list) is in [standard_knowledge_core/src/ioos_categories.rs](./standard_knowledge_core/src/ioos_categories.rs).

## Contributing Code

Cargo as manages the Rust components of the project, while Maturin and uv help keep things inline when working from the Python side of things.

### Rust Testing

`cargo test` will run tests in all the workspaces.

As the CLI changes, it's tests should be updated with `TRYCMD=overwrite cargo test`.

For new CLI tests, it's easiest to copy one of the files in `standard_knowledge_cli/tests/cmd`, and tweak the `args` to match the new command, then run `TRYCMD=overwrite cargo test` to replace the status code, stdout and stderr.

### Python testing

From `standard_knowledge_py`, `uv run pytest` will run tests.
It will also pick up changes in Rust, both for the Python bindings and changes in the core library as well.
`uv run python` will open a shell with the library rebuilt for interactive tinkering.

### Utils

- `utils/update_standards.py` - Run with `uv run --script utils/update_standards.py` to update the standard names and alias files from CF Conventions that are imported into the Rust library.

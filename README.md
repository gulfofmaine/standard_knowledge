# standard_knowledge
Programmatically augmenting CF Standards with operational knowledge

```py
# uv run python
# in standard_knowledge_py

import standard_knowledge

library = standard_knowledge.StandardsLibrary()

# Load all CF standards
library.load_cf_standards()

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
  "units": "Pa",
  "standard_name": "air_pressure_at_mean_sea_level",
}

❯ standard_knowledge get air_pressure_at_mean_sea_level
air_pressure_at_mean_sea_level - Pa
  Aliases: air_pressure_at_sea_level

Air pressure at sea level is the quantity often abbreviated as MSLP or PMSL. Air pressure is the force per unit area which would be exerted when the moving gas molecules of which the air is composed strike a theoretical surface of any orientation. "Mean sea level" means the time mean of sea surface elevation at a given location over an arbitrary period sufficient to eliminate the tidal signals.
```



For now, `cargo run` for an example of what it can load.

## Goals

Provide a cross language way (by packaging Rust into Python, Javascript, and other language libraries) of sharing learnings from IOOS data teams.

- QARTOD config suggestions
- Translations from CF-ese
- Common column/variable names
- Suggested conventions based on standards

Other ideas:

- Convention field suggestions/translations
- Rebuilding the core of the compliance checker/IOOS_QC in Rust so they can be run in the browser as well?

## Utils

- `utils/update_standards.py` - Run with `uv run --script utils/update_standards.py` to update the standard names and alias files from CF Conventions that are imported into the Rust library.

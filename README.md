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

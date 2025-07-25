# standard_knowledge
Programmatically augmenting CF Standards with operational knowledge

For now, `cargo run` to see what it can load.

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

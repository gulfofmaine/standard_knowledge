# Standard Knowledge

```py
# uv run python

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
standards = library.filter().by_variable_name("pressure")
# Notice the `.filter()`? It returns a StandardsFilter object,
# so you can chain multiple filters together.
# by_ioos_category, by_unit, has_qartod_tests

# Search for standards across multiple fields (name, aliases, common variable names, related standards)
under_pressure = library.filter().search("pressure")
```

Test with `uv run pytest`

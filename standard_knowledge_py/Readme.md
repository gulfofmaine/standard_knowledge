# Standard Knowledge

```py
# uv run python

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

Test with `uv run pytest`

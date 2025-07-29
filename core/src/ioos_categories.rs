// Categories from https://github.com/ERDDAP/erddap/blob/26c55b4f125ece1e70081a4c46565cf4b8bd6eda/WEB-INF/classes/gov/noaa/pfel/erddap/variable/EDV.java#L119

use std::collections::HashSet;

pub fn erddap_categories() -> HashSet<&'static str> {
    HashSet::from([
        "Bathymetry",
        "Biology", // bob added
        "Bottom Character",
        "CO2", // bob added pCO2 2011-05-19, 2011-10-11 changed to CO2
        "Colored Dissolved Organic Matter", // added 2011-05-19
        "Contaminants",
        "Currents", // was "Surface Currents"
        "Dissolved Nutrients",
        "Dissolved O2",
        "Ecology", // bob added
        "Fish Abundance",
        "Fish Species",
        "Heat Flux",
        "Hydrology", // bob added 2011-02-07
        "Ice Distribution",
        "Identifier",
        "Location",    // bob added
        "Meteorology", // bob added; use if not Temperature or Wind
        "Ocean Color",
        "Optical Properties", // what is dividing line?  OptProp is for atmosphere, too
        "Other",              // bob added
        "Pathogens",
        "Physical Oceanography", // Bob added 2011-10-11
        "Phytoplankton Species", // ??the species name? better to use Taxonomy??  Add
        // "Phytoplankton
        // Abundance"?
        "Pressure",     // bob added
        "Productivity", // bob added
        "Quality",      // bob added 2010-11-10
        "Salinity",
        "Sea Level",
        "Soils",       // bob added 2011-10-06
        "Statistics",  // bob added 2010-12-24
        "Stream Flow", // added 2011-05-19
        "Surface Waves",
        "Taxonomy", // bob added
        "Temperature",
        "Time",                   // bob added
        "Total Suspended Matter", // added 2011-05-19
        "Unknown",
        "Wind", // had Wind. 2011-05-19 has "Wind Speed and Direction", but that seems
        // unnecessarily
        // limited
        "Zooplankton Species", // ??the species name? better to use Taxonomy??
        "Zooplankton Abundance",
    ])
}

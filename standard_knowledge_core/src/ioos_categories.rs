// Categories from https://github.com/ERDDAP/erddap/blob/26c55b4f125ece1e70081a4c46565cf4b8bd6eda/WEB-INF/classes/gov/noaa/pfel/erddap/variable/EDV.java#L119

#[derive(Debug, Clone)]
pub enum IOOSCategory {
    Bathymetry,
    Biology, // bob added
    BottomCharacter,
    CO2,                           // bob added pCO2 2011-05-19, 2011-10-11 changed to CO2
    ColoredDissolvedOrganicMatter, // added 2011-05-19
    Contaminants,
    Currents, // was "Surface Currents"
    DissolvedNutrients,
    DissolvedO2,
    Ecology, // bob added
    FishAbundance,
    FishSpecies,
    HeatFlux,
    Hydrology, // bob added 2011-02-07
    IceDistribution,
    Identifier,
    Location,    // bob added
    Meteorology, // bob added; use if not Temperature or Wind
    OceanColor,
    OpticalProperties, // what is dividing line?  OptProp is for atmosphere, too
    Other,             // bob added
    Pathogens,
    PhysicalOceanography, // Bob added 2011-10-11
    PhytoplanktonSpecies, // ??the species name? better to use Taxonomy??  Add
    //   // "Phytoplankton
    //   // Abundance"?
    Pressure,     // bob added
    Productivity, // bob added
    Quality,      // bob added 2010-11-10
    Salinity,
    SeaLevel,
    Soils,      // bob added 2011-10-06
    Statistics, // bob added 2010-12-24
    StreamFlow, // added 2011-05-19
    SurfaceWaves,
    Taxonomy, // bob added
    Temperature,
    Time,                 // bob added
    TotalSuspendedMatter, // added 2011-05-19
    Unknown,
    Wind, // had Wind. 2011-05-19 has "Wind Speed and Direction", but that seems
    //   // unnecessarily
    //   // limited
    ZooplanktonSpecies, // ??the species name? better to use Taxonomy??
    ZooplanktonAbundance,
}

// impl IOOSCategory {
//     fn as_str(&self) -> &'static str {
//         match self {
//             IOOSCategory::Bathymetry => "Bathymetry",
//             IOOSCategory::Biology => "Biology", // bob added
//             IOOSCategory::BottomCharacter => "Bottom Character",
//             IOOSCategory::CO2 => "CO2", // bob added pCO2 2011-05-19, 2011-10-11 changed to CO2
//             IOOSCategory::ColoredDissolvedOrganicMatter => "Colored Dissolved Organic Matter", // added 2011-05-19
//             IOOSCategory::Contaminants => "Contaminants",
//             IOOSCategory::Currents => "Currents", // was "Surface Currents"
//             IOOSCategory::DissolvedNutrients => "Dissolved Nutrients",
//             IOOSCategory::DissolvedO2 => "Dissolved O2",
//             IOOSCategory::Ecology => "Ecology", // bob added
//             IOOSCategory::FishAbundance => "Fish Abundance",
//             IOOSCategory::FishSpecies => "Fish Species",
//             IOOSCategory::HeatFlux => "Heat Flux",
//             IOOSCategory::Hydrology => "Hydrology", // bob added 2011-02-07
//             IOOSCategory::IceDistribution => "Ice Distribution",
//             IOOSCategory::Identifier => "Identifier",
//             IOOSCategory::Location => "Location", // bob added
//             IOOSCategory::Meteorology => "Meteorology", // bob added; use if not Temperature or Wind
//             IOOSCategory::OceanColor => "Ocean Color",
//             IOOSCategory::OpticalProperties => "Optical Properties", // what is dividing line?  OptProp is for atmosphere, too
//             IOOSCategory::Other => "Other",                          // bob added
//             IOOSCategory::Pathogens => "Pathogens",
//             IOOSCategory::PhysicalOceanography => "Physical Oceanography", // Bob added 2011-10-11
//             IOOSCategory::PhytoplanktonSpecies => "Phytoplankton Species", // ??the species name? better to use Taxonomy??  Add
//             // "Phytoplankton
//             // Abundance"?
//             IOOSCategory::Pressure => "Pressure", // bob added
//             IOOSCategory::Productivity => "Productivity", // bob added
//             IOOSCategory::Quality => "Quality",   // bob added 2010-11-10
//             IOOSCategory::Salinity => "Salinity",
//             IOOSCategory::SeaLevel => "Sea Level",
//             IOOSCategory::Soils => "Soils", // bob added 2011-10-06
//             IOOSCategory::Statistics => "Statistics", // bob added 2010-12-24
//             IOOSCategory::StreamFlow => "Stream Flow", // added 2011-05-19
//             IOOSCategory::SurfaceWaves => "Surface Waves",
//             IOOSCategory::Taxonomy => "Taxonomy", // bob added
//             IOOSCategory::Temperature => "Temperature",
//             IOOSCategory::Time => "Time", // bob added
//             IOOSCategory::TotalSuspendedMatter => "Total Suspended Matter", // added 2011-05-19
//             IOOSCategory::Unknown => "Unknown",
//             IOOSCategory::Wind => "Wind", // had Wind. 2011-05-19 has "Wind Speed and Direction", but that seems
//             // unnecessarily
//             // limited
//             IOOSCategory::ZooplanktonSpecies => "Zooplankton Species", // ??the species name? better to use Taxonomy??
//             IOOSCategory::ZooplanktonAbundance => "Zooplankton Abundance",
//         }
//     }
// }

// impl IOOSCategory {
// pub fn from_str(s: &str) -> Option<Self> {
//     match s {
//         "Bathymetry" => Some(IOOSCategory::Bathymetry),
//         "Biology" => Some(IOOSCategory::Biology),
//         "Bottom_Character" => Some(IOOSCategory::Bottom_Character),
//         "CO2" => Some(IOOSCategory::CO2),
//         "Colored Dissolved Organic Matter" => Some(IOOSCategory::Colored_Dissolved_Organic_Matter),
//         "Contaminants" => Some(IOOSCategory::Contaminants),
//         "Currents" => Some(IOOSCategory::Currents),
//         "Dissolved Nutrients" => Some(IOOSCategory::Dissolved_Nutrients),
//         "Dissolved O2" => Some(IOOSCategory::Dissolved_O2),
//         "Ecology" => Some(IOOSCategory::Ecology),
//         "Fish Abundance" => Some(IOOSCategory::Fish_Abundance),
//         "Fish Species" => Some(IOOSCategory::Fish_Species),
//         "Heat Flux" => Some(IOOSCategory::Heat_Flux),
//     }
// }

// }

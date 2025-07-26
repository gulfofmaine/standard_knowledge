import standard_knowledge


def test_get_standard():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()

    standard = library.get("air_pressure_at_mean_sea_level")

    assert standard.name == "air_pressure_at_mean_sea_level"
    assert standard.long_name is None
    assert standard.unit == "Pa"
    assert "Air pressure is the force per unit" in standard.description
    assert "air_pressure_at_sea_level" in standard.aliases


def test_get_standard_attrs():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()

    suggestion = {
        "name": "air_pressure_at_mean_sea_level",
        "long_name": "Air Pressure at Sea Level",
        "ioos_category": "Meteorology",
    }

    library.apply_suggestions([suggestion])

    standard = library.get("air_pressure_at_mean_sea_level")

    attrs = standard.attrs()

    assert attrs["standard_name"] == "air_pressure_at_mean_sea_level"
    assert attrs["units"] == "Pa"
    assert attrs["long_name"] == "Air Pressure at Sea Level"
    assert attrs["ioos_category"] == "Meteorology"

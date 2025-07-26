import standard_knowledge


def test_get_standard_attrs():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    standard = library.get("air_pressure_at_mean_sea_level")

    attrs = standard.attrs()

    assert attrs["standard_name"] == "air_pressure_at_mean_sea_level"
    assert attrs["units"] == "Pa"

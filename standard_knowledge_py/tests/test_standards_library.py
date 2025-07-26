import pytest

import standard_knowledge


def test_library_load_standards():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()


def test_get_standard_by_name():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    standard = library.get("air_pressure_at_mean_sea_level")
    assert str(standard) == "<Standard: air_pressure_at_mean_sea_level>"
    assert standard.name == "air_pressure_at_mean_sea_level"


def test_get_standard_by_alias():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    standard = library.get("air_pressure_at_sea_level")
    assert standard.name == "air_pressure_at_mean_sea_level"


def test_unknown_standard():
    library = standard_knowledge.StandardsLibrary()
    with pytest.raises(KeyError):
        library.get("air_pressure_at_sea_level")


def test_suggestions_must_have_name():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()

    suggestion = {"long_name": "Air Pressure"}

    with pytest.raises(KeyError) as e:
        library.apply_suggestions([suggestion])

    assert "name of the standard" in str(e.value)


def test_can_add_suggestions():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    standard = library.get("air_pressure_at_sea_level")
    assert standard.name == "air_pressure_at_mean_sea_level"
    assert standard.long_name is None

    suggestion = {
        "name": "air_pressure_at_mean_sea_level",
        "long_name": "Air Pressure at Sea Level",
        "ioos_category": "Meteorology",
        "common_variable_names": ["air_pressure", "pressure"],
        "related_standards": ["air_pressure"],
        "other_units": ["bar"],
        "comments": "Some adjustment for altitude may be needed"
    }

    library.apply_suggestions([suggestion])

    updated_standard = library.get("air_pressure_at_sea_level")
    assert updated_standard.name == suggestion["name"]
    assert updated_standard.long_name == suggestion["long_name"]
    assert updated_standard.ioos_category == suggestion["ioos_category"]
    assert "pressure" in updated_standard.common_variable_names
    assert "air_pressure" in updated_standard.related_standards
    assert "bar" in updated_standard.other_units
    assert updated_standard.comments == suggestion["comments"]

    assert standard != updated_standard

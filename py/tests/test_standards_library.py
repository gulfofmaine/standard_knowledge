import pytest

import standard_knowledge


KNOWLEDGE = {
    "name": "air_pressure_at_mean_sea_level",
    "long_name": "Air Pressure at Sea Level",
    "ioos_category": "Meteorology",
    "common_variable_names": ["air_pressure", "pressure"],
    "related_standards": ["air_pressure"],
    "other_units": ["bar"],
    "comments": "Some adjustment for altitude may be needed",
}


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


def test_knowledge_must_have_name():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()

    knowledge = {"long_name": "Air Pressure"}

    with pytest.raises(KeyError) as e:
        library.apply_knowledge([knowledge])

    assert "name of the standard" in str(e.value)


def test_can_add_knowledge():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    standard = library.get("air_pressure_at_sea_level")
    assert standard.name == "air_pressure_at_mean_sea_level"
    assert standard.long_name is None

    library.apply_knowledge([KNOWLEDGE])

    updated_standard = library.get("air_pressure_at_sea_level")
    assert updated_standard.name == KNOWLEDGE["name"]
    assert updated_standard.long_name == KNOWLEDGE["long_name"]
    assert updated_standard.ioos_category == KNOWLEDGE["ioos_category"]
    assert "pressure" in updated_standard.common_variable_names
    assert "air_pressure" in updated_standard.related_standards
    assert "bar" in updated_standard.other_units
    assert updated_standard.comments == KNOWLEDGE["comments"]

    assert standard != updated_standard


def test_find_standards_by_variable_names():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    library.apply_knowledge([KNOWLEDGE])

    standards = library.filter().by_variable_name("pressure")

    standard = standards[0]
    assert standard.name == KNOWLEDGE["name"]


def test_find_standards_by_variable_names_knowledge():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    library.load_knowledge()

    standards = library.filter().by_variable_name("atmospheric_pressure")

    standard = standards[0]
    assert standard.name == "air_pressure_at_mean_sea_level"
    assert standard.ioos_category == "Meteorology"


def test_search_standard():
    library = standard_knowledge.StandardsLibrary()
    library.load_cf_standards()
    library.apply_knowledge([KNOWLEDGE])

    standards = library.filter().search("pressure")

    assert len(standards) > 0
    pressure = standards[0]
    assert pressure.name == KNOWLEDGE["name"], (
        "since there isn't a direct name or alias match, the suggested column should make it first"
    )

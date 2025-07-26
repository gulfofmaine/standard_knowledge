# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "beautifulsoup4",
#     "lxml",
#     "pyyaml",
#     "requests",
# ]
# [tool.uv]
# exclude-newer = "2025-07-17T00:00:00Z"
# ///
from pathlib import Path
import requests
from bs4 import BeautifulSoup
import yaml

standards_url = "https://raw.githubusercontent.com/cf-convention/vocabularies/refs/heads/main/docs/cf-standard-names/current/cf-standard-name-table.xml"
cf_yaml = (
    Path(__file__).parent / "../standard_knowledge_core/standards/_cf_standards.yaml"
)

standard_names = {}
aliases = {}

response = requests.get(standards_url)

soup = BeautifulSoup(response.text, "xml")

for node in soup.find_all("entry"):
    name = str(node.get("id"))
    standard_names[name] = {
        "unit": str(node.canonical_units.string),
        "description": str(node.description.string) or "",
    }

for node in soup.find_all("alias"):
    name = str(node.get("id"))
    aliases[name] = str(node.entry_id.string)


dump = {"standard_names": standard_names, "aliases": aliases}

with cf_yaml.open("w") as f:
    yaml.dump(dump, f)

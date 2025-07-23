# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "beautifulsoup4",
#     "lxml",
#     "requests",
# ]
# [tool.uv]
# exclude-newer = "2025-07-17T00:00:00Z"
# ///
from pathlib import Path
import requests
from bs4 import BeautifulSoup

standards_url = "https://raw.githubusercontent.com/cf-convention/vocabularies/refs/heads/main/docs/cf-standard-names/current/cf-standard-name-table.xml"

standard_names = {}
aliases = {}

response = requests.get(standards_url)

soup = BeautifulSoup(response.text, "xml")

for node in soup.find_all("entry"):
    name = node.get("id")
    standard_names[name] = {
        "unit": node.canonical_units.string,
        "description": node.description.string or "",
    }

for node in soup.find_all("alias"):
    name = node.get("id")
    aliases[name] = node.entry_id.string

with (Path(__file__).parent / "../src/standard_names/raw_standard_names.tsv").open(
    "w"
) as f:
    for name, data in standard_names.items():
        f.write(f"{name}\t{data['unit']}\t{data['description'].replace('\n', ' ')}\n")


with (Path(__file__).parent / "../src/standard_names/raw_aliases.tsv").open("w") as f:
    for alias, name in aliases.items():
        f.write(f"{alias}\t{name}\n")

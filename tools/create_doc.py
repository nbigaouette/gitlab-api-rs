#!/usr/bin/env python3

import sys
import re

# Source: https://gitlab.com/gitlab-org/gitlab-ce/raw/master/doc/api/projects.md
filename = "projects.md"

with open(filename, 'r') as f:
    markdown = f.read()
# print("markdown:", markdown)

# Strip out all `json` code blocks included in the file.
p = re.compile("```json.*?```", re.MULTILINE | re.DOTALL)
markdown_wo_json = re.sub(p, "", markdown)

GET_block = "GET /"

sectionsList = re.sub("[^#]#", "TOSPLIT#", markdown_wo_json).split("TOSPLIT")
# print("sectionsList:", sectionsList)
for section in sectionsList:
    print("-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
    if GET_block in section:
        print("***************************** GET!!!")
        print(section)

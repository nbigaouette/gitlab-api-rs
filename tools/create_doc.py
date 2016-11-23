#!/usr/bin/env python3

import os
import re
import sys
import urllib.request


api_filename = "projects.md"
url = "https://gitlab.com/gitlab-org/gitlab-ce/raw/master/doc/api/" + api_filename


doc_dir = "doc_tmp"
if not os.path.exists(doc_dir):
    os.makedirs(doc_dir)


filename, headers = urllib.request.urlretrieve(url)

with open(filename, 'r') as f:
    markdown = f.read()
# print("markdown:", markdown)
urllib.request.urlcleanup()

# Strip out all `json` code blocks included in the file.
p = re.compile("```json.*?```", re.MULTILINE | re.DOTALL)
markdown_wo_json = re.sub(p, "", markdown)

GET_block = "GET /"

p_GET_block = re.compile("```\n(%s.*?)\n```" % GET_block, re.MULTILINE | re.DOTALL)
p_GET_variable = re.compile("(:[^/]*)")


sectionsList = re.sub("[^#]#", "TOSPLIT#", markdown_wo_json).split("TOSPLIT")

for section in sectionsList:
    if GET_block in section:
        lines = section.splitlines()
        title = lines[0].replace("#", "").strip()
        # print("title:", title)

        # section = re.sub(p_GET_block, "```\n```")
        m = p_GET_block.search(section)
        GET_command = m.group(1)
        GET_variables = p_GET_variable.findall(GET_command)
        # Sort the variables in decreasing order of _length_. The reason is that a replace of a shorter
        # variable might catch a longer one and corrupt the final result.
        GET_variables.sort(key = lambda s: -len(s))

        # Replace occurrences of the found variables with upper case, removing the ":"
        new_GET_command = GET_command
        for GET_variable in GET_variables:
            new_GET_command = new_GET_command.replace(GET_variable, GET_variable.replace(":", "").upper())

        # section = section.replace(GET_command, new_GET_command)
        lines = [line.replace(GET_command, new_GET_command) for line in lines]

        # print("title:", title)
        filename = api_filename.replace(".md", "") + "-GET-" + title.replace(" ", "-").lower() + ".md"
        print("filename:", filename)
        full_filename = os.path.join(doc_dir, filename)
        with open(full_filename, "w") as f:
            f.write("//! # %s\n" % title)
            for line in lines[1:]:
                f.write("//! %s\n" % line)

#!/usr/bin/env bash

# https://github.com/kbknapp/cargo-graph
# cargo install cargo-graph

cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange > gitlab-api-rs.dot
dot -Tpng > gitlab-api-rs.png gitlab-api-rs.dot

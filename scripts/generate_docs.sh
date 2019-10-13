#!/bin/bash
set -ex

project_root() {
    cd "$(dirname "$0")"
    cd ..
}

generate_docs() {
    cargo doc --open --no-deps --document-private-items
}

project_root
generate_docs

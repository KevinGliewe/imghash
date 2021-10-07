#!/usr/bin/env python3
import fileinput
import os
import sys

script_dir = os.path.dirname(os.path.realpath(__file__))
repo_dir = os.path.abspath(os.path.join(script_dir, '..'))
cargo_file = os.path.join(repo_dir, 'Cargo.toml')

version = '0.0.0'

if len(sys.argv) > 1:
    version = sys.argv[1]

find = "version = \"0.0.0\""
replace = f"version = \"{version}\""

print(f"Updating version in {cargo_file}")
print(f"  from {find} to {replace}")



with fileinput.FileInput(cargo_file, inplace=True, backup='.bak') as file:
    for line in file:
        print(line.replace(find, replace), end='')
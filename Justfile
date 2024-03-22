set fallback := true
set shell := ["bash", "-uc"]
set windows-shell := ["sh", "-uc"]

# `just --list` (or just `just`) will print all the recipes in
# the current Justfile. `just RECIPE` will run the macro/job.
#
# Read up on just here: https://github.com/casey/just

_default:
  @just --list

install:
  pnpm install

build:
  cargo build

test:
  cargo test

<!--
SPDX-FileCopyrightText: 2023 Christina Sørensen
SPDX-FileContributor: Christina Sørensen

SPDX-License-Identifier: AGPL-3.0-only
-->

# Changelog

## [0.1.7] - 2023-11-02

### Bug Fixes

- Remove fortune I'm unsure about

### Features

- Port 241 lines of medicine

### Build

- Bump tempfile from 3.8.0 to 3.8.1
- Change to rime.cx and semnix

## [0.1.6] - 2023-10-26

### Features

- Start porting food
- Add some fedi tune
- Begin porting humorists

### Miscellaneous Tasks

- Update all dependencies
- Release fortune-kind v0.1.6

## [0.1.5] - 2023-10-19

### Features

- Add a fortune
- Add more fortunes
- Add nethack sounds
- Start porting debian

### Miscellaneous Tasks

- Release fortune-kind v0.1.5

### Build

- Bump actions/checkout from 3 to 4
- Bump DeterminateSystems/nix-installer-action from 5 to 6

## [0.1.4] - 2023-10-12

### Bug Fixes

- Removed inaccurate fortune
- Remove unnescesarry joke
- Removed confucious inside fortune cookie due to request
- Remove homophobic fortune
- Add progress marker

### Features

- Transfer a few goedel tunes
- Port more tunes
- Finish porting goedel
- Start porting disclaimer
- Port more disclaimers
- Finish porting disclaimer
- Start porting
- Removed fortuens I weren't sure about
- Port more news
- Port all of magic
- Begin porting linux
- OH and added
- Add more fortunes
- Port more fortunes
- Begin porting kids
- Add fortunes from fedi

### Miscellaneous Tasks

- Release fortune-kind v0.1.4

### Refactor

- Move random to own file
- Move out to own file
- Move to own file

### Build

- Update flake.lock

### Ci

- Create flakehub-publish-tagged.yml

## [0.1.3] - 2023-10-09

### Bug Fixes

- Fix gitignore
- Fix gitignore further
- Fix build, phew :p

### Documentation

- Update flake description

### Miscellaneous Tasks

- Release fortune-kind v0.1.3

### Testing

- Fix tests in ci

## [0.1.2] - 2023-10-09

### Documentation

- Add Nix/NixOS installation instructions
- Fix gh alerts
- Change layout slightly
- Fix Nix/NixOS installation instructions
- Change layout
- Make motivation more clear

### Features

- Install from flake
- Update justfile
- Autogen bash completions
- Add all completion types
- Automatic manpage generation
- Gen/install manpages, completion
- Auto-install shell completions, man pages

### Miscellaneous Tasks

- Release fortune-kind v0.1.2

### Refactor

- Command line input

### Build

- Bump clap from 4.4.4 to 4.4.6
- Bump DeterminateSystems/nix-installer-action from 4 to 5
- Use eza style automagic release

## [0.1.1] - 2023-10-06

### Bug Fixes

- Make io_err panic on default arm

### Documentation

- Create editorial guidelines
- Fix typos
- Fix typo
- Linewrap EDITORIAL.md
- Add revision policy
- Remove empty lines in ordered list
- Wrap lines, add editorial

### Features

- Add pets: deeleted some repeats, removed a story that features animal violence
- Remove a repeat fortune from pets, remove poorly formatted goldfish fortune, remove reference to suicide in pets
- Delete oldtunes/pets
- Introduce FORTUNE_DIR env var

### Miscellaneous Tasks

- Bump version to v0.1.1

### Refactor

- Remove unused module
- Fix some clippy lints
- Fix clippy lint
- Clippy passes
- S/fortunes_dir/fortune_dir/g
- Change error printed for NotFound to be `err` not `io_err`
- Introduce env getters
- Make fortune file error handling reusable
- Move `handle_errors` out of get_quote

### Build

- Bump actions/checkout from 2 to 4
- Bump clap from 4.3.23 to 4.4.4
- Format flake
- Format flake

### Ci

- Add some CI

## [0.1.0] - 2023-09-18

### Documentation

- Add readme to root
- Add bare minimum
- Fix spelling mistake
- Fix spelling mistake
- Update README.md
- $$$$$$$ :D
- Document search module
- Document file module
- Document random module
- Document fortunes module
- Add todos for removal of example code
- Add demo gif
- Update README.md

### Features

- Init search_string
- Add read_all_files
- Add find MVP
- Fortune-mod style search

### Miscellaneous Tasks

- Initial commit
- Add ascii-art fortunes
- Use pratchett fortunes
- Use translate-me fortunes
- Use majority of paradoxum fortunes
- Remove off/fortunes, unfunny
- Remove off/rotated
- Used half of off/art
- Removed off/racism, for obvious reasons
- Use most of off/cookie
- Remove misandry (see reasoning)
- Remove misogyny
- Remove rest of paradoxum
- Use all of tao
- Release 0.1.0

### Refactor

- Add file module
- Introduce modules random, fortune
- Module fortunes -> fortune

### Testing

- Add no_match
- Test file module
- Testing for get_quote
- Add tests for random::random

### Build

- Change name to fortune-kind
- Remove old cmake
- Lock cargo file
- Remove offensive fortunes cmake file
- Add grep (ripgrep)
- Add grep-matcher, grep-regex
- Add tempdir
- Move tempfile to dev-dependencies
- Add dev-dependency assert-cmd
- Add release script


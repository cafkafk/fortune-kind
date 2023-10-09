# Changelog

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


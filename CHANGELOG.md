# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0](https://github.com/PlexSheep/numf/compare/v0.4.1...v0.5.0) - 2026-06-04

### Added

- twos complement stuff goes both ways #13
- convert twos complement for positive signed integers too
- finally comvert twos complement to absolute
- basic twos complement conversion #13
- adding signed integer conversion is hard #13
- *(parser)* ignore underscores #4

### Fixed

- logger did not actually get initialized
- bintols module was not declared in main module

### Other

- thats it i have no idea what the fuck a numf_parser is or does
- negative inputs are currently not supported #13
- more trait shenanigans
- UseManyTraitsForGenericUnsignedInt macro
- add tests for the signed integer format
- clarify what the padding flag does
- allow "dead code" for some public APIs
- bintols without submodules
- Merge pull request #12 from PlexSheep/dependabot/github_actions/actions/checkout-6
- adjust license date and set my actual name
- repository link was wrong
- adjust crate level docs
- [**breaking**] remove libpt by using alternatives or inlining
- add pre commit hooks
- remove old release scripts
- update license
- *(deps)* bump stefanzweifel/git-auto-commit-action from 5 to 7
- *(help)* add note for undescores and list the formats better
- *(parser)* add a test to confirm that underscores are ignored #4
- Merge branch 'master' into devel
- move everything to github for ci/cd

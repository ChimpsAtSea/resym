# Changelog

## [Unreleased]

### Added

- Implement symbol reconstruction and diffing capabilities
- Add 4 commands to `resymc`: `list-symbols`, `dump-symbol`, `dump-all-symbols` and `diff-symbol`
- Add a `MSVC` primitive type representations (@HaydnTrigg)
- Add a setting to switch display of integer values between decimal and hexadecimal (@HaydnTrigg)
- Add new `classes`, `unions` and `enums` filters for type searches
- Allow navigating type and symbol lists with the keyboard via arrow up/down and page up/down (proposal by @0xeb)

### Changed

- Hexadecimal integer values are now displayed with the appropriate number of digits (@HaydnTrigg)

## [0.4.0] - 2024-03-24

### Added

- Allow opening a PDB file and diffing two PDB files using drag and drop in `resym` (@learn-more)
- Add support for the `wasm32-unknown-unknown` target
- Publish a web version of `resym` from the `main` branch automatically
- Allowing loading PDBs from URLs in `resym` (the feature can be disabled at build time)
- Implement basic module reconstruction and diffing capabilities
- Add 3 new commands to `resymc`: `list-modules`, `dump-module`, `diff-module`
- Add support for small MSF file format (e.g., VC++ 6 PDBs) in the `pdb` crate fork (@jon-zu)
- Add "Xrefs to" and "Xrefs from" tabs in the bottom panel of `resym`
- Add "Find Xrefs to" button to find types that use the current type in `resym`
- Add a keyboard shortcut to look for cross-references to a type (Alt+X) in `resym`
- Reconstructed output for C types can now be compiled without modifications

### Changed

- `resym` and `resymc` can be built without `rayon` (but are still built with it by default)
- Type list is now ordered alphabetically in `resym`

### Fixed

- Fix wrong size detection for unnamed unions in structs, leading to infinite loops in certain cases
- Fix reconstruction of function pointer arrays
- Fix reconstruction of function pointer's arguments
- Fix reconstruction of certain class/struct constructors

## [0.3.0] - 2023-02-19

### Added

- Allow switching between different primitive type representations
- Add a "Save" button to easily save reconstructed types into files
- Add keyboard shortcuts for opening PDB files and saving reconstructed types (Ctrl+O and Ctrl+S respectively)
- Allow changing the editor's font size via the settings menu
- Add a `dump-all` command to `resymc`, which dumps all types in a given PDB file (proposal by @xarkes)

### Fixed

- Reconstruct access specifiers for base classes (@TrinityDevelopers)
- Reconstruct type qualifiers for member functions (@TrinityDevelopers)
- Fix reconstruction of function pointer return types for member functions (@TrinityDevelopers)
- Fix incorrect reconstruction of class/struct and union destructors (@TrinityDevelopers)
- Fix "File" menu not closing when clicking a button (@mrexodia)
- Fix field offsets and struct/classes/unions sizes being truncated when greater than 2^16 (@xarkes)
- Fix the `list` command not outputting new lines in output files in `resymc` (@xarkes)
- Fix incorrect reconstruction of bitfields as unions
- Reconstruct C++20's **char8_t** primitive type

## [0.2.0] - 2022-05-22

### Added

- Command-line version of the tool (`resymc`)
- Syntax highlighting (both in the GUI and in the CLI version of the tool)
- Basic type diffing capability
- Line numbering (only for the GUI version of the tool)

## [0.1.0] - 2022-05-04

Initial release

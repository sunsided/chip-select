# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.2.0] - 2024-07-06

[0.2.0]: https://github.com/sunsided/chip-select/releases/tag/v0.2.0

### Added

- Added a guard that allows to deselect the chip on drop.

### Removed

- Removed the notion of chip auto-selects since for SPI, it's common to unselect anyway.

## [0.1.0] - 2024-07-06

[0.1.0]: https://github.com/sunsided/chip-select/releases/tag/v0.1.0

### Added

- Initial commit providing the `ChipSelect<Ping>`, `ActiveLow` and `ActiveHigh` traits.

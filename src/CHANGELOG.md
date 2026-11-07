# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]
### Added
- Красивый ASCII-баннер с ником и отсылкой к Yuri.

## [1.0.1] - 2026-11-07
### Added
- Модуль `report.rs` для цветного вывода портов.
- Обработка ошибки при отсутствии аргументов (вместо паники).

### Changed
- Парсинг аргументов теперь возвращает `Result` вместо паники.

### Fixed
- Исправлена ошибка `index out of bounds` при запуске без аргументов.
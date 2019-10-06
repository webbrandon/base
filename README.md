# Rust CLI Baseline

This is a baseline for creating a clean CLI interface with autocompletion and verbose logging. Please feel free to clone and use it for your own development.

**Features provided in baseline:**  
- StructOpt menu module
- Verbose logging
- Generate autocompletion script

## StuctOpt Menu Module

A StuctOpt baseline menu module is provided in `cli` module. Basic logic for autocompletion menu option and verbose logging are already integrated.

## Generating Autocomplete Module On Build

Auto completion script for the below list of supported shells are generated at build time base on the menu options provided from StructOpt menu module.  Nothing needs to be done to manage this because it will self update.

**Supported shells:**
- Bash
- Zsh
- Fish
- Elvish
- PowerShell

## Verbose Logging

Verbose logging is provided with the use of the `debug!(log_config, "{}", x)` macro.

---

## Work In Progress
Feel free to contribute or use in any way.s

# ![base](docs/image/base.png)

This is a rust language baseline for creating a CLI interface with robust features for a variety of use cases. Please feel free to clone and use it for your own development.

**Current features provided in base.**  
- StructOpt menu module
- Verbose logging
- Generate autocompletion script

## Cli Interface

StuctOpt is used in `CmdCtl` module which provides a baseline to cli interface used to handle request.  Add subcommand modules to `src/command_control/cmd_model` directory and include your subcommand module in the `Commands` enum in `src/command_control/cmd_model/mod.rs`. If you wish to extend base interface the entry point is the `CmdCtl` module.

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

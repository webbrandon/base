# Generating A Rust Cli Autocomplete SubCommand At Build

This is a demo of how you can create an autocomplete subcommand based on your 
cli's subcommand and flag options at build time for various shells.

### What I will use:
I will be using several crates which are currently considered to be the best in 
class for common use cases.  First we will use Clap.rs so we can easily create 
common cli feature and nicely format subcommands and flags into a programmable 
structure using StructOpt.

#### Autocomplete for shell:
- bash
- zsh
- fish
- elvish
- powershell

---
#### Work In Progress
This demo works but the build.rs file and template segments used to achieve this 
a very hackie.  I would like to come back and do this more elegantly but since it
only a build time feature I may see if others commit suggestions instead over time.



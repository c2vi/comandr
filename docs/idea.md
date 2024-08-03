
# general
- Comandr should be able to be included into a project of any UI toolkig and language (web, python-qt, slint, flutter, ....).
- Comands
- there is a "state", a key-value thing, that influences how commands work, what commands are availavle and the used hotkeys. (eg for things like VIM modes)
- hotkeys are in groups, that can be enabled
    - eg when the commandr window is shown the "comandr-window" group with the "comandr.window.up" command becomes available...
- when setting hotkeys you can specify the arguments to the command (eg: CTRL+L always makes a new folder named hello)
- there is config (also key-value thingy), that get loaded into the state on startup
- quick way to define custom commans with some code....
- very quick way to define hotkeys
- the ui components are kind of interfaces and an implementation can be provided from multiple places, eg: js, qt, slint, swing, ...

# feature ideas
- modules, that can listen for any pattern on the input (eg for natural language unit conversions)
- be able to favourite commnads, so that they appear on top
- unit conversions
    - a module
- hex, dec, ascii, .... conversions
    - a module
- simple math
    - obviosly you can also launch a matlab shell in there ... thanks to victorinix
    - a module
- run commands
    - applications are commands (if used as a launcher)
    - a module, that scans the $PATH for binaries
- before pressing enter do a shortcut, to set a shortcut for the thing you have entered in
- things like "s hello world", eg to search for hello world in settings
- look at alfred and powertoys runner features
- launch other shells: bash, python, js, matlab, fish
- load your own config (for hotkeys) from eg the url c2vi.dev/c



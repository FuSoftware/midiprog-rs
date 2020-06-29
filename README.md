# MIDI Programmer

The goal of this project is to provide a simple CLI tool to easily interact with any synthesizer, using both standard MIDI commands, as well as SYSEX commands.

## Syntax

Currently, two syntaxes are in use, one for the configuration files, and one for the interpreter.

### Config

Config files are used to specify the MIDI commands used by the synthesizers, their aliases, as well as where each parameter should be inserted in them.

```
synth
    -id "ju-2"
    -manufacturer "Roland"
    -name "Alpha Juno 2"

command
    -name "Individual Tone Parameter"
    -midi "F0 41 36 0n 23 20 01 p v F7"
    -@parameter "n : 0.5 : Channel"
    -@parameter "p : 1 : Parameter"
    -@parameter "v : 1 : Value"
    -alias "ipr parameter param"
```

### Interpreter

```
midiconfig /home/midiprog-rs/data/midi.cfg
config /home/midiprog-rs/data/dw-8000.cfg
synth dw-8000
port 1 1
channel 0
```

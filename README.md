# kgemu

This is a work in progress rust create that will complie and run assembly code.

The compiler and emulator use json RAW files to get definitions of how the assembly language is written and complied in to machine code, and how the machine code is run.

Right now the first implmented feature is parsing a sample thumb assembly using regex into the component parts of each line.

Next cleaning up the difinitions and posible start work on converting them to and from json.
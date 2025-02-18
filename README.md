# kgemu

This is a work in progress rust create that will complie and run assembly code.

The compiler and emulator use json RAW files to get definitions of how the assembly language is written and complied in to machine code, and how the machine code is run.


### Implemented/testing:

- Definition for language (might refactor regex to be even more generic/user defined)
- Parse code using language def into parts by line
- Definition for processor

### Next to Work On:

- create sturcts or enmums for language opc
	- needs the opc name
	- the diffrent versions based on registers or literals
- link language to processor so complier can start taking parsed code and convert to binary inctructions.

### TODO:

- Lay out logic for two pass complier mostly for keeping track of labels and relative address offsets.
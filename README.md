# chip8-emulator
A CHIP-8 interpreted programming language written in Rust

## CHIP-8 Specifications
### Memory
- CHIP-8 has 4096 (0x1000) memory locations and all of them are 8 bits.
- 0 ~ 512 (0x000 ~ 0x200): Used by CHIP-8 interpreter itself.
- 3840 ~ 4096 (0xF00 ~ 0xFFF): Used as a display refresh.
  - 3744 ~ 3839 (0xEA0 ~ 0xEFF): Used as a call stack, internal use, and other variables.
> In modern CHIP-8 implementations, where the interpreter is running natively outside the 4K memory space, there is no need for any of the lower 512 bytes memory space to be used, but it is common to store font data in those lower 512 bytes (0x000-0x200).


## Commands

Create documents
```
$ cargo rustdoc -- --document-private-items
```

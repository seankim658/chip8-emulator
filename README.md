# Chip-8 Emulator

Diving into emulation development at the same time as continuing to learn Rust. In doing some research, building a Chip-8 emulator is the has become the "Hello World" for emulation development.

Chip-8 is an interpreted programming lanuage developed by Joseph Weibecker for an 1802 Microprocessor. Chip-8 programs are run on a Chip-8 virtual machine, which is what I will be building an emulator for in this project. I frequently referenced and followed [this](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) guide as I worked through this project.

## Chip-8 Virtual Machine Tech Specs

- 64x32 monochrome display.
  - Graphics are drawn to screen by drawing sprites which 8 pixels wide and can be 1 to 15 pixels in height.
  - Sprite pixels are `XOR'd` with their corresponding screen pixels.
  - Sprite pixels that are `set` flip the color of the corresponding screen pixel, while unset sprite pixels do nothing.
  - The carry flag (`VF`) is set to `1` if any screen pixels are flipped from set to unset when a sprite is drawn and set to `0` otherwise, this feature is used for collision detection.
- Sixteen 8-bit general purpose registers.
  - The register names go from `V0` to `VF` (although the `VF` register doubles as a flag and should be avoided).
- 16-bit index register (`I`).
  - Although 16-bits, only the lower 12 bits are used for addressing purposes due to the 4KB memory size (12 bits is sufficient to address any location within this memory range).
- 16-bit program counter (`PC`).
- 4KB (or 4096 bytes) of memory.
- 16-bit stack for calling and returning from subroutines.
- 16-key hex keyboard input.
  - The 16 keys are labelled with hexadecimal values (0-F).
- Two special registers which decrement at 60 hertz and trigger upon reaching zero.
  - 8-bit delay timer: Used for time-based game events.
  - 8-bit sound timer: Used to trigger the audio beep.

## Crates

- `chip-core`: Defines the backend emulator implementation.

## Opcode Table

Chip-8 has 35 opcodes, which are all two bytes long and stored big-endian (meaning the most significant byte of a word is stored at the smallest memory address and the least significant byte at the largest).

| Idx | Opcode | Shorthand  | Description                                                                                                                                                                              |
| --- | ------ | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | `0000` | `NOP`      | Do nothing. Useful for syncing timing and alignment.                                                                                                                                     |
| 2   | `00E0` | `CLS`      | Clear screen, reset the screen buffer.                                                                                                                                                   |
| 3   | `00EE` | `RET`      | Return from subroutine by setting the program counter to the memory address to return the return address.                                                                                |
| 4   | `1NNN` | `JUMP`     | Jump to memory location `NNN` by setting the program counter to that memory location.                                                                                                    |
| 5   | `2NNN` | `CALL`     | Call subroutine at the memory location `NNN`. Push current program counter to the stack to use later when returning from the subroutine.                                                 |
| 6   | `3XNN` | `SKIP_EQ`  | Skip the next instruction if value retrieved from V register `X` is equal to `NN` (`VX == NN`).                                                                                          |
| 7   | `4XNN` | `SKIP_NEQ` | Skip the next instruction if value retrieved from V register `X` is not equal to `NN` (`VX != NN`).                                                                                      |
| 8   | `5XY0` | `SKIP_V`   | Skip the next instruction if value retrieved from V registers `X` and `Y` are equal (`VX != VY`).                                                                                        |
| 9   | `6XNN` | `SET`      | Set the value of V register `X` to `NN`.                                                                                                                                                 |
| 10  | `7XNN` | `ADD`      | Adds the value `NN` to the value in V register `X` (wraps on `u8` overflow).                                                                                                             |
| 11  | `8XY0` | `SET_V`    | Set the value of V register `X` to the value stored in V register `Y`.                                                                                                                   |
| 12  | `8XY1` | `OR`       | Set the value of V register `X` to the bitwise OR with the value in V register `Y`.                                                                                                      |
| 13  | `8XY2` | `AND`      | Set the value of V register `X` to the bitwise AND with the value in V register `Y`.                                                                                                     |
| 14  | `8XY3` | `XOR`      | Set the value of V register `X` to the bitwise XOR with the value in V register `Y`.                                                                                                     |
| 15  | `8XY4` | `ADD_V`    | Adds the value in register `X` to the value in register `Y`. Store the value in register `X` and set the carry flag in register `F` based on if the operation underflowed or overflowed. |

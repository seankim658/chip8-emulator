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

| Opcode | Type | Description |
|--------|------|-------------|

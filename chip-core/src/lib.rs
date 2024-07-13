/// Random-access memory (RAM) size.
const RAM_SIZE: usize = 4096;
/// The RAM offset for ROM the available address space.
const START_ADDRESS: u16 = 0x200;
/// Number of general purpose registers.
const NUM_REGS: usize = 16;
/// Size of the stack.
const STACK_SIZE: usize = 16;
/// Number of supported keyboard inputs.
const NUM_KEYS: usize = 16;
/// Display width.
const SCREEN_WIDTH: usize = 64;
/// Display height.
const SCREEN_HEIGHT: usize = 32;

/// Amount of memory taken up by pre-loaded fonts (16
/// supported characters that require 5 bytes each).
const FONTSET_SIZE: usize = 80;
/// Pre-configured font set (all hexadecimal characters, 0-F).
const FONTSET: [u8; FONTSET_SIZE] = [
    // 1111 0000
    // 1001 0000
    // 1001 0000
    // 1001 0000
    // 1111 0000
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    // 0010 0000
    // 0110 0000
    // 0010 0000
    // 0010 0000
    // 0111 0000
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    // 1111 0000
    // 0001 0000
    // 1111 0000
    // 1000 0000
    // 1111 0000
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    // 1111 0000
    // 0001 0000
    // 1111 0000
    // 0001 0000
    // 1111 0000
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    // 1001 0000
    // 1001 0000
    // 1111 0000
    // 0001 0000
    // 0001 0000
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    // 1111 0000
    // 1000 0000
    // 1111 0000
    // 0001 0000
    // 1111 0000
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    // 1111 0000
    // 1000 0000
    // 1111 0000
    // 1001 0000
    // 1111 0000
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    // 1111 0000
    // 0001 0000
    // 0010 0000
    // 0100 0000
    // 0100 0000
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    // 1111 0000
    // 1001 0000
    // 1111 0000
    // 1001 0000
    // 1111 0000
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    // 1111 0000
    // 1001 0000
    // 1111 0000
    // 0001 0000
    // 1111 0000
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    // 1111 0000
    // 1001 0000
    // 1111 0000
    // 1001 0000
    // 1001 0000
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    // 1110 0000
    // 1001 0000
    // 1110 0000
    // 1001 0000
    // 1110 0000
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    // 1111 0000
    // 1000 0000
    // 1000 0000
    // 1000 0000
    // 1111 0000
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    // 1110 0000
    // 1001 0000
    // 1001 0000
    // 1001 0000
    // 1110 0000
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    // 1111 0000
    // 1000 0000
    // 1111 0000
    // 1000 0000
    // 1111 0000
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    // 1111 0000
    // 1000 0000
    // 1111 0000
    // 1000 0000
    // 1000 0000
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Main struct that defines the emulator and current state.
pub struct Emulator {
    /// The 16-bit program counter.
    program_counter: u16,
    /// The 4kb RAM array.
    ram: [u8; RAM_SIZE],
    /// The display screen stored as a row major 1D array.
    /// The screen is monochrome so it will just be stored
    /// as an array of booleans indicating black or white.
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    /// The sixteen 8-bit V registers.
    registers: [u8; NUM_REGS],
    /// The 16-bit index register.
    i_register: u16,
    /// The stack pointer, keeps track of the current top of the stack.
    stack_pointer: u16,
    /// The 16-bit stack.
    stack: [u16; STACK_SIZE],
    /// Boolean array to keep track of the 16 different key presses.
    keys: [bool; NUM_KEYS],
    /// The 8-bit delay timer.
    delay_timer: u8,
    /// The 8-bit sound timer.
    sound_timer: u8,
}

impl Emulator {
    /// Constructor.
    pub fn new() -> Self {
        let mut new_emulator = Self {
            program_counter: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            registers: [0; NUM_REGS],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };

        new_emulator.load_fonts();
        new_emulator
    }

    /// Reset the emulator state.
    pub fn reset(&mut self) {
        *self = Emulator::new();
    }

    /// Defines one CPU loop iteration:
    /// 1. Starts with the `Fetch` step, fetches the value from the ROM
    /// data (which is loaded into RAM) at the memory address stored in
    /// the program counter.
    /// 2. Decode the instruction.
    /// 3. Execute the instruction.
    /// 4. Move the program counter to the next instruction.
    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    /// The two special purpose timers, the delay and sound timers,
    /// tick once per frame rather than once per CPU cycle. As a
    /// result, these neeed a separate ticker function.
    pub fn timer_tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Fetches the current instruction from the memory address that
    /// the program counter is pointing to.
    fn fetch(&mut self) -> u16 {
        // Since each opcode is two bytes and the RAM is stored as an
        // array of single bytes, we first grab the higher byte of the
        // current instruction and then grab the lower byte and combine
        // them in big endian fashion.
        //
        // In Rust, array indices are of type usize so have to cast
        // the program counter from u16 to usize.
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[{ self.program_counter + 1 } as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        opcode
    }

    /// Decodes and performs the opcode instruction.
    ///
    /// #### Parameters:
    /// - opcode: The opcode fetched from the program counter.
    ///
    fn execute(&mut self, opcode: u16) {
        // We need to separate out each hex digit in the 2 byte opcode.
        // We'll do this by bitwise AND'ing to retrieve the relevant
        // bits and then right shifting them by the offset amount.
        let hex_1 = ((opcode & 0xF000) >> 12) as usize;
        let hex_2 = ((opcode & 0x0F00) >> 8) as usize;
        let hex_3 = ((opcode & 0x00F0) >> 4) as usize;
        let hex_4 = (opcode & 0x000F) as usize;

        // Match statement for the opcode decoding and execution.
        match (hex_1, hex_2, hex_3, hex_4) {
            // NOP; do nothing opcode.
            (0, 0, 0, 0) => (),
            // CLS; clear screen opcode.
            (0, 0, 0xE, 0) => {
                // Clear the screen buffer.
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT]
            }
            // RET; return from subroutine.
            (0, 0, 0xE, 0xE) => {
                // Pop the address to return to from the stack.
                let return_address = self.pop();
                // Set the program counter to the return address.
                self.program_counter = return_address;
            }
            // JUMP; jump to memory location at NNN.
            (1, _, _, _) => {
                // Grab the memory address to jump to.
                let nnn = opcode & 0xFFF;
                // Set the program counter to the jump address.
                self.program_counter = nnn;
            }
            // CALL; jump to subroutine at the memory location NNN.
            (2, _, _, _) => {
                // Grab the memory address of the subroutine being called.
                let nnn = opcode & 0xFFF;
                // Push the current program counter onto the stack so it
                // can be popped later when returning from the subroutine.
                self.push(self.program_counter);
                // Set the program counter to the subroutine address.
                self.program_counter = nnn;
            }
            // SKIP_EQ; 3XNN, skip one instruction (2 bytes) if some condition
            // is true. X is the register to retrieve a value from and NN is the
            // raw value to do the VX == NN comparison.
            (3, _, _, _) => {
                // Grab the raw value for the comparison.
                let nn = (opcode & 0xFF) as u8;
                // Grab the register to retrieve from.
                let x = hex_2;
                // Conditional operation.
                if self.registers[x] == nn {
                    self.program_counter += 2;
                }
            }
            // SKIP_NEQ; 4XNN, skip one instruction (2 bytes) if some condition
            // is true. X is the register to retrieve a value from and NN is the
            // raw value to do the VX != NN comparison.
            (4, _, _, _) => {
                // Grab the raw value for the comparison.
                let nn = (opcode & 0xFF) as u8;
                // Grab the register to retrieve from.
                let x = hex_2;
                // Conditional operation.
                if self.registers[x] != nn {
                    self.program_counter += 2;
                }
            }
            // SKIP_V; 5XY0, skip one instruction (2 bytes) if some condition
            // is true. X is the first register to retrieve a value from and Y
            // is the second register to retrieve a value from. The least
            // significant value is not used (opcode requires it be set to 0).
            (5, _, _, 0) => {
                // Grab the first register to retrieve from.
                let x = hex_2;
                // Grab the second register to retrieve from.
                let y = hex_3;
                // Conditional operation.
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }
            // SET; 6XNN, set register VX to the value NN.
            (6, _, _, _) => {
                // Grab the register to set the value for.
                let x = hex_2;
                // Grab the value to set the register.
                let nn = (opcode & 0xFF) as u8;
                self.registers[x] = nn;
            }
            // ADD; 7XNN, add the value NN to the value in register VX.
            (7, _, _, _) => {
                // Grab the register to add to.
                let x = hex_2;
                // Grab the value to add.
                let nn = (opcode & 0xFF) as u8;
                // Note we can't use the regular addition operator here
                // because Rust (in debug mode) will panic in the event
                // of an overflow. Wrapping add wraps around the maximum
                // value on overflow.
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }
            // SET_V; 8XY0, sets the value in register VX to the value in VY.
            (8, _, _, 0) => {
                // Grab the target register.
                let x = hex_2;
                // Grab the source register.
                let y = hex_3;
                // Set VX.
                self.registers[x] = self.registers[y];
            }
            // OR; 8XY1, sets the value in register VX to the result of
            // a bitwise OR with the value in register VY.
            (8, _, _, 1) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Set the value of VX from the bitwise or.
                self.registers[x] |= self.registers[y];
            }
            // AND; 8XY2, sets the value in register VX to the result of
            // a bitwise AND with the value in register VY.
            (8, _, _, 2) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Set the value of VX from the bitwise and.
                self.registers[x] &= self.registers[y];
            }
            // XOR; 8XY3, sets the value in register VX to the result of
            // a bitwise XOR with the value in register VY.
            (8, _, _, 3) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Set the value of VX from the bitwise XOR.
                self.registers[x] ^= self.registers[y];
            }
            // ADD_V; 8XY4, adds the value in register VY to the value
            // in register VX and stores it in register VX.
            (8, _, _, 4) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Add the values together and get the value (which will be a wrapping
                // add if an overflow occurs) and a boolean flag indicating if an
                // overflow occurred.
                let (new_vx, carry) = self.registers[x].overflowing_add(self.registers[y]);
                // Set the carry flag for the VF carry register. Indicates whether the
                // last operation resulted in an overflow or underflow.
                let new_vf = if carry { 1 } else { 0 };
                // Set the new register values.
                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            // SUB_V; 8XY5, subtracts the value in the register VY from the
            // value in register VX and stores it in register VX.
            (8, _, _, 5) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Perform the subtraction and get the value (which will be a wrapping
                // subtract if an underflow occurs) and a boolean flag indicating if
                // an underflow occurred.
                let (new_vx, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                // Set the carry flag for the VF register.
                let new_vf = if borrow { 0 } else { 1 };
                // Set the new register values.
                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            // SING_RSHIFT; 8XY6, performs a single right shift on the value in register VX and
            // store the overflow bit in the flag register.
            (8, _, _, 6) => {
                // Grab the value to shift.
                let x = hex_2;
                // Capture the dropped bit.
                let lsb = self.registers[x] & 1;
                // Shift the value.
                self.registers[x] >>= 1;
                // Set the dropped bit.
                self.registers[0xF] = lsb;
            }
            // SUB_X; 8XY7, subtracts the value in the register VX from the value
            // in register VY and stores it in register VX.
            (8, _, _, 7) => {
                // Grab the first register.
                let x = hex_2;
                // Grab the second register.
                let y = hex_3;
                // Perform the subtraction and get the value (which will be a wrapping
                // subtract if an underflow occurs) and a boolean flag indicating if
                // an underflow occurred.
                let (new_vx, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                // Set the carry flag for the VF register.
                let new_vf = if borrow { 0 } else { 1 };
                // Set the new register values.
                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            }
            // SING_LSHIFT; 8XYE, performs a single left shift on the value in register VX
            // and stores the overflowed value in the VF flag register.
            (8, _, _, 0xE) => {
                // Grab the register.
                let x = hex_2;
                // Grab the overflow bit.
                let msb = (self.registers[x] >> 7) & 1;
                // Single left shift the register value.
                self.registers[x] <<= 1;
                // Set the flag register to the overflow bit.
                self.registers[0xF] = msb;
            }
            // TODO 
            // Rust match statements must be exhaustive, so we need this match
            // to handle unsupported opcodes.
            (_, _, _, _) => unimplemented!("Opcode not supported: {}", opcode),
        }
    }

    /// Load the pre-configured fonts into RAM.
    fn load_fonts(&mut self) {
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    /// Push an address onto the stack.
    ///
    /// #### Parameters:
    /// - val: The address to push onto the stack.
    ///
    fn push(&mut self, val: u16) {
        // At the current top of the stack, add the value.
        self.stack[self.stack_pointer as usize] = val;
        // Increment the stack pointer.
        self.stack_pointer += 1;
    }

    /// Pop an address off of the stack.
    ///
    /// #### Panics
    ///
    /// Panics if pop is called on an empty stack.
    ///
    /// #### Returns:
    /// - The address at the top of the stack.
    ///
    fn pop(&mut self) -> u16 {
        // Decrement the stack pointer.
        self.stack_pointer -= 1;
        // Get the last address pushed into the stack.
        self.stack[self.stack_pointer as usize]
    }
}

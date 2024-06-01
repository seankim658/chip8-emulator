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
        let hex_1 = (opcode & 0xF000) >> 12;
        let hex_2 = (opcode & 0x0F00) >> 8;
        let hex_3 = (opcode & 0x00F0) >> 4;
        let hex_4 = opcode & 0x000F;
        // TODO 
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

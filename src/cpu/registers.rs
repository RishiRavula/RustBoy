use bitflags::bitflags;


/*
    F / Flags Register
    Used to mark Z(ero) Flag - bit 7
    Subtractio(n) Flag - bit 6
    Half-(C)arry Flag - bit 5
    (C)arry Flag - bit 4
 */
bitflags! {
    pub struct Flags: u8 {
        const Z = 0b1000_0000;
        const N = 0b0100_0000;
        const H = 0b0010_0000;
        const C = 0b0001_0000;
    }
}

pub struct Registers {
    pub a: u8, //Accumulator, Most operations occur on this register
    pub f: Flags, //Flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16, // Stack Pointer
    pub pc: u16, // Program Counter
}


impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE, // Initial stack pointer value
            pc: 0x0100, // Initial program counter value
        }
    }

    /*
        16 bit registers are simply AF, BC, DE, HL with a high/low reg
        We bit shift the high register by 8 bits and add the low register to it
        upcasting fills the most significant bits with 0, downcasting omits the top bits
     */

    //AF Register Getter & Setters
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f.bits() as u16)
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from_bits_truncate(value as u8);
    }

    //BC Register Getter & Setters
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    //DE Register Getter & Setters
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    //HL Register Getter & Setters
    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}
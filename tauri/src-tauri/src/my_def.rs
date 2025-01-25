pub mod constants {
    pub const NOP: i8 = 0;
    pub const ADD: i8 = 1;
    pub const SUBT: i8 = 2;
    pub const NEG: i8 = 3;

    pub const NICHT: i8 = 4;
    pub const UND: i8 = 5;
    pub const ODER: i8 = 6;
    pub const BEQ: i8 = 7;
    pub const BNEQ: i8 = 8;
    pub const BLT: i8 = 9;
    pub const JMP: i8 = 10;
    pub const LDW: i8 = 11;
    pub const STW: i8 = 12;
    pub const LDI: i8 = 13;
    pub const MUL: i8 = 14;
    pub const DIV: i8 = 15;
    pub const MODU: i8 = 16;

    pub const HALT: i8 = 0b011110;
}

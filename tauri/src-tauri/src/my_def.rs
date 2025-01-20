pub mod constants {
    pub const ADD: i8 = 0;
    pub const SUBT: i8 = 1;
    pub const NEG: i8 = 2;

    pub const NICHT: i8 = 3;
    pub const UND: i8 = 4;
    pub const ODER: i8 = 5;
    pub const BEQ: i8 = 6;
    pub const BNEQ: i8 = 7;
    pub const BLT: i8 = 8;
    pub const JMP: i8 = 9;
    pub const LDW: i8 = 10;
    pub const STW: i8 = 11;
    pub const MUL: i8 = 12;
    pub const DIV: i8 = 13;
    pub const MODU: i8 = 14;
    pub const MOVI: i8 = 15;

    pub const HALT: i8 = 0b011110;
    pub const NOOP: i8 = 0b011111;
}

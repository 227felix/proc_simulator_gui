pub mod proc {
    use serde::Serialize;

    use crate::my_def::constants::*;

    use std::{
        io::BufRead,
        path::{Path, PathBuf},
    };

    #[derive(Clone, Serialize)]
    struct FetchPhase {
        pc: i32,
        ir: i32,
    }

    impl FetchPhase {
        fn rising_edge(&mut self, rom: &Vec<i32>, pc: i32) {
            println!("FetchPhase rising edge");
            self.ir = rom[self.pc as usize];
            self.pc = pc + 1;
            println!("IR: {:032b}", self.ir);
        }

        fn falling_edge(&self) {
            println!("FetchPhase falling edge");
        }

        fn get_ir(&self) -> i32 {
            self.ir
        }

        fn get_pc(&self) -> i32 {
            self.pc
        }
    }

    #[derive(Clone, Serialize)]
    struct DecodePhase {
        reg_bank: Vec<i32>,
        ir: i32,
        opcode: i8,
        r1: i8,
        r2: i8,
        imm: i16,
        long_imm: i32,
        r3: i8,
        r4: i8,
        r5: i8,
        a: i32,
        b: i32,
        wb: i32,
        wb_addr: i32,
        write_en: bool,
        pc: i32,
    }

    impl DecodePhase {
        fn rising_edge(&mut self, ir: i32, wb: i32, wb_addr: i32, write_en: bool, pc: i32) {
            println!("DecodePhase rising edge");

            self.ir = ir;

            self.opcode = (self.ir >> 26) as i8;
            self.r1 = ((self.ir >> 21) & 0b11111) as i8;
            self.r2 = ((self.ir >> 16) & 0b11111) as i8;
            self.imm = (self.ir & 0xFFFF) as i16;
            self.long_imm = self.ir & 0x3FFFFFF;
            self.r3 = ((self.ir >> 11) & 0b11111) as i8;
            self.r4 = ((self.ir >> 6) & 0b11111) as i8;
            self.r5 = (self.ir & 0b11111) as i8;
            self.a = self.reg_bank[self.r1 as usize];
            self.b = self.reg_bank[self.r2 as usize];
            self.wb = wb;
            self.wb_addr = wb_addr;
            self.write_en = write_en;
            self.pc = pc;
            if write_en {
                self.reg_bank[wb_addr as usize] = wb;
            }
            self.pretty_print();
        }

        fn falling_edge(&self) {
            println!("DecodePhase falling edge");
        }

        fn pretty_print(&self) {
            println!("==================== Decode Phase ====================");
            println!("IR:          {:032b}", self.ir);
            println!("Opcode:      {:08b}", self.opcode);
            println!("R1:          {:05b}", self.r1);
            println!("R2:          {:05b}", self.r2);
            println!("Imm:         {:016b}", self.imm);
            println!("Long Imm:    {:032b}", self.long_imm);
            println!("R3:          {:05b}", self.r3);
            println!("R4:          {:05b}", self.r4);
            println!("R5:          {:05b}", self.r5);
            println!("A:           {:032b}", self.a);
            println!("B:           {:032b}", self.b);
            println!("WB:          {:032b}", self.wb);
            println!("WB Addr:     {:032b}", self.wb_addr);
            println!("Write Enable:{:01b}", self.write_en as i8);
            println!("PC:          {:032b}", self.pc);
            println!("------------------------------------------------------");
            println!("Register Bank:");
            for (i, reg) in self.reg_bank.iter().enumerate() {
                println!("R{:02}: {:032b}", i, reg);
            }
            println!("======================================================");
        }

        fn get_opcode(&self) -> i8 {
            self.opcode
        }

        fn get_r1(&self) -> i8 {
            self.r1
        }

        fn get_r2(&self) -> i8 {
            self.r2
        }

        fn get_imm(&self) -> i16 {
            self.imm
        }

        fn get_long_imm(&self) -> i32 {
            self.long_imm
        }

        fn get_r3(&self) -> i8 {
            self.r3
        }

        fn get_r4(&self) -> i8 {
            self.r4
        }

        fn get_r5(&self) -> i8 {
            self.r5
        }

        fn get_a(&self) -> i32 {
            self.a
        }

        fn get_b(&self) -> i32 {
            self.b
        }

        fn get_pc(&self) -> i32 {
            self.pc
        }
    }

    #[derive(Clone, Serialize)]
    struct ExecutePhase {
        opcode: i8,
        r1: i8,
        r2: i8,
        imm: i16,
        long_imm: i32,
        r3: i8,
        r4: i8,
        r5: i8,
        a: i32,
        b: i32,
        pc: i32,
        br_flag: bool,
    }

    impl ExecutePhase {
        fn rising_edge(
            &mut self,
            opcode: i8,
            r1: i8,
            r2: i8,
            imm: i16,
            long_imm: i32,
            r3: i8,
            r4: i8,
            r5: i8,
            a: i32,
            b: i32,
            pc: i32,
        ) {
            println!("ExecutePhase rising edge");

            self.opcode = opcode;
            self.r1 = r1;
            self.r2 = r2;
            self.imm = imm;
            self.long_imm = long_imm;
            self.r3 = r3;
            self.r4 = r4;
            self.r5 = r5;
            self.a = a;
            self.pc = pc;

            // do the alu_op
            let (alu_out, br_flag) = Alu::op(opcode, a, b, imm, pc);
            self.br_flag = br_flag;
            if opcode < 21 {
                //TODO fix this
                self.b = alu_out;
            } else {
                self.b = b;
            }

            self.pretty_print();
        }

        fn falling_edge(&self) {
            println!("ExecutePhase falling edge");
        }

        fn pretty_print(&self) {
            println!("==================== Execute Phase ====================");
            println!("Opcode:      {:08b}", self.opcode);
            println!("R1:          {:05b}", self.r1);
            println!("R2:          {:05b}", self.r2);
            println!("Imm:         {:016b}", self.imm);
            println!("Long Imm:    {:032b}", self.long_imm);
            println!("R3:          {:05b}", self.r3);
            println!("R4:          {:05b}", self.r4);
            println!("R5:          {:05b}", self.r5);
            println!("A:           {:032b}", self.a);
            println!("B:           {:032b}", self.b);
            println!("PC:          {:032b}", self.pc);
            println!("Branch Flag: {:01b}", self.br_flag as i8);
            println!("======================================================");
        }

        fn get_opcode(&self) -> i8 {
            self.opcode
        }

        fn get_r1(&self) -> i8 {
            self.r1
        }

        fn get_imm(&self) -> i16 {
            self.imm
        }

        fn get_long_imm(&self) -> i32 {
            self.long_imm
        }

        fn get_a(&self) -> i32 {
            self.a
        }

        fn get_b(&self) -> i32 {
            self.b
        }

        fn get_pc(&self) -> i32 {
            self.pc
        }

        fn get_br_flag(&self) -> bool {
            self.br_flag
        }
    }

    #[derive(Clone, Serialize)]
    struct MemoryPhase {
        opcode: i8,
        r1: i8,
        imm: i16,
        long_imm: i32,
        data: i32,
        addr: i32,
        nwe: bool,
        data_out: i32,
        pc: i32,
        br_flag: bool,
    }

    impl MemoryPhase {
        fn rising_edge(
            &mut self,
            opcode: i8,
            r1: i8,
            imm: i16,
            long_imm: i32,
            data: i32,
            addr: i32,
            pc: i32,
            br_flag: bool,
            ram: &mut Vec<i32>,
        ) {
            println!("MemoryPhase rising edge");

            self.opcode = opcode;
            self.r1 = r1;
            self.imm = imm;
            self.long_imm = long_imm;
            self.data = data;
            self.addr = addr;
            self.pc = pc;
            self.br_flag = br_flag;

            if self.opcode == STW {
                self.nwe = false;
            } else {
                self.nwe = true;
            }
            if self.nwe {
                self.data_out = ram[self.addr as usize]; // FIXME: eventuell sollte man den RAM als eigene Struktur haben um die Verzögerung korrekt darzustellen
            } else {
                ram[self.addr as usize] = self.data;
            }

            if self.opcode == LDW {
                self.data_out = ram[self.addr as usize];
            } else {
                self.data_out = data;
            }

            if self.opcode == JMP {
                self.pc = self.long_imm;
            }

            if self.br_flag {
                self.pc += self.imm as i32;
            }

            self.pretty_print();
        }

        fn falling_edge(&self) {
            println!("MemoryPhase falling edge");
        }

        fn pretty_print(&self) {
            println!("==================== Memory Phase =====================");
            println!("Opcode:      {:08b}", self.opcode);
            println!("R1:          {:05b}", self.r1);
            println!("Imm:         {:016b}", self.imm);
            println!("Long Imm:    {:032b}", self.long_imm);
            println!("Data:        {:032b}", self.data);
            println!("Addr:        {:032b}", self.addr);
            println!("NWE:         {:01b}", self.nwe as i8);
            println!("Data Out:    {:032b}", self.data_out);
            println!("PC:          {:032b}", self.pc);
            println!("Branch Flag: {:01b}", self.br_flag as i8);
            println!("======================================================");
        }

        fn get_pc(&self) -> i32 {
            self.pc
        }

        fn get_opcode(&self) -> i8 {
            self.opcode
        }

        fn get_r1(&self) -> i8 {
            self.r1
        }

        fn get_data_out(&self) -> i32 {
            self.data_out
        }
    }

    #[derive(Clone, Serialize)]
    struct WriteBackPhase {
        opcode: i8,
        r1: i8,
        data: i32,
        write_en: bool,
    }

    impl WriteBackPhase {
        fn rising_edge(&mut self, opcode: i8, r1: i8, data: i32) {
            println!("WriteBackPhase rising edge");

            self.opcode = opcode;
            self.r1 = r1;
            self.data = data;
            if opcode == LDW || opcode < 6 {
                self.write_en = true;
            } else {
                self.write_en = false;
            }

            self.pretty_print();
        }

        fn falling_edge(&self) {
            println!("WriteBackPhase falling edge");
        }

        fn pretty_print(&self) {
            println!("==================== WriteBack Phase ==================");
            println!("Opcode:      {:08b}", self.opcode);
            println!("R1:          {:05b}", self.r1);
            println!("Data:        {:032b}", self.data);
            println!("Write Enable:{:01b}", self.write_en as i8);
            println!("======================================================");
        }

        fn get_wb(&self) -> i32 {
            self.data
        }

        fn get_wb_addr(&self) -> i32 {
            self.r1 as i32
        }

        fn get_write_en(&self) -> bool {
            self.write_en
        }
    }

    #[derive(Serialize)]
    pub struct Processor {
        rom: Vec<i32>,
        ram: Vec<i32>,
        fetch: FetchPhase,
        decode: DecodePhase,
        execute: ExecutePhase,
        memory: MemoryPhase,
        write_back: WriteBackPhase,
    }

    impl Processor {
        pub fn clock(&mut self) {
            // run the rising edge
            let fetch_clone = self.fetch.clone();
            let decode_clone = self.decode.clone();
            let execute_clone = self.execute.clone();
            let memory_clone = self.memory.clone();

            let mut pc = self.memory.get_pc();
            self.fetch.rising_edge(&self.rom, pc);

            self.decode.rising_edge(
                fetch_clone.get_ir(),
                self.write_back.get_wb(),
                self.write_back.get_wb_addr(),
                self.write_back.get_write_en(),
                fetch_clone.get_pc(),
            );

            self.execute.rising_edge(
                decode_clone.get_opcode(),
                decode_clone.get_r1(),
                decode_clone.get_r2(),
                decode_clone.get_imm(),
                decode_clone.get_long_imm(),
                decode_clone.get_r3(),
                decode_clone.get_r4(),
                decode_clone.get_r5(),
                decode_clone.get_a(),
                decode_clone.get_b(),
                decode_clone.get_pc(),
            );

            self.memory.rising_edge(
                execute_clone.get_opcode(),
                execute_clone.get_r1(),
                execute_clone.get_imm(),
                execute_clone.get_long_imm(),
                execute_clone.get_a(),
                execute_clone.get_b(),
                execute_clone.get_pc(),
                execute_clone.get_br_flag(),
                &mut self.ram,
            );

            self.write_back.rising_edge(
                memory_clone.get_opcode(),
                memory_clone.get_r1(),
                memory_clone.get_data_out(),
            );

            // run the falling edge
            self.fetch.falling_edge();
            self.decode.falling_edge();
            self.execute.falling_edge();
            self.memory.falling_edge();
            self.write_back.falling_edge();
        }

        pub fn new(path: PathBuf) -> Processor {
            let rom = DataReader::read_rom_from_file(&path, 1024);
            let ram = vec![0; 1024];
            let fetch = FetchPhase { pc: 3, ir: 0 };
            let decode = DecodePhase {
                reg_bank: vec![0; 32],
                ir: 0,
                opcode: 0,
                r1: 0,
                r2: 0,
                imm: 0,
                long_imm: 0,
                r3: 0,
                r4: 0,
                r5: 0,
                a: 0,
                b: 0,
                wb: 0,
                wb_addr: 0,
                write_en: false,
                pc: 0,
            };
            let execute = ExecutePhase {
                opcode: 0,
                r1: 0,
                r2: 0,
                imm: 0,
                long_imm: 0,
                r3: 0,
                r4: 0,
                r5: 0,
                a: 0,
                b: 0,
                pc: 0,
                br_flag: false,
            };
            let memory = MemoryPhase {
                opcode: 0,
                r1: 0,
                imm: 0,
                long_imm: 0,
                data: 0,
                addr: 0,
                nwe: false,
                data_out: 0,
                pc: 0,
                br_flag: false,
            };
            let write_back = WriteBackPhase {
                opcode: 0,
                r1: 0,
                data: 0,
                write_en: false,
            };
            Processor {
                rom,
                ram,
                fetch,
                decode,
                execute,
                memory,
                write_back,
            }
        }

        pub fn get_state_serialized(&self) -> String {
            serde_json::to_string(&self).unwrap()
        }
    }

    struct Alu;

    // implement the static methods for the Alu struct
    impl Alu {
        // TODO: Fix this
        fn op(opcode: i8, a: i32, b: i32, imm: i16, pc: i32) -> (i32, bool) {
            match opcode {
                ADD => (a + b, false),
                SUBT => (a - b, false),
                NEG => (-a, false),
                NICHT => (!a, false),
                UND => (a ^ b, false),
                ODER => (a | b, false),
                BEQ => {
                    if a == b {
                        (0, true)
                    } else {
                        (0, false)
                    }
                }
                BNEQ => {
                    if a != b {
                        (0, true)
                    } else {
                        (0, false)
                    }
                }
                BLT => {
                    if a < b {
                        (0, true)
                    } else {
                        (0, false)
                    }
                }
                _ => (0, false),
            }
        }
    }

    struct DataReader {}

    impl DataReader {
        fn read_rom_from_file(file_path: &Path, rom_size: u32) -> Vec<i32> {
            let mut rom = vec![0; rom_size as usize];
            let file = std::fs::File::open(file_path);
            if file.is_err() {
                panic!("File not found");
            }
            let file = file.unwrap();
            let reader = std::io::BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                let value = i32::from_str_radix(&line, 16).unwrap();
                rom[i] = value;
                println!("{:032b}", value);
            }
            rom
        }
    }
}

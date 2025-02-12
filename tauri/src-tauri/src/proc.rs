pub mod proc {
    use serde::Serialize;

    use crate::{load_program, my_def::constants::*};

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
        fn rising_edge(&mut self, rom: &Vec<i32>, pc: i32, pc_alt: i32, branch_flag: bool) {
            self.ir = rom[self.pc as usize];
            // FIXME: fix halt logic
            let opcode = (self.ir >> 26) as i8;
            if !(opcode == HALT) {
                self.pc = pc + 1;
            }
            if branch_flag {
                self.pc = pc_alt;
            }
        }

        fn falling_edge(&self) {}

        fn get_ir(&self) -> i32 {
            self.ir
        }

        fn get_pc(&self) -> i32 {
            self.pc
        }
    }

    #[derive(Clone, Serialize)]
    struct DecodePhase {
        opcode: i8,
        r1: i8,
        r2: i8,
        r3: i8,
        r4: i8,
        r5: i8,
        imm: i16,
        long_imm: i32,
        ir: i32,
        reg_bank: Vec<i32>,
        a: i32,
        b: i32,
        wb: i32,
        wb_addr: i32,
        write_en: bool,
        pc: i32,
    }

    impl DecodePhase {
        fn rising_edge(&mut self, ir: i32, wb: i32, wb_addr: i32, write_en: bool, pc: i32) {
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
        }

        fn falling_edge(&self) {}

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
        r3: i8,
        r4: i8,
        r5: i8,
        imm: i16,
        long_imm: i32,
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
            self.opcode = opcode;
            self.r1 = r2; //FIXME
            self.r2 = r2;
            self.imm = imm;
            self.long_imm = long_imm;
            self.r3 = r3;
            self.r4 = r4;
            self.r5 = r5;
            self.b = b;
            self.pc = pc;

            // do the alu_op
            let (alu_out, br_flag) = Alu::op(opcode, a, b, imm, pc);
            self.br_flag = br_flag;
            if opcode < BEQ {
                //TODO fix this
                self.a = alu_out;
                // r3 ist die ziel register adresse für alu op
                self.r1 = r3; // r3 ist das ziel register
            } else {
                self.r1 = r2;
                self.a = a;
            }
            if opcode == LDI {
                self.a = imm as i32;
            }
        }

        fn falling_edge(&self) {}

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

    #[derive(Clone, Serialize, Debug)]
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
            changed_ramfield: &mut (i32, i32),
        ) {
            let addr = addr % 65536;
            // convert the addr to a usize by converting it to bin and interpreting the last 16 bits as an unsigned intS
            self.opcode = opcode;
            self.r1 = r1;
            self.imm = imm;
            self.long_imm = long_imm;
            self.data = data;
            self.addr = addr % 65536;
            self.pc = pc;
            self.br_flag = br_flag;

            if self.opcode == STW {
                self.nwe = false;
            } else {
                self.nwe = true;
            }
            if self.nwe {
                if self.addr as usize >= ram.len() {
                    // FIXME
                    self.addr = 0;
                }
                self.data_out = ram[self.addr as usize]; // FIXME: eventuell sollte man den RAM als eigene Struktur haben um die Verzögerung korrekt darzustellen
                *changed_ramfield = (-1, -1);
            } else {
                println!("Addr: {}", addr);
                let addr_bin = format!("{:032b}", addr);
                println!("Addr bin: {}", addr_bin);
                let addr_test = usize::from_str_radix(&addr_bin[16..], 2).unwrap();
                println!("Addr: {}", addr_test);
                ram[addr_test as usize] = self.data;
                *changed_ramfield = (addr_test as i32, self.data);
            }

            if self.opcode == LDW {
                self.data_out = ram[self.addr as usize];
            } else if self.opcode == LDI {
                self.data_out = self.data as i32;
            } else {
                self.data_out = data;
            }

            if self.br_flag {
                self.pc += self.imm as i32;
            }
            if self.opcode == JMP {
                self.pc = self.long_imm;
                self.br_flag = true;
            }
        }

        fn falling_edge(&self) {}

        fn get_pc(&self) -> i32 {
            self.pc
        }

        fn get_opcode(&self) -> i8 {
            self.opcode
        }

        fn get_r1(&self) -> i8 {
            self.r1
        }

        fn get_r2(&self) -> i8 {
            self.r1
        }

        fn get_data_out(&self) -> i32 {
            self.data_out
        }

        fn get_br_flag(&self) -> bool {
            self.br_flag
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
            self.opcode = opcode;
            self.r1 = r1;
            self.data = data;
            println!("opcode: {}", opcode);
            if opcode == LDW || opcode == LDI || (opcode < BEQ && opcode > NOP) || opcode == MOV {
                self.write_en = true;
            } else {
                self.write_en = false;
            }
        }

        fn falling_edge(&self) {}

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
        changed_ramfield: (i32, i32),
        fetch: FetchPhase,
        decode: DecodePhase,
        execute: ExecutePhase,
        memory: MemoryPhase,
        write_back: WriteBackPhase,
        num_representation: String,
        file_path: PathBuf,
    }

    impl Processor {
        pub fn reload_program(&mut self) -> Processor {
            self.load_program(self.file_path.clone())
        }

        pub fn load_program(&self, path: PathBuf) -> Processor {
            let rom = DataReader::read_rom_from_file(&path, 65536);
            let mut new_proc = Processor::new_with_rom(rom, &path);

            new_proc.set_num_rep(self.num_representation.clone());
            new_proc
        }
        pub fn reset(&self) -> Processor {
            //
            let rom = self.rom.clone();
            let rom_path = self.file_path.clone();

            let mut new_proc = Processor::new_with_rom(rom, &rom_path);
            new_proc.set_num_rep(self.num_representation.clone());
            new_proc
        }

        fn new_with_rom(rom: Vec<i32>, rom_path: &PathBuf) -> Processor {
            let ram = vec![0; 65536];
            let fetch = FetchPhase { pc: 0, ir: 0 };
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
                changed_ramfield: (0, 0),
                num_representation: "hex".to_string(),
                file_path: rom_path.to_path_buf(),
            }
        }
        pub fn clock(&mut self) {
            // run the rising edge
            let fetch_clone = self.fetch.clone();
            let decode_clone = self.decode.clone();
            let execute_clone = self.execute.clone();
            let memory_clone = self.memory.clone();

            let new_pc = fetch_clone.get_pc();
            let new_pc_alt = memory_clone.get_pc();
            let branch_flag = memory_clone.get_br_flag();

            self.fetch
                .rising_edge(&self.rom, new_pc, new_pc_alt, branch_flag);

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
                &mut self.changed_ramfield,
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

        pub fn new_empty_rom() -> Processor {
            let rom = vec![0; 65536];
            let ram = vec![0; 65536];
            let fetch = FetchPhase { pc: 0, ir: 0 };
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
                changed_ramfield: (0, 0),
                num_representation: "hex".to_string(),
                file_path: PathBuf::new(),
            }
        }

        pub fn new(path: PathBuf, num_representation: String) -> Processor {
            let rom = DataReader::read_rom_from_file(&path, 65536);
            let ram = vec![0; 65536];
            let fetch = FetchPhase { pc: 0, ir: 0 };
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
                changed_ramfield: (0, 0),
                num_representation,
                file_path: path,
            }
        }

        pub fn set_num_rep(&mut self, representation: String) {
            self.num_representation = representation;
        }

        pub fn get_rom_path(&self) -> &PathBuf {
            &self.file_path
        }

        pub fn get_state_serialized(&self) -> String {
            let serialized = match self.num_representation.as_str() {
                "hex" => serde_json::to_string(&self.to_hex()).unwrap(),
                "dec" => serde_json::to_string(&self.to_dec()).unwrap(),
                "bin" => serde_json::to_string(&self.to_bin()).unwrap(),
                _ => serde_json::to_string(&self).unwrap(),
            };
            serialized
        }

        fn to_hex(&self) -> ProcessorHex {
            ProcessorHex {
                rom: self.rom.iter().map(|x| format!("{:08x}", x)).collect(),
                ram: self.ram.iter().map(|x| format!("{:08x}", x)).collect(),
                fetch: FetchPhaseHex {
                    pc: format!("{:08x}", self.fetch.pc),
                    ir: format!("{:08x}", self.fetch.ir),
                },
                decode: DecodePhaseHex {
                    opcode: format!("{:02x}", self.decode.opcode),
                    r1: format!("{:02x}", self.decode.r1),
                    r2: format!("{:02x}", self.decode.r2),
                    r3: format!("{:02x}", self.decode.r3),
                    r4: format!("{:02x}", self.decode.r4),
                    r5: format!("{:02x}", self.decode.r5),
                    imm: format!("{:04x}", self.decode.imm),
                    long_imm: format!("{:08x}", self.decode.long_imm),
                    ir: format!("{:08x}", self.decode.ir),
                    reg_bank: self
                        .decode
                        .reg_bank
                        .iter()
                        .map(|x| format!("{:08x}", x))
                        .collect(),
                    a: format!("{:08x}", self.decode.a),
                    b: format!("{:08x}", self.decode.b),
                    wb: format!("{:08x}", self.decode.wb),
                    wb_addr: format!("{:08x}", self.decode.wb_addr),
                    write_en: format!("{:01x}", self.decode.write_en as i8),
                    pc: format!("{:08x}", self.decode.pc),
                },
                execute: ExecutePhaseHex {
                    opcode: format!("{:02x}", self.execute.opcode),
                    r1: format!("{:02x}", self.execute.r1),
                    r2: format!("{:02x}", self.execute.r2),
                    r3: format!("{:02x}", self.execute.r3),
                    r4: format!("{:02x}", self.execute.r4),
                    r5: format!("{:02x}", self.execute.r5),
                    imm: format!("{:04x}", self.execute.imm),
                    long_imm: format!("{:08x}", self.execute.long_imm),
                    a: format!("{:08x}", self.execute.a),
                    b: format!("{:08x}", self.execute.b),
                    pc: format!("{:08x}", self.execute.pc),
                    br_flag: format!("{:01x}", self.execute.br_flag as i8),
                },
                memory: MemoryPhaseHex {
                    opcode: format!("{:02x}", self.memory.opcode),
                    r1: format!("{:02x}", self.memory.r1),
                    imm: format!("{:04x}", self.memory.imm),
                    long_imm: format!("{:08x}", self.memory.long_imm),
                    data: format!("{:08x}", self.memory.data),
                    addr: format!("{:08x}", self.memory.addr),
                    nwe: format!("{:01x}", self.memory.nwe as i8),
                    data_out: format!("{:08x}", self.memory.data_out),
                    pc: format!("{:08x}", self.memory.pc),
                    br_flag: format!("{:01x}", self.memory.br_flag as i8),
                },
                write_back: WriteBackPhaseHex {
                    opcode: format!("{:02x}", self.write_back.opcode),
                    r1: format!("{:02x}", self.write_back.r1),
                    data: format!("{:08x}", self.write_back.data),
                    write_en: format!("{:01x}", self.write_back.write_en as i8),
                },
                num_representation: self.num_representation.clone(),
                file_path: self.file_path.to_string_lossy().to_string(),
                changed_ramfield: (
                    format!("{:08x}", self.changed_ramfield.0),
                    format!("{:08x}", self.changed_ramfield.1),
                ),
            }
        }

        fn to_dec(&self) -> ProcessorDec {
            ProcessorDec {
                rom: self.rom.iter().map(|x| format!("{:010}", x)).collect(),
                ram: self.ram.iter().map(|x| format!("{:010}", x)).collect(),
                fetch: FetchPhaseDec {
                    pc: format!("{:010}", self.fetch.pc),
                    ir: format!("{:010}", self.fetch.ir),
                },
                decode: DecodePhaseDec {
                    opcode: format!("{:03}", self.decode.opcode),
                    r1: format!("{:03}", self.decode.r1),
                    r2: format!("{:03}", self.decode.r2),
                    r3: format!("{:03}", self.decode.r3),
                    r4: format!("{:03}", self.decode.r4),
                    r5: format!("{:03}", self.decode.r5),
                    imm: format!("{:05}", self.decode.imm),
                    long_imm: format!("{:010}", self.decode.long_imm),
                    ir: format!("{:010}", self.decode.ir),
                    reg_bank: self
                        .decode
                        .reg_bank
                        .iter()
                        .map(|x| format!("{:010}", x))
                        .collect(),
                    a: format!("{:010}", self.decode.a),
                    b: format!("{:010}", self.decode.b),
                    wb: format!("{:010}", self.decode.wb),
                    wb_addr: format!("{:010}", self.decode.wb_addr),
                    write_en: format!("{:01}", self.decode.write_en as i8),
                    pc: format!("{:010}", self.decode.pc),
                },
                execute: ExecutePhaseDec {
                    opcode: format!("{:03}", self.execute.opcode),
                    r1: format!("{:03}", self.execute.r1),
                    r2: format!("{:03}", self.execute.r2),
                    r3: format!("{:03}", self.execute.r3),
                    r4: format!("{:03}", self.execute.r4),
                    r5: format!("{:03}", self.execute.r5),
                    imm: format!("{:05}", self.execute.imm),
                    long_imm: format!("{:010}", self.execute.long_imm),
                    a: format!("{:010}", self.execute.a),
                    b: format!("{:010}", self.execute.b),
                    pc: format!("{:010}", self.execute.pc),
                    br_flag: format!("{:01}", self.execute.br_flag as i8),
                },
                memory: MemoryPhaseDec {
                    opcode: format!("{:03}", self.memory.opcode),
                    r1: format!("{:03}", self.memory.r1),
                    imm: format!("{:05}", self.memory.imm),
                    long_imm: format!("{:010}", self.memory.long_imm),
                    data: format!("{:010}", self.memory.data),
                    addr: format!("{:010}", self.memory.addr),
                    nwe: format!("{:01}", self.memory.nwe as i8),
                    data_out: format!("{:010}", self.memory.data_out),
                    pc: format!("{:010}", self.memory.pc),
                    br_flag: format!("{:01}", self.memory.br_flag as i8),
                },
                write_back: WriteBackPhaseDec {
                    opcode: format!("{:03}", self.write_back.opcode),
                    r1: format!("{:03}", self.write_back.r1),
                    data: format!("{:010}", self.write_back.data),
                    write_en: format!("{:01}", self.write_back.write_en as i8),
                },
                num_representation: self.num_representation.clone(),
                file_path: self.file_path.to_string_lossy().to_string(),
                changed_ramfield: (
                    format!("{:010}", self.changed_ramfield.0),
                    format!("{:010}", self.changed_ramfield.1),
                ),
            }
        }

        fn to_bin(&self) -> ProcessorBin {
            ProcessorBin {
                rom: self.rom.iter().map(|x| format!("{:032b}", x)).collect(),
                ram: self.ram.iter().map(|x| format!("{:032b}", x)).collect(),
                fetch: FetchPhaseBin {
                    pc: format!("{:032b}", self.fetch.pc),
                    ir: format!("{:032b}", self.fetch.ir),
                },
                decode: DecodePhaseBin {
                    opcode: format!("{:08b}", self.decode.opcode),
                    r1: format!("{:08b}", self.decode.r1),
                    r2: format!("{:08b}", self.decode.r2),
                    r3: format!("{:08b}", self.decode.r3),
                    r4: format!("{:08b}", self.decode.r4),
                    r5: format!("{:08b}", self.decode.r5),
                    imm: format!("{:016b}", self.decode.imm),
                    long_imm: format!("{:032b}", self.decode.long_imm),
                    ir: format!("{:032b}", self.decode.ir),
                    reg_bank: self
                        .decode
                        .reg_bank
                        .iter()
                        .map(|x| format!("{:032b}", x))
                        .collect(),
                    a: format!("{:032b}", self.decode.a),
                    b: format!("{:032b}", self.decode.b),
                    wb: format!("{:032b}", self.decode.wb),
                    wb_addr: format!("{:032b}", self.decode.wb_addr),
                    write_en: format!("{:01b}", self.decode.write_en as i8),
                    pc: format!("{:032b}", self.decode.pc),
                },
                execute: ExecutePhaseBin {
                    opcode: format!("{:08b}", self.execute.opcode),
                    r1: format!("{:08b}", self.execute.r1),
                    r2: format!("{:08b}", self.execute.r2),
                    r3: format!("{:08b}", self.execute.r3),
                    r4: format!("{:08b}", self.execute.r4),
                    r5: format!("{:08b}", self.execute.r5),
                    imm: format!("{:016b}", self.execute.imm),
                    long_imm: format!("{:032b}", self.execute.long_imm),
                    a: format!("{:032b}", self.execute.a),
                    b: format!("{:032b}", self.execute.b),
                    pc: format!("{:032b}", self.execute.pc),
                    br_flag: format!("{:01b}", self.execute.br_flag as i8),
                },
                memory: MemoryPhaseBin {
                    opcode: format!("{:08b}", self.memory.opcode),
                    r1: format!("{:08b}", self.memory.r1),
                    imm: format!("{:016b}", self.memory.imm),
                    long_imm: format!("{:032b}", self.memory.long_imm),
                    data: format!("{:032b}", self.memory.data),
                    addr: format!("{:032b}", self.memory.addr),
                    nwe: format!("{:01b}", self.memory.nwe as i8),
                    data_out: format!("{:032b}", self.memory.data_out),
                    pc: format!("{:032b}", self.memory.pc),
                    br_flag: format!("{:01b}", self.memory.br_flag as i8),
                },
                write_back: WriteBackPhaseBin {
                    opcode: format!("{:08b}", self.write_back.opcode),
                    r1: format!("{:08b}", self.write_back.r1),
                    data: format!("{:032b}", self.write_back.data),
                    write_en: format!("{:01b}", self.write_back.write_en as i8),
                },
                num_representation: self.num_representation.clone(),
                file_path: self.file_path.to_string_lossy().to_string(),
                changed_ramfield: (
                    format!("{:032b}", self.changed_ramfield.0),
                    format!("{:032b}", self.changed_ramfield.1),
                ),
            }
        }
    }

    #[derive(Serialize)]
    struct ProcessorHex {
        rom: Vec<String>,
        ram: Vec<String>,
        fetch: FetchPhaseHex,
        decode: DecodePhaseHex,
        execute: ExecutePhaseHex,
        memory: MemoryPhaseHex,
        write_back: WriteBackPhaseHex,
        num_representation: String,
        file_path: String,
        changed_ramfield: (String, String),
    }

    #[derive(Serialize)]
    struct FetchPhaseHex {
        pc: String,
        ir: String,
    }

    #[derive(Serialize)]
    struct DecodePhaseHex {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        ir: String,
        reg_bank: Vec<String>,
        a: String,
        b: String,
        wb: String,
        wb_addr: String,
        write_en: String,
        pc: String,
    }

    #[derive(Serialize)]
    struct ExecutePhaseHex {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        a: String,
        b: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct MemoryPhaseHex {
        opcode: String,
        r1: String,
        imm: String,
        long_imm: String,
        data: String,
        addr: String,
        nwe: String,
        data_out: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct WriteBackPhaseHex {
        opcode: String,
        r1: String,
        data: String,
        write_en: String,
    }

    #[derive(Serialize)]
    struct ProcessorDec {
        rom: Vec<String>,
        ram: Vec<String>,
        fetch: FetchPhaseDec,
        decode: DecodePhaseDec,
        execute: ExecutePhaseDec,
        memory: MemoryPhaseDec,
        write_back: WriteBackPhaseDec,
        num_representation: String,
        file_path: String,
        changed_ramfield: (String, String),
    }

    #[derive(Serialize)]
    struct FetchPhaseDec {
        pc: String,
        ir: String,
    }

    #[derive(Serialize)]
    struct DecodePhaseDec {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        ir: String,
        reg_bank: Vec<String>,
        a: String,
        b: String,
        wb: String,
        wb_addr: String,
        write_en: String,
        pc: String,
    }

    #[derive(Serialize)]
    struct ExecutePhaseDec {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        a: String,
        b: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct MemoryPhaseDec {
        opcode: String,
        r1: String,
        imm: String,
        long_imm: String,
        data: String,
        addr: String,
        nwe: String,
        data_out: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct WriteBackPhaseDec {
        opcode: String,
        r1: String,
        data: String,
        write_en: String,
    }

    #[derive(Serialize)]
    struct ProcessorBin {
        rom: Vec<String>,
        ram: Vec<String>,
        fetch: FetchPhaseBin,
        decode: DecodePhaseBin,
        execute: ExecutePhaseBin,
        memory: MemoryPhaseBin,
        write_back: WriteBackPhaseBin,
        num_representation: String,
        file_path: String,
        changed_ramfield: (String, String),
    }

    #[derive(Serialize)]
    struct FetchPhaseBin {
        pc: String,
        ir: String,
    }

    #[derive(Serialize)]
    struct DecodePhaseBin {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        ir: String,
        reg_bank: Vec<String>,
        a: String,
        b: String,
        wb: String,
        wb_addr: String,
        write_en: String,
        pc: String,
    }

    #[derive(Serialize)]
    struct ExecutePhaseBin {
        opcode: String,
        r1: String,
        r2: String,
        r3: String,
        r4: String,
        r5: String,
        imm: String,
        long_imm: String,
        a: String,
        b: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct MemoryPhaseBin {
        opcode: String,
        r1: String,
        imm: String,
        long_imm: String,
        data: String,
        addr: String,
        nwe: String,
        data_out: String,
        pc: String,
        br_flag: String,
    }

    #[derive(Serialize)]
    struct WriteBackPhaseBin {
        opcode: String,
        r1: String,
        data: String,
        write_en: String,
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
                // return empty rom if file not found
                return rom;
            }
            let file = file.unwrap();
            let reader = std::io::BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                // FIXME: Hässliche logic
                let line = line.unwrap();
                let split: Vec<&str> = line.split(" ").collect();
                let line_nr = split[0].parse::<u32>().unwrap();
                println!("{:?}", split);
                let line = split[1];
                let line = line.split("//").next().unwrap().trim(); // Remove comments
                let line = line.replace(" ", ""); // Remove spaces
                if !line.is_empty() {
                    println!("{}", line);
                    let value = i32::from_str_radix(&line, 2).unwrap();
                    rom[line_nr as usize] = value;
                    println!("{:032b}", value);
                }
            }
            println!("");
            rom
        }
    }
}

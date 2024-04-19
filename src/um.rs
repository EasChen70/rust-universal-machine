use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::process;

use bitpack;
type Umi = u32;
pub struct Field {
width: u32,
lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};
//ask ta what to do when failure occurs -- panic 
//how to load in a program
#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
CMov,
Load,
Store,
Add,
Multiplication,
Division, 
Bitwise, 
Halt, 
MSeg,
UmSeg, 
Output, 
Input, 
LProgram, 
LValue,
}

pub fn rum(first: Vec<u32>) {
    let mut memory: Vec<Vec<u32>> = Vec::new();
    let mut register: Vec<u32> = vec![0; 8];
    let mut identifier: Vec<usize> = Vec::new();

    // Initialize the first memory segment, memory[0]
    memory.push(first);

    let mut pc: usize = 0; // Program counter as usize

    while pc < memory[0].len() {
        let hold = disassemble(memory[0][pc]);
        if hold.0 > 13 {
            process::exit(1);
        }

        if pc >= memory[0].len() {
            process::exit(1);
        }

        // Handle valid instructions and increment pc
        if hold.0 == 1 {
            if identifier.contains(&(register[hold.2 as usize] as usize)) {
                process::exit(1);
            } else if register[hold.3 as usize] as usize >= memory.len()
                || register[hold.2 as usize] as usize >= memory.len()
                || register[hold.3 as usize] as usize >= memory[register[hold.2 as usize] as usize].len()
            {
                process::exit(1);
            } else {
                register[hold.1 as usize] = memory[register[hold.2 as usize] as usize]
                    [register[hold.3 as usize] as usize];
                pc += 1;
            }
        } else if hold.0 == 2 {
            if identifier.contains(&(register[hold.1 as usize] as usize)) {
                process::exit(1);
            } else if register[hold.2 as usize] as usize >= memory.len()
                || register[hold.1 as usize] as usize >= memory.len()
                || register[hold.2 as usize] as usize >= memory[register[hold.1 as usize] as usize].len()
            {
                process::exit(1);
            } else {
                memory[register[hold.1 as usize] as usize][register[hold.2 as usize] as usize] =
                    register[hold.3 as usize];
                pc += 1;
            }
        } else if hold.0 == 0 {
            if register[hold.3 as usize] != 0 {
                register[hold.1 as usize] = register[hold.2 as usize];
            }
            pc += 1;
        } else if (3..=6).contains(&hold.0) {
            register[hold.1 as usize] = simple_instructions(
                hold.0,
                register[hold.2 as usize],
                register[hold.3 as usize],
            );
            pc += 1;
        } else if hold.0 == 7 {
            process::exit(0);
        } else if hold.0 == 8 {
            if identifier.is_empty() {
                memory.push(vec![0; register[hold.3 as usize] as usize]);
            } else {
                memory[identifier[0]].resize(register[hold.3 as usize] as usize, 0);
                identifier.remove(0);
            }
            pc += 1;
        } else if hold.0 == 9 {
            if register[hold.3 as usize] == 0 {
                process::exit(1);
            } else if identifier.contains(&(register[hold.3 as usize] as usize)) {
                process::exit(1);
            } else {
                identifier.push(register[hold.3 as usize] as usize);
                pc += 1;
            }
        } else if hold.0 == 10 {
            if register[hold.3 as usize] <= 255 {
                print!("{}", register[hold.3 as usize] as u8 as char);
            } else {
                process::exit(1);
            }
            pc += 1;
        } else if hold.0 == 11 {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let num: u32 = input.trim().parse().expect("Invalid input");
            register[hold.3 as usize] = num;
            pc += 1;
        } else if hold.0 == 12 {
            if identifier.contains(&(register[hold.3 as usize] as usize)) {
                process::exit(1);
            }
            if register[hold.2 as usize] == 0 {
                pc = register[hold.3 as usize] as usize;
            } else {
                let temp: Vec<u32> = memory[register[hold.2 as usize] as usize].clone();
                memory[0] = temp;
                pc = register[hold.3 as usize] as usize;
            }
        } else if hold.0 == 13 {
            register[hold.1 as usize] = hold.2;
            pc += 1;
        }
    }
}



/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64).unwrap() as u32
}
/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    return FromPrimitive::from_u32(bitpack::bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64).unwrap() as u32)
}
pub fn disassemble(inst: Umi) -> (u32,u32,u32,u32) {
    match op(inst) {
        Some(Opcode::LValue) => {
            return (get(&OP,inst),get(&RL, inst),get(&VL, inst),get(&RC, inst));
        },

        _ =>{
            return (get(&OP,inst),get(&RA, inst),get(&RB, inst),get(&RC, inst));
        },
    }
}

pub fn simple_instructions(op: u32, rb: u32, rc: u32) -> u32 {
    match op {
        3 => ((rb as u64 + rc as u64) % (1u64 << 32)) as u32,
        4 => ((rb as u64 * rc as u64) % (1u64 << 32)) as u32,
        5 => {
            if rc == 0 {
                process::exit(1);
            } else {
                rb / rc
            }
        }
        6 => !(rb & rc),
        _ => process::exit(1),
    }
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{io::Read, process};
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

pub fn rum(fisrt: Vec<u32>){
    let mut memory:Vec<Vec<u32>> = vec![];
    let mut register:Vec<u32> = vec![0;8];
    let mut idenfier:Vec<u32> = vec![];
    //initalizes the first memory segement, memory[0]
    memory.push(fisrt);
    //make pc equal to the first word in memory[0]
    let mut pc: u32 = memory[0][0];
    for x in  0 .. memory[0].len(){
        //gets the op, ra, rb, rc
        let hold = disassemble(pc);
        //makes sure it is a valid op code 
        if  hold.0 > 13{
            process::exit(1);
        }
        //segment load
        //loads the segment into register a 
        else if hold.0 == 1{
            register[hold.1 as usize] = memory[register[hold.2 as usize] as usize][register[hold.3 as usize] as usize];
        }
        //segment stores
        //stores the segment 
        else if hold.0 == 2{
            memory[register[hold.1 as usize] as usize][register[hold.2 as usize] as usize] = register[hold.3 as usize];
        }
        //does the instructions cmove, add, multiplication, division, and bitwise NAND
        else if hold.0 == 0 || (3 <= hold.0 && hold.0<= 6){
            if register[hold.3 as usize] != 0{
                register[hold.1 as usize] = simple_instrutions(hold.0, register[hold.2 as usize], register[hold.3 as usize]);
            }
        }
        //halts 
        else if hold.0 == 7{
            process::exit(0);
        }
        //creates a segment 
        else if hold.0 == 8{
            //ask ta what does it mean when a bit pattern is not all zeros 
            let temp:Vec<u32> = vec![0;register[hold.3 as usize] as usize];
            //if there are no empty segments, then the new segment is pushed to the end 
            if idenfier.is_empty(){
                memory.push(temp);
            }
            //if there is an unmapped segment, then we will map the new segment into the unmapped segment and then 
            //remove the identifer from the list 
            else{
                memory[idenfier[0] as usize] = temp;
                idenfier.remove(0);
            }
        }
        //adds an unmapped identifer so that we can check when we are mapping 
        else if hold.0 == 9{
            idenfier.push(register[hold.3 as usize]);
        }
        //output
        else if hold.0 == 10{
            if register[hold.3 as usize] <= 255{
                match u8::try_from(register[hold.3 as usize]){
                Ok(val) => {print!("{}",val as char)},
                Err(error) => {panic!("{}",error)}
                }
                pc = memory[0][x+1];
            }
            else{
                process::exit(1);
            }
        }
        else if hold.0 == 11{
            //ask ta to check input and output -- look at notes 
            // Read a single byte of input from stdin
        let input_byte = std::io::stdin().bytes().next().unwrap().unwrap();

        // Convert the input byte to u32
        let input_u32: u32 = input_byte as u32;
        if input_u32 <=255{
            //ask what does it mean when an input has been signaled 
            // Store the input u32 in register[hold.3]
            register[hold.3 as usize] = input_u32;
        }
        }
        //load program 
        else if hold.0 == 12{
            //makes pc equal to the specific memory
            if register[hold.2 as usize] == 0{
                pc = memory[0][register[hold.3 as usize] as usize];
            }
            //makes temp equal to the cloned memory and stores it in memory[0]
            else {
                let temp:Vec<u32> = memory[register[hold.2 as usize] as usize].clone();
                memory[0] = temp;
                pc = memory[0][register[hold.3 as usize] as usize];
                
            }

        }
        //stores the value in register a 
        else if hold.0 == 13{
            register[hold.1 as usize] = hold.2;
            pc = memory[0][x+1];
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

pub fn simple_instrutions (op:u32, rb:u32,rc:u32) -> u32 {
    let power:u32 = 2;
    if op == 0{
        return rb;
    }
    else if op == 3{
        return (rb + rc) % power.pow(32);
    }
    else if op == 4{
        return (rb * rc) % power.pow(32);
    }
    else if op == 5{
        //ask about what to do for division 
        if rc == 0{
            process::exit(1);
        }else {
            return rb/rc;
        }
    }
    else if op == 6{
        return rb & rc;
    }
    else {
        //fail
    }
    return 1
}

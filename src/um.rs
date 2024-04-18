use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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
    let mut idenfier:Vec<u32> = vec![0,1];
    let hi = idenfier[0] as usize;
    memory.push(fisrt);
    let mut pc: u32 = memory[0][0];
    for x in  0 .. memory[(idenfier[0] as usize)].len(){
        let hold = disassemble(pc);
        if hold.0 == 0{
            register[hold.1 as usize] = simpleInstrutions(hold.0, register[hold.2 as usize], register[hold.3 as usize]);
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

pub fn simpleInstrutions (op:u32, rb:u32,rc:u32) -> u32 {
    let power:u32 = 2;
    if op == 3{
        return (rb + rc) % power.pow(32);
    }
    else if op == 4{
        return (rb * rc) % power.pow(32);
    }
    else if op == 5{
        //ask about what to do for division 
        return(rb/rc);
    }
    else if op == 6{
        return rb & rc;
    }
    else {
        //fail
    }
    return 1
}

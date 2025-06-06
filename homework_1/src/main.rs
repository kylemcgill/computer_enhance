use std::fs;
use std::env;

const HOME_PATH: &str = "/Users/kylemcgill/workspace/computer_enhance/";
const ASM_INSTRUCTION_TABLE: [[&str; 2]; 8] = [
    ["AL", "AX"],
    ["CL", "CX"],
    ["DL", "DX"],
    ["BL", "BX"],
    ["AH", "SP"],
    ["CH", "BP"],
    ["DH", "SI"],
    ["BH", "DI"]
];

const OP_CODE_TABLE: [&str; 1] = [
    "mov"
];

#[repr(u8)]
enum OpCode {
    Mov = 0x88
}

// Each operation is at minimum 2 bytes long to give all
// information needed for the operation and the registers
// to use.
#[repr(u8)]
enum AsmFlag {
    OpCode           = 0xFC,
    DirectionOperand = 0x02,
    WordOperation    = 0x01,
    RegisterMode     = 0xC0,
    RegisterOperand  = 0x38, // Extension of opcode
    RegisterToUse    = 0x07
}

fn get_register_name(register: u8, w: u8) -> String {
    ASM_INSTRUCTION_TABLE[register as usize][w as usize].to_string()
}

fn get_op_code_name(op_code: u8) -> String {
    match op_code {
        val if val == OpCode::Mov as u8 => {
            println!("Found Mov");
            return "mov".to_string()
        },
        _ => println!("Error: Did not find opcode!"),
    }
    return String::new();
}

fn disassemble(contents: Vec<u8>) {
    println!("Size of contents: {} bytes", contents.len());
    for index in (0..contents.len()).step_by(2) {
        let left_side: u8 = contents[index];
        let right_side: u8 = contents[index + 1];
    
        // D = b0; therefore REG is the source
        // REG = b011 -> BX
        // R/M = b001 -> CX 
        // This is all because MOD = b11
        println!("Hex of operation: left_side = {:X}, right_side = {:X}", left_side, right_side);
        println!("Binary of operation: left_side = {:b}, right_side = {:b}", left_side, right_side);
    
        let op_code: u8 = left_side & OpCode::Mov as u8;
        let op_name = get_op_code_name(op_code);
    
        let left_register = (contents[index + 1] & AsmFlag::RegisterOperand as u8) >> 3;
        let right_register = contents[index + 1] & AsmFlag::RegisterToUse as u8;
        let word_operation = contents[index] & AsmFlag::WordOperation as u8;
        let direction = contents[index] & AsmFlag::DirectionOperand as u8 >> 1;
        println!("left register bin: {}, right register bin: {}", left_register, right_register);
        let left_register_name = get_register_name(left_register, word_operation);
        let right_register_name = get_register_name(right_register, word_operation);
        println!("left register {}", left_register_name);
        println!("right register {}", right_register_name);
        
        if direction == 1{
            println!("{} {}, {}", op_name, right_register_name, left_register_name);
        } else {
            println!("{} {}, {}", op_name, left_register_name, right_register_name);
        }    
    }    
}
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Disassembling asm in Homework 1...");
    let contents = fs::read(HOME_PATH.to_owned() + args[1].as_str())?;
    disassemble(contents);
    println!("Done!");
    Ok(())
    
}

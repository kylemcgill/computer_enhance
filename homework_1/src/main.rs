use std::fs;
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

fn get_op_code_name(op: u8) -> String {
    OP_CODE_TABLE[op as usize].to_string()
}

fn disassemble(contents: Vec<u8>) {
    println!("Size of contents: {} bytes", contents.len());
    
    let left_side: u8 = contents[0];
    let right_side: u8 = contents[1];

    // D = b0; therefore REG is the source
    // REG = b011 -> BX
    // R/M = b001 -> CX 
    // This is all because MOD = b11
    println!("Hex of operation: left_side = {:X}, right_side = {:X}", left_side, right_side);
    println!("Binary of operation: left_side = {:b}, right_side = {:b}", left_side, right_side);

    let mut op_name = String::new();
    let op_code: u8 = left_side & OpCode::Mov as u8;
    match op_code {
        val if val == OpCode::Mov as u8 => {
            println!("Found Mov");
            op_name = "mov".to_string();
        },
        _ => println!("Error: Did not find opcode!"),
    }

    assert!(op_code == OpCode::Mov as u8);

    let left_register = (contents[1] & AsmFlag::RegisterOperand as u8) >> 3;
    let right_register = contents[1] & AsmFlag::RegisterToUse as u8;
    let word_operation = contents[0] & AsmFlag::WordOperation as u8;
    let direction = contents[0] & AsmFlag::DirectionOperand as u8 >> 1;
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
fn main() -> std::io::Result<()> {
    println!("Disassembling asm in Homework 1...");
    let contents = fs::read(HOME_PATH.to_owned() + "computer_enhance/perfaware/part1/listing_0037_single_register_mov")?;
    disassemble(contents);
    println!("Done!");
    Ok(())
    
}

use std::fmt;
use stack_vm::{Instruction, InstructionTable, Machine, Builder, WriteManyTable, Code};
extern crate console_error_panic_hook;
use std::panic;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    I(i64),
    S(String),
}

impl Operand {
    fn to_i(&self) -> Option<i64> {
        match self {
            Operand::I(i) => Some(*i),
            _ => None,
        }
    }

    fn to_s(&self) -> Option<&str> {
        match self {
            Operand::S(ref s) => Some(s),
            _ => None,
        }
    }
}

impl From<&i64> for Operand {
    fn from(i: &i64) -> Self {
        Operand::I(*i)
    }
}

impl From<i64> for Operand {
    fn from(i: i64) -> Self {
        Operand::I(i)
    }
}

impl<'a> From<&'a str> for Operand {
    fn from(s: &'a str) -> Self {
        Operand::S(s.to_string())
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f,"{:?}", self)
        match self {
            Operand::I(i) => write!(f, "{}", i),
            Operand::S(s) => write!(f, "{}", s),
        }
    }
}

fn load_val(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg);
}

fn write_var(machine: &mut Machine<Operand>, args: &[usize]) {
    let val = machine.operand_pop().clone();
    let label = machine.get_data(args[0]).clone();

    machine.set_local(label.to_s().unwrap(), val);
}

fn read_var(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).clone();
    let val = machine.get_local(label.to_s().unwrap()).unwrap().clone();

    machine.operand_push(val);
}

fn add(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().clone();
    let lhs = machine.operand_pop().clone();
    let result = lhs.to_i().unwrap() + rhs.to_i().unwrap();
    machine.operand_push(Operand::from(result));
}

fn multiply(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().clone();
    let lhs = machine.operand_pop().clone();
    let result = lhs.to_i().unwrap() * rhs.to_i().unwrap();
    machine.operand_push(Operand::from(result));
}

fn ret(machine: &mut Machine<Operand>, _args: &[usize]) {
    machine.ret();
}

fn prepare_instruction_table() -> InstructionTable<Operand> {
    let mut instruction_table = InstructionTable::new();
    instruction_table.insert(Instruction::new(0, "LOAD_VAL", 1, load_val));
    instruction_table.insert(Instruction::new(1, "WRITE_VAR", 1, write_var));
    instruction_table.insert(Instruction::new(2, "READ_VAR", 1, read_var));
    instruction_table.insert(Instruction::new(3, "ADD", 0, add));
    instruction_table.insert(Instruction::new(4, "MULTIPLY", 0, multiply));
    instruction_table.insert(Instruction::new(5, "RETURN_VALUE", 0, ret));
    return instruction_table;
}

fn parse_byte_code<'a>(byte_code: &'a str,
                       instruction_table: &'a InstructionTable<Operand>) -> Result<Builder<'a, Operand>> {
    let mut builder: Builder<Operand> = Builder::new(&instruction_table);

    let lines: Vec<&str> = byte_code.split("\n").collect();
    for line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        let items: Vec<&str> = line.trim().split(" ").collect();
        match items.len() {
            1 => {
                builder.push(items[0].trim(), vec![]);
            }
            2 => {
                let instr = items[0].trim();
                let arg = items[1].trim();
                let test = &arg.parse::<i64>();
                match test {
                    Ok(num) => {
                        builder.push(instr, vec![Operand::from(num)]);
                    }
                    Err(_) => {
                        let trimmed = arg.trim_matches('\'');
                        builder.push(instr, vec![Operand::from(trimmed)]);
                    }
                }
            }
            _ => {
                return Err(format!("Unexpected bytecode line: {}", line));
            }
        }
    }
    return Ok(builder);
}

pub fn run_machine(byte_code: &str) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let instruction_table = prepare_instruction_table();
    let constants: WriteManyTable<Operand> = WriteManyTable::new();
    match parse_byte_code(byte_code, &instruction_table) {
        Ok(builder) => {
            let mut machine: Machine<Operand> = Machine::new(Code::from(builder),
                                                     &constants, &instruction_table);
            machine.run();
            let result = machine.operand_pop();
            return result.to_string();
        },
        Err(err) => {
            return err;
        }
    }
}

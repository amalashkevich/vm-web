use std::fmt;
use stack_vm::{Instruction, InstructionTable, Machine, Builder, WriteManyTable, Code};
extern crate console_error_panic_hook;
use std::panic;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    I(i64),
    S(String),
    B(bool)
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

    fn to_bool(&self) -> Option<bool> {
        match self {
            Operand::B(b) => Some(*b),
            _ => None
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

impl From<bool> for Operand {
    fn from(b: bool) -> Self {
        Operand::B(b)
    }
}


impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f,"{:?}", self)
        match self {
            Operand::I(i) => write!(f, "{}", i),
            Operand::S(s) => write!(f, "{}", s),
            Operand::B(b) => write!(f, "{}", b),
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

fn cmp_lt(machine: &mut Machine<Operand>, _args: &[usize]) {
    let rhs = machine.operand_pop().clone();
    let lhs = machine.operand_pop().clone();
    let result = lhs.to_i().unwrap() < rhs.to_i().unwrap();
    machine.operand_push(Operand::from(result));
}

fn pop_jump_if_false(machine: &mut Machine<Operand>, args: &[usize]) {
    let condition = machine.operand_pop().to_bool().unwrap();
    if !condition {
        let label = machine.get_data(args[0]).clone();
        machine.jump(label.to_s().unwrap());
    }
}

fn print_(machine: &mut Machine<Operand>, args: &[usize]) {
    let val = machine.operand_pop().clone();

    print!("{}", val.to_string());
}

fn jump(machine: &mut Machine<Operand>, args: &[usize]) {
    let label = machine.get_data(args[0]).clone();
    machine.jump(label.to_s().unwrap());
}

fn push(machine: &mut Machine<Operand>, args: &[usize]) {
    let arg = machine.get_data(args[0]).clone();
    machine.operand_push(arg);
}

fn send_channel(machine: &mut Machine<Operand>, args: &[usize]) {
    let ch = machine.operand_pop().clone();
    let val = machine.operand_pop().clone();
    println!("SEND (ch={}, val={})", ch, val);
}

fn recv_channel(machine: &mut Machine<Operand>, args: &[usize]) {
    let ch = machine.operand_pop().clone();
    println!("RECV (ch={})", ch);
    let val: &str = "recv_channel_value";
    machine.operand_push(Operand::from(val));
}

use std::thread;
use std::time::Duration;

fn spawn(machine: &mut Machine<Operand>, args: &[usize]) {
    let func1 = machine.operand_pop().clone();
    let func2 = machine.operand_pop().clone();
    println!("SPAWN (func1={}, func2={})", func1, func2);

    thread::spawn(|| {
        fn func1() {
            for i in 1..10 {
                println!("spawned func1 i={}", i);
                thread::sleep(Duration::from_millis(1));
            }
            println!("spawned func1 end");
        }
        func1();
    });
    thread::spawn(|| {
        fn func2() {
            for i in 1..10 {
                println!("spawned func2 i={}", i);
                thread::sleep(Duration::from_millis(1));
            }
            println!("spawned func2 end");
        }
        func2();
    });
}

fn prepare_instruction_table() -> InstructionTable<Operand> {
    let mut instruction_table = InstructionTable::new();
    instruction_table.insert(Instruction::new(0, "LOAD_VAL", 1, load_val));
    instruction_table.insert(Instruction::new(1, "WRITE_VAR", 1, write_var));
    instruction_table.insert(Instruction::new(2, "READ_VAR", 1, read_var));
    instruction_table.insert(Instruction::new(3, "ADD", 0, add));
    instruction_table.insert(Instruction::new(4, "MULTIPLY", 0, multiply));
    instruction_table.insert(Instruction::new(5, "RETURN_VALUE", 0, ret));
    instruction_table.insert(Instruction::new(6, "CMP_LT", 0, cmp_lt));
    instruction_table.insert(Instruction::new(7, "POP_JUMP_IF_FALSE", 1, pop_jump_if_false));
    instruction_table.insert(Instruction::new(8, "PRINT", 1, print_));
    instruction_table.insert(Instruction::new(9, "JUMP", 1, jump));

    instruction_table.insert(Instruction::new(10, "PUSH", 1, push));
    instruction_table.insert(Instruction::new(11, "SEND_CHANNEL", 0, send_channel));
    instruction_table.insert(Instruction::new(12, "RECV_CHANNEL", 0, recv_channel));
    instruction_table.insert(Instruction::new(13, "SPAWN", 0, spawn));
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

                if instr == "LABEL" {
                    builder.label(arg);
                } else {
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
            }
            _ => {
                return Err(format!("Unexpected bytecode line: {}", line));
            }
        }
    }
    return Ok(builder);
}

pub fn run_machine(byte_code: &str) -> Option<String> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let instruction_table = prepare_instruction_table();
    let constants: WriteManyTable<Operand> = WriteManyTable::new();
    match parse_byte_code(byte_code, &instruction_table) {
        Ok(builder) => {
            let mut machine: Machine<Operand> = Machine::new(Code::from(builder),
                                                     &constants, &instruction_table);
            machine.run();
            if machine.operand_stack.is_empty() {
                return None;
            }
            return Some(machine.operand_pop().to_string());
        },
        Err(err) => {
            return Some(err);
        }
    }
}


pub fn ex_main() {
    let instruction_table = prepare_instruction_table();
    let mut builder: Builder<Operand> = Builder::new(&instruction_table);

    builder.push("LOAD_VAL", vec![Operand::from(0)]);
    builder.push("WRITE_VAR", vec![Operand::from("x")]);

    builder.label(":loop1start");
    builder.push("READ_VAR", vec![Operand::from("x")]);
    builder.push("LOAD_VAL", vec![Operand::from(3)]);
    builder.push("CMP_LT", vec![]);
    builder.push("POP_JUMP_IF_FALSE", vec![Operand::from(":loop1end")]);

    builder.push("READ_VAR", vec![Operand::from("x")]);
    builder.push("PRINT", vec![Operand::from("x")]);

    builder.push("READ_VAR", vec![Operand::from("x")]);
    builder.push("LOAD_VAL", vec![Operand::from(1)]);
    builder.push("ADD", vec![]);
    builder.push("WRITE_VAR", vec![Operand::from("x")]);

    builder.push("JUMP", vec![Operand::from(":loop1start")]);
    builder.label(":loop1end");
    builder.push("RETURN_VALUE", vec![]);


    let constants: WriteManyTable<Operand> = WriteManyTable::new();

    let mut machine: Machine<Operand> = Machine::new(Code::from(builder), &constants, &instruction_table);
    machine.run();

    let result = machine.operand_pop();
    print!("Result={}", result);
    assert_eq!(result, Operand::from(4));
}

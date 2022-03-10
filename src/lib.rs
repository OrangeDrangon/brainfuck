use std::{collections::VecDeque, io::Write};

use cells::Cells;
use parse::{parse_program, Instruction};

mod cells;
mod parse;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Done,
    Running,
    Paused,
    NotStarted,
}

#[derive(Default, Clone, Copy)]
pub struct BrainfuckVMOptions {
    pub enable_breakpoints: bool,
}

pub struct BrainfuckVM {
    pc: usize,
    cells: Cells,
    input_chars: VecDeque<char>,
    instructions: Vec<Instruction>,
    status: Status,
    vm_options: BrainfuckVMOptions,
}

impl BrainfuckVM {
    pub fn new(program: &str, input: Option<&str>, vm_options: BrainfuckVMOptions) -> Self {
        Self {
            pc: Default::default(),
            cells: Default::default(),
            input_chars: input.unwrap_or("").chars().collect(),
            status: Status::NotStarted,
            instructions: parse_program(program),
            vm_options,
        }
    }

    pub fn step(&mut self) -> Status {
        if self.status != Status::Done {
            self.process_instruction();
            self.pc += 1;
        }

        self.update_status()
    }

    pub fn run(&mut self) -> Status {
        if self.not_done() {
            self.status = Status::Running;
        }

        while self.running() {
            self.step();
        }

        self.status
    }

    pub fn not_done(&self) -> bool {
        self.status != Status::Done
    }

    pub fn running(&self) -> bool {
        self.status == Status::Running
    }

    fn update_status(&mut self) -> Status {
        if self.pc >= self.instructions.len() {
            self.status = Status::Done;
        }

        self.status
    }

    fn process_instruction(&mut self) {
        match self.instructions[self.pc] {
            Instruction::PointerIncrement => self.cells.increment_pointer(),
            Instruction::PointerDecrement => self.cells.decrement_pointer(),
            Instruction::CellIncrement => self.cells.increment_cell(),
            Instruction::CellDecrement => self.cells.decrement_cell(),
            Instruction::Breakpoint => {
                if self.vm_options.enable_breakpoints {
                    self.status = Status::Paused;
                }
            }
            Instruction::CellOutput => {
                print!("{}", self.cells.get_cell() as char);
                std::io::stdout().flush().expect("failed to flush stdout");
            }
            Instruction::CellInput => self
                .cells
                .set_cell(self.input_chars.pop_front().expect("ran out of input") as u8),

            Instruction::Open(target) => {
                if self.cells.get_cell() == 0 {
                    self.pc = target;
                }
            }
            Instruction::Close(target) => {
                if self.cells.get_cell() != 0 {
                    self.pc = target;
                }
            }
        };
    }
}

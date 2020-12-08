#![feature(str_split_once)]
use std::collections::HashSet;

fn main() {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();

    let mut cpu = Cpu::new(instructions);
    cpu.run_mutate_one();
}

struct Cpu {
    pc: isize,
    acc: isize,
    instructions: Vec<String>
}

impl Cpu {
    pub fn new(instructions: Vec<String>) -> Self {
        Self {
            pc: 0,
            acc: 0,
            instructions
        }
    }

    // Part B
    pub fn run_mutate_one(&mut self) {
        for i in 0..self.instructions.len() {
            // reset after last run
            self.pc = 0;
            self.acc = 0;

            // swap out the mutated instruction
            let old_ins = self.instructions.get(i).unwrap().to_owned();
            let new_ins = match old_ins.split_once(' ').unwrap() {
                ("acc", _) => continue,
                ("jmp", v) => format!("nop {}", v),
                ("nop", v) => format!("jmp {}", v),
                _ => panic!("unexpected")
            };
            *self.instructions.get_mut(i).unwrap() = new_ins;

            // perform a run
            if let Some(res) = self.run() {
                println!("{}", res);
                break;
            }

            // swap it back to how it was
            *self.instructions.get_mut(i).unwrap() = old_ins;
        }
    }

    // Part A with minor modifications
    pub fn run(&mut self) -> Option<isize> {
        let mut seen = HashSet::new();
        loop {
            let ins = self.instructions.get(self.pc as usize).unwrap().clone();
            self.run_ins(&ins);
            if self.pc >= self.instructions.len() as isize {
                break Some(self.acc)
            } else if !seen.insert(self.pc) {
                // This isn't foolproof but I'm not smart enough to solve the halting problem
                break None
            }
        }
    }

    fn run_ins(&mut self, ins: &str) {
        let (ins_name, ins_val_str) = ins.split_once(' ').unwrap();
        let ins_val: isize = ins_val_str.parse().unwrap();
        match ins_name {
            "acc" => { self.acc += ins_val; self.pc += 1; },
            "jmp" => self.pc += ins_val,
            "nop" => self.pc += 1,
            _ => panic!("unrecognized instruction")
        }
    }
}

use std::collections::VecDeque;

use tracing::{debug, trace};

#[derive(Debug)]
pub struct Cpu {
    id: usize,
    ip: usize,
    relative_base: i64,
    mem: Vec<i64>,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    halted: bool,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}
impl From<char> for ParameterMode {
    fn from(value: char) -> Self {
        match value {
            '0' => Self::Position,
            '1' => Self::Immediate,
            '2' => Self::Relative,
            _ => panic!("Unknown parameter mode id {value}")
        }
    }
}

impl Cpu {
    pub fn new(id: usize, mem: &[i64]) -> Self {
        Self {
            id,
            ip: 0,
            relative_base: 0,
            mem: mem.to_vec(),
            inputs: VecDeque::new(),
            outputs: Vec::new(),
            halted: false,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn input(&mut self, value: i64) {
        self.inputs.push_back(value);
    }

    pub fn take_output(&mut self) -> Vec<i64> {
        let mut outputs = Vec::new();
        outputs.append(&mut self.outputs);
        outputs
    }

    pub fn has_halted(&self) -> bool {
        self.halted
    }

    pub fn execute(&mut self) -> bool {
        let opcode = self.mem[self.ip];
        let s: Vec<_> = format!("{opcode}").chars().rev().collect();
        let opcode = s
            .iter()
            .take(2)
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let parameter_modes = s.iter().skip(2).map(|v| ParameterMode::from(*v)).collect::<Vec<_>>();
        debug!(opcode, parameter_mode = debug(&parameter_modes), "t2");
        match opcode {
            99 => {
                self.execute_halt();
                true
            }
            1 => {
                self.execute_add(&parameter_modes);
                false
            }
            2 => {
                self.execute_mult(&parameter_modes);
                false
            }
            3 => {
                self.execute_input(&parameter_modes);
                false
            }
            4 => {
                self.execute_output(&parameter_modes);
                false
            }
            5 => {
                self.execute_jump_if_true(&parameter_modes);
                false
            }
            6 => {
                self.execute_jump_if_false(&parameter_modes);
                false
            }
            7 => {
                self.execute_less_than(&parameter_modes);
                false
            }
            8 => {
                self.execute_equals(&parameter_modes);
                false
            }
            9 => {
                self.adjust_relative_base(&parameter_modes);
                false
            }
            _ => panic!("unknown opcode: {opcode}"),
        }
    }
}

impl Cpu {
    fn value(&self, offset: usize, parameter_modes: &[ParameterMode]) -> i64 {
        use ParameterMode::*;
        match parameter_modes.get(offset - 1).unwrap_or(&Position) {
            Position => {
                let value = self.mem[self.ip + offset];
                if value < 0 {
                    panic!("Attempt to access negative memory: {value}");
                }
                *self.mem.get(value as usize).unwrap_or(&0)
            },
            Immediate => {self.mem[self.ip + offset]},
            Relative => {
                let address = self.mem[self.ip + offset] + self.relative_base;
                if address < 0 {
                    panic!("Attempt to access negative memory: {address}");
                }
                *self.mem.get(address as usize).unwrap_or(&0)
            },
            mode => panic!("Unknown parameter mode {mode:?}")
        }
    }

    fn set_value(&mut self, offset: usize, parameter_modes: &[ParameterMode], value: i64) {
        use ParameterMode::*;
        debug!(cpu=debug(&self), offset, parameter_modes = debug(parameter_modes), value, "set_value");
        let address = match parameter_modes.get(offset - 1).unwrap_or(&Position) {
            Position => self.mem[self.ip + offset],
            Relative => self.mem[self.ip + offset] + self.relative_base, 
            mode => panic!("Unknown parameter mode {mode:?}")
        };
        if address < 0 {
            panic!("Cannot access negative addresses: {address}");
        }
        let address = address as usize;
        if self.mem.len() <= address {
            self.mem.resize(address + 1, 0);
        }
        self.mem[address] = value;
    }

    fn execute_halt(&mut self) {
        self.halted = true;
        debug!(id = self.id, cpu = debug(&self), "halt");
    }

    fn execute_add(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);

        let r = a + b;
        debug!(a, b, r, "add");
        self.set_value(3, parameter_modes, r);
        self.ip += 4;
    }

    fn execute_mult(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);

        let r = a * b;
        debug!(a, b, r, "mult");
        self.set_value(3, parameter_modes, r);
        self.ip += 4;
    }

    fn execute_input(&mut self, parameter_modes: &[ParameterMode]) {
        if self.inputs.is_empty() {
            debug!(cpu = self.id, cpu = debug(&self), "no input");
            return;
        }
        let input = self.inputs.pop_front().unwrap();
        self.set_value(1, parameter_modes, input);
        self.ip += 2;
    }

    fn execute_output(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);

        // let mut a = self.mem[self.ip + 1];
        // if !parameter_mode.first().unwrap_or(&false) {
        //     a = self.mem[a as usize];
        // }
        debug!(a, "output");
        self.outputs.push(a);
        self.ip += 2;
    }

    fn execute_jump_if_true(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);
        // let mut a = self.mem[self.ip + 1];
        // if !parameter_mode.first().unwrap_or(&false) {
        //     a = self.mem[a as usize];
        // }
        // let mut b = self.mem[self.ip + 2];
        // if !parameter_mode.get(1).unwrap_or(&false) {
        //     b = self.mem[b as usize];
        //}
        if a != 0 {
            self.ip = b as usize;
        } else {
            self.ip += 3;
        }
    }

    fn execute_jump_if_false(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);
        // let mut a = self.mem[self.ip + 1];
        // if !parameter_mode.first().unwrap_or(&false) {
        //     a = self.mem[a as usize];
        // }
        // let mut b = self.mem[self.ip + 2];
        // if !parameter_mode.get(1).unwrap_or(&false) {
        //     b = self.mem[b as usize];
        // }
        debug!(a, b, "jf");
        if a == 0 {
            self.ip = b as usize;
        } else {
            self.ip += 3;
        }
    }

    fn execute_less_than(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);

        let r = if a < b { 1 } else { 0 };
        debug!(a, b, r, "<");
        self.set_value(3, parameter_modes, r);
        self.ip += 4;
    }

    fn execute_equals(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        let b = self.value(2, parameter_modes);

        let r = if a == b { 1 } else { 0 };
        debug!(a, b, r, "=");
        self.set_value(3, parameter_modes, r);
        self.ip += 4;
    }

    fn adjust_relative_base(&mut self, parameter_modes: &[ParameterMode]) {
        let a = self.value(1, parameter_modes);
        // let mut a = self.mem[self.ip + 1];
        // if !parameter_mode.first().unwrap_or(&false) {
        //     a = self.mem[a as usize];
        // }
        debug!(a, self.relative_base, "relative base");
        self.relative_base += a;
        self.ip += 2;
    }
}

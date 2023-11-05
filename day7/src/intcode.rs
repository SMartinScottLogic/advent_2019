use std::collections::VecDeque;

use tracing::{debug, trace};

#[derive(Debug)]
pub struct CPU {
    id: usize,
    ip: usize,
    mem: Vec<i64>,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
    halted: bool,
}

impl CPU {
    pub fn new(id: usize, mem: &[i64]) -> Self {
        Self {
            id,
            ip: 0,
            mem: mem.to_vec(),
            inputs: VecDeque::new(),
            outputs: Vec::new(),
            halted: false,
        }
    }
}

impl CPU {
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
        let immediate = s.iter().skip(2).map(|v| v != &'0').collect::<Vec<_>>();
        trace!(opcode, immediate = debug(&immediate), "t2");
        match opcode {
            99 => {
                self.execute_halt();
                true
            }
            1 => {
                self.execute_add(&immediate);
                false
            }
            2 => {
                self.execute_mult(&immediate);
                false
            }
            3 => {
                self.execute_input();
                false
            }
            4 => {
                self.execute_output(&immediate);
                false
            }
            5 => {
                self.execute_jump_if_true(&immediate);
                false
            }
            6 => {
                self.execute_jump_if_false(&immediate);
                false
            }
            7 => {
                self.execute_less_than(&immediate);
                false
            }
            8 => {
                self.execute_equals(&immediate);
                false
            }
            _ => panic!("unknown opcode: {opcode}"),
        }
    }

    fn execute_halt(&mut self) {
        self.halted = true;
        debug!(id = self.id, cpu = debug(&self), "halt");
    }

    fn execute_add(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        let mut b = self.mem[self.ip + 2];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        let addr_r = self.mem[self.ip + 3] as usize;
        let r = a + b;
        debug!(a, b, addr_r, r, "add");
        self.mem[addr_r] = r;
        self.ip += 4;
    }

    fn execute_mult(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        let mut b = self.mem[self.ip + 2];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        let addr_r = self.mem[self.ip + 3] as usize;
        let r = a * b;
        debug!(a, b, addr_r, r, "mult");
        self.mem[addr_r] = r;
        self.ip += 4;
    }

    fn execute_input(&mut self) {
        if self.inputs.is_empty() {
            debug!(cpu = self.id, cpu = debug(&self), "no input");
            return;
        }
        let input = self.inputs.pop_front().unwrap();
        let addr = self.mem[self.ip + 1] as usize;
        self.mem[addr] = input;
        debug!(addr, "input");
        self.ip += 2;
    }

    fn execute_output(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        debug!(a, "output");
        self.outputs.push(a);
        self.ip += 2;
    }

    fn execute_jump_if_true(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        let mut b = self.mem[self.ip + 2];
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        if a != 0 {
            self.ip = b as usize;
        } else {
            self.ip += 3;
        }
    }

    fn execute_jump_if_false(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        let mut b = self.mem[self.ip + 2];
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        debug!(a, b, "jf");
        if a == 0 {
            self.ip = b as usize;
        } else {
            self.ip += 3;
        }
    }

    fn execute_less_than(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        let mut b = self.mem[self.ip + 2];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        let addr_r = self.mem[self.ip + 3] as usize;
        let r = if a < b { 1 } else { 0 };
        debug!(a, b, addr_r, r, "<");
        self.mem[addr_r] = r;
        self.ip += 4;
    }

    fn execute_equals(&mut self, immediate: &[bool]) {
        let mut a = self.mem[self.ip + 1];
        let mut b = self.mem[self.ip + 2];
        if !immediate.first().unwrap_or(&false) {
            a = self.mem[a as usize];
        }
        if !immediate.get(1).unwrap_or(&false) {
            b = self.mem[b as usize];
        }
        let addr_r = self.mem[self.ip + 3] as usize;
        let r = if a == b { 1 } else { 0 };
        debug!(a, b, addr_r, r, "=");
        self.mem[addr_r] = r;
        self.ip += 4;
    }
}

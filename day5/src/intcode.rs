use std::collections::VecDeque;

use tracing::debug;

pub fn execute(
    ip: usize,
    mem: &mut [i64],
    inputs: &mut VecDeque<i64>,
    outputs: &mut Vec<i64>,
) -> (bool, usize) {
    let opcode = mem[ip];
    let s: Vec<_> = format!("{opcode}").chars().rev().collect();
    let opcode = s
        .iter()
        .take(2)
        .rev()
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let immediate = s.iter().skip(2).map(|v| v != &'0').collect::<Vec<_>>();
    debug!(opcode, immediate = debug(&immediate), "t2");
    // TODO immediate mode extraction
    match opcode {
        99 => (true, execute_halt()),
        1 => (false, execute_add(ip, mem, &immediate)),
        2 => (false, execute_mult(ip, mem, &immediate)),
        3 => (false, execute_input(ip, mem, inputs)),
        4 => (false, execute_output(ip, mem, &immediate, outputs)),
        5 => (false, execute_jump_if_true(ip, mem, &immediate)),
        6 => (false, execute_jump_if_false(ip, mem, &immediate)),
        7 => (false, execute_less_than(ip, mem, &immediate)),
        8 => (false, execute_equals(ip, mem, &immediate)),
        _ => panic!("unknown opcode: {opcode}"),
    }
}

fn execute_halt() -> usize {
    1
}

fn execute_add(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    let mut b = mem[ip + 2];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    if !immediate.get(1).unwrap_or(&false) {
        b = mem[b as usize];
    }
    let addr_r = mem[ip + 3] as usize;
    let r = a + b;
    debug!(a, b, addr_r, r, "add");
    mem[addr_r] = r;
    ip + 4
}

fn execute_mult(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    let mut b = mem[ip + 2];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    if !immediate.get(1).unwrap_or(&false) {
        b = mem[b as usize];
    }
    let addr_r = mem[ip + 3] as usize;
    let r = a * b;
    debug!(a, b, addr_r, r, "mult");
    mem[addr_r] = r;
    ip + 4
}

fn execute_input(ip: usize, mem: &mut [i64], inputs: &mut VecDeque<i64>) -> usize {
    let input = inputs.pop_front().unwrap();
    let addr = mem[ip + 1] as usize;
    mem[addr] = input;
    debug!(addr, "input");
    ip + 2
}

fn execute_output(ip: usize, mem: &mut [i64], immediate: &[bool], outputs: &mut Vec<i64>) -> usize {
    let mut a = mem[ip + 1];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    debug!(a, "output");
    outputs.push(a);
    ip + 2
}

fn execute_jump_if_true(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    let mut b = mem[ip + 2];
    if !immediate.get(1).unwrap_or(&false) {
        b = mem[b as usize];
    }
    if a != 0 {
        b as usize
    } else {
        ip + 3
    }
}

fn execute_jump_if_false(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    let mut b = mem[ip + 2];
    if !immediate.get(1).unwrap_or(&false) {
        b = mem[b as usize];
    }
    debug!(a, b, "jf");
    if a == 0 {
        b as usize
    } else {
        ip + 3
    }
}

fn execute_less_than(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    let mut b = mem[ip + 2];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    if !immediate.get(1).unwrap_or(&false) {
        b = mem[b as usize];
    }
    let addr_r = mem[ip + 3] as usize;
    let r = if a < b { 1 } else { 0 };
    debug!(a, b, addr_r, r, "<");
    mem[addr_r] = r;
    ip + 4
}

fn execute_equals(ip: usize, mem: &mut [i64], immediate: &[bool]) -> usize {
    let mut a = mem[ip + 1];
    let mut b = mem[ip + 2];
    if !immediate.first().unwrap_or(&false) {
        a = mem[a as usize];
    }
    if !immediate.get(1).unwrap_or(&false) {
        b = *mem.get(b as usize).unwrap_or(&0);
    }
    let addr_r = mem[ip + 3] as usize;
    let r = if a == b { 1 } else { 0 };
    debug!(a, b, addr_r, r, "=");
    mem[addr_r] = r;
    ip + 4
}

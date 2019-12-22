use std::collections::VecDeque;
use std::mem;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub struct Intcode {
    memory: Vec<i64>,
    ip: usize,
    relative_base: i64,
    state: State,
}

pub trait Input {
    fn get_input(&mut self) -> Option<i64>;
}

pub trait Output {
    fn receive_output(&mut self, output: i64);
}

#[derive(Clone, Debug)]
pub struct IoBus {
    values: VecDeque<i64>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum State {
    Running,
    WaitingForInput(Op),
    Halted,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
    Halt,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Op {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

impl Intcode {
    pub fn execute(&mut self) {
        self.execute_with_io(|| 0, |_| {});
    }

    pub fn execute_with_io<I: Input, O: Output>(&mut self, mut input: I, mut output: O) {
        while self.step(&mut input, &mut output) {}
    }

    pub fn is_halted(&self) -> bool {
        self.state == State::Halted
    }

    pub fn mem_access(&mut self, address: i64) -> &mut i64 {
        if address < 0 {
            panic!("Oh no, address is less than 0: {}", address);
        }
        if address as usize >= self.memory.len() {
            self.memory.resize(
                std::cmp::max(address as usize + 1, self.memory.len() * 2),
                0,
            );
        }
        self.memory.get_mut(address as usize).unwrap()
    }

    fn op(&mut self) -> Op {
        let val = *self.mem_access(self.ip as i64) as u32;
        let (opcode, param_count) = match val % 100 {
            1 => (Opcode::Add, 3),
            2 => (Opcode::Multiply, 3),
            3 => (Opcode::Input, 1),
            4 => (Opcode::Output, 1),
            5 => (Opcode::JumpIfTrue, 2),
            6 => (Opcode::JumpIfFalse, 2),
            7 => (Opcode::LessThan, 3),
            8 => (Opcode::Equals, 3),
            9 => (Opcode::RelativeBaseOffset, 1),
            99 => (Opcode::Halt, 0),
            _ => panic!("invalid operation"),
        };
        let mut param_modes = val / 100;
        let mut parameters = Vec::with_capacity(param_count);
        for i in 1..=param_count {
            parameters.push(Parameter {
                mode: match param_modes % 10 {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    2 => ParameterMode::Relative,
                    _ => panic!("invalid parameter mode"),
                },
                value: *self.mem_access((self.ip + i) as i64),
            });
            param_modes /= 10;
        }
        self.ip += 1 + param_count;
        Op { opcode, parameters }
    }

    fn step<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) -> bool {
        if self.state == State::Halted {
            return false;
        }

        // If we saved an operation while waiting for input, use that, otherwise read a new op
        let op =
            if let State::WaitingForInput(lastop) = mem::replace(&mut self.state, State::Running) {
                lastop
            } else {
                self.op()
            };

        // Check for halt
        if op.opcode == Opcode::Halt {
            self.state = State::Halted;
            return false;
        }

        // Execute the op
        match op.opcode {
            Opcode::Add => {
                let val = self.load(op.parameters[0]) + self.load(op.parameters[1]);
                self.write(op.parameters[2], val);
            }
            Opcode::Multiply => {
                let val = self.load(op.parameters[0]) * self.load(op.parameters[1]);
                self.write(op.parameters[2], val);
            }
            Opcode::Input => match input.get_input() {
                None => {
                    self.state = State::WaitingForInput(op);
                    return false;
                }
                Some(input) => self.write(op.parameters[0], input),
            },
            Opcode::Output => output.receive_output(self.load(op.parameters[0])),
            Opcode::JumpIfTrue => {
                if self.load(op.parameters[0]) != 0 {
                    self.ip = self.load(op.parameters[1]) as usize;
                }
            }
            Opcode::JumpIfFalse => {
                if self.load(op.parameters[0]) == 0 {
                    self.ip = self.load(op.parameters[1]) as usize;
                }
            }
            Opcode::LessThan => {
                let val = if self.load(op.parameters[0]) < self.load(op.parameters[1]) {
                    1
                } else {
                    0
                };
                self.write(op.parameters[2], val);
            }
            Opcode::Equals => {
                let val = if self.load(op.parameters[0]) == self.load(op.parameters[1]) {
                    1
                } else {
                    0
                };
                self.write(op.parameters[2], val);
            }
            Opcode::RelativeBaseOffset => {
                self.relative_base += self.load(op.parameters[0]);
            }
            _ => unimplemented!(),
        };
        true
    }

    fn load(&mut self, parameter: Parameter) -> i64 {
        match parameter.mode {
            ParameterMode::Position => *self.mem_access(parameter.value),
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Relative => *self.mem_access(self.relative_base + parameter.value),
        }
    }

    fn write(&mut self, destination: Parameter, value: i64) {
        match destination.mode {
            ParameterMode::Position => *self.mem_access(destination.value) = value,
            ParameterMode::Relative => {
                *self.mem_access(self.relative_base + destination.value) = value;
            }
            ParameterMode::Immediate => {
                panic!("immediate parameter mode not supported for write");
            }
        }
    }
}

impl<T> Input for T
where
    T: FnMut() -> i64,
{
    fn get_input(&mut self) -> Option<i64> {
        Some(self())
    }
}

impl<T> Output for T
where
    T: FnMut(i64),
{
    fn receive_output(&mut self, output: i64) {
        self(output);
    }
}

impl IoBus {
    pub fn read(&mut self) -> Option<i64> {
        self.values.pop_front()
    }

    pub fn write(&mut self, value: i64) {
        self.values.push_back(value);
    }

    pub fn read_str(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.read() {
            if c > 255 {
                self.values.push_front(c);
                break;
            }
            s.push(c as u8 as char);
        }
        s
    }

    pub fn write_str(&mut self, s: &str) {
        s.chars()
            .map(|ch| ch as u8 as i64)
            .for_each(|ch| self.write(ch));
    }
}

impl Default for IoBus {
    fn default() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl Input for IoBus {
    fn get_input(&mut self) -> Option<i64> {
        self.read()
    }
}

impl Output for IoBus {
    fn receive_output(&mut self, output: i64) {
        self.write(output);
    }
}

impl Input for &mut IoBus {
    fn get_input(&mut self) -> Option<i64> {
        self.read()
    }
}

impl Output for &mut IoBus {
    fn receive_output(&mut self, output: i64) {
        self.write(output);
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: Result<Vec<i64>, ParseIntError> = s.split(',').map(str::parse).collect();
        Ok(Self {
            memory: code?,
            ip: 0,
            relative_base: 0,
            state: State::Running,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_step() {
        let mut program = Intcode {
            memory: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            ip: 0,
            relative_base: 0,
            state: State::Running,
        };

        program.step(&mut || 0, &mut |_| {});
        assert_eq!(
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            program.memory
        );
        assert_eq!(4, program.ip);

        program.step(&mut || 0, &mut |_| {});
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            program.memory
        );
        assert_eq!(8, program.ip);
    }
}

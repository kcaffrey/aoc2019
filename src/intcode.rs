use std::mem;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub struct Intcode {
    pub memory: Vec<i32>,
    ip: usize,
    state: State,
}

pub trait Input {
    fn get_input(&mut self) -> Option<i32>;
}

pub trait Output {
    fn receive_output(&mut self, output: i32);
}

#[derive(Clone, PartialEq, Eq)]
enum State {
    Running,
    WaitingForInput(Op),
    Halted,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(Clone, PartialEq, Eq)]
struct Op {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
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

    fn op(&mut self) -> Op {
        let val = self.memory[self.ip] as u32;
        let (opcode, param_count) = match val % 100 {
            1 => (Opcode::Add, 3),
            2 => (Opcode::Multiply, 3),
            3 => (Opcode::Input, 1),
            4 => (Opcode::Output, 1),
            5 => (Opcode::JumpIfTrue, 2),
            6 => (Opcode::JumpIfFalse, 2),
            7 => (Opcode::LessThan, 3),
            8 => (Opcode::Equals, 3),
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
                    _ => panic!("invalid parameter mode"),
                },
                value: self.memory[self.ip + i],
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
                self.memory[op.parameters[2].value as usize] =
                    self.load(op.parameters[0]) + self.load(op.parameters[1]);
            }
            Opcode::Multiply => {
                self.memory[op.parameters[2].value as usize] =
                    self.load(op.parameters[0]) * self.load(op.parameters[1]);
            }
            Opcode::Input => match input.get_input() {
                None => {
                    self.state = State::WaitingForInput(op);
                    return false;
                }
                Some(input) => self.memory[op.parameters[0].value as usize] = input,
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
                self.memory[op.parameters[2].value as usize] =
                    if self.load(op.parameters[0]) < self.load(op.parameters[1]) {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Equals => {
                self.memory[op.parameters[2].value as usize] =
                    if self.load(op.parameters[0]) == self.load(op.parameters[1]) {
                        1
                    } else {
                        0
                    };
            }
            _ => unimplemented!(),
        };
        true
    }

    fn load(&self, parameter: Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Position => self.memory[parameter.value as usize],
            ParameterMode::Immediate => parameter.value,
        }
    }
}

impl<T> Input for T
where
    T: FnMut() -> i32,
{
    fn get_input(&mut self) -> Option<i32> {
        Some(self())
    }
}

impl<T> Output for T
where
    T: FnMut(i32),
{
    fn receive_output(&mut self, output: i32) {
        self(output);
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: Result<Vec<i32>, ParseIntError> = s.split(',').map(str::parse).collect();
        Ok(Self {
            memory: code?,
            ip: 0,
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

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub struct Intcode {
    pub memory: Vec<i32>,
    ip: usize,
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

struct Op {
    opcode: Opcode,
    parameters: Vec<Parameter>,
}

#[derive(Copy, Clone)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Copy, Clone)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Intcode {
    pub fn execute(&mut self) {
        self.execute_with_io(|| 0, |_| {});
    }

    pub fn execute_with_io<I, O>(&mut self, mut input: I, mut output: O)
    where
        I: FnMut() -> i32,
        O: FnMut(i32),
    {
        while self.step(|| input(), |v| output(v)) {}
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

    fn step<I, O>(&mut self, mut input: I, mut output: O) -> bool
    where
        I: FnMut() -> i32,
        O: FnMut(i32),
    {
        let op = self.op();
        if op.opcode == Opcode::Halt {
            return false;
        }
        match op.opcode {
            Opcode::Add => {
                self.memory[op.parameters[2].value as usize] =
                    self.load(op.parameters[0]) + self.load(op.parameters[1]);
            }
            Opcode::Multiply => {
                self.memory[op.parameters[2].value as usize] =
                    self.load(op.parameters[0]) * self.load(op.parameters[1]);
            }
            Opcode::Input => self.memory[op.parameters[0].value as usize] = input(),
            Opcode::Output => output(self.load(op.parameters[0])),
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

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: Result<Vec<i32>, ParseIntError> = s.split(',').map(str::parse).collect();
        Ok(Self {
            memory: code?,
            ip: 0,
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
        };

        program.step(|| 0, |_| {});
        assert_eq!(
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            program.memory
        );
        assert_eq!(4, program.ip);

        program.step(|| 0, |_| {});
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            program.memory
        );
        assert_eq!(8, program.ip);
    }
}

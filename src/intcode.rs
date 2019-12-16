use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone)]
pub struct Intcode {
    pub memory: Vec<u32>,
    ip: usize,
}

impl Intcode {
    pub fn execute(&mut self) {
        while self.memory[self.ip] != 99 {
            self.step();
        }
    }

    fn step(&mut self) {
        let opcode = self.memory[self.ip];
        let param_length = 3;
        let (operand1, operand2) = (self.memory[self.memory[self.ip + 1] as usize], self.memory[self.memory[self.ip + 2]  as usize]);
        let result = match opcode {
            1 => operand1 + operand2,
            2 => operand1 * operand2,
            _ => panic!("unexpected value")
        };
        let result_address = self.memory[self.ip + 3] as usize;
        self.memory[result_address] = result;
        self.ip += 1 + param_length;
    }
}

impl FromStr for Intcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code: Result<Vec<u32>, ParseIntError> = s.split(',').map(str::parse).collect();
        Ok(Self{
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

        program.step();
        assert_eq!(vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program.memory);
        assert_eq!(4, program.ip);

        program.step();
        assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], program.memory);
        assert_eq!(8, program.ip);
    }
}
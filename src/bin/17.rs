advent_of_code::solution!(17);

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u32> for OpCode {
    fn from(prog: u32) -> Self {
        match prog {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Invalid Program value: {}", prog),
        }
    }
}

#[derive(Debug)]
struct Register(u32);

#[derive(Debug)]
struct Registers {
    a: Register,
    b: Register,
    c: Register,
}

impl Registers {
    fn new(input: &str) -> Self {
        let mut blocks = input.split("\n\n");
        let regs_block = blocks.next().unwrap();
        let mut registers = vec![];
        for s in regs_block.lines() {
            let parts: Vec<&str> = s.split(":").collect();
            if parts.len() == 2 {
                if let Ok(num) = parts[1].trim().parse::<u32>() {
                    registers.push(num);
                }
            }
        }

        assert_eq!(registers.len(), 3);
        Registers {
            a: Register(registers[0]),
            b: Register(registers[1]),
            c: Register(registers[2]),
        }
    }
}

#[derive(Debug, Clone)]
struct Program(u32);

#[derive(Debug)]
struct Programs(Vec<Program>);

impl Programs {
    fn new(input: &str) -> Self {
        let mut blocks = input.split("\n\n").skip(1);
        let prog_block = blocks.next().unwrap();
        Programs(
            prog_block
                .replace("Program: ", "")
                .split(",")
                .filter_map(|s| Some(Program(s.trim().parse::<u32>().unwrap())))
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Computer {
    registers: Registers,
    programs: Programs,
    pc: usize,
    output: Vec<u32>,
}

impl Computer {
    fn new(input: &str) -> Self {
        Computer {
            registers: Registers::new(input),
            programs: Programs::new(input),
            pc: 0,
            output: vec![],
        }
    }

    fn run(&mut self) {
        let programs_len = self.programs.0.len();
        while self.pc < programs_len {
            // First prog is the OpCode and second is the operand
            let val_1 = self.programs.0[self.pc].0;
            let val_2 = self.programs.0[self.pc + 1].0;

            let opcode = OpCode::from(val_1);
            let operand = val_2;
            self.calculate(opcode, operand);
        }
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn combo(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.registers.a.0,
            5 => self.registers.b.0,
            6 => self.registers.c.0,
            _ => panic!("Combo operands 7 and over are reserved"),
        }
    }

    fn calculate(&mut self, opcode: OpCode, operand: u32) {
        let combo = self.combo(operand);
        match opcode {
            OpCode::Adv => {
                // same as a = a / (2 ^ combo)
                self.registers.a.0 = self.registers.a.0 >> combo;
                self.pc += 2;
            }
            OpCode::Bxl => {
                self.registers.b.0 = self.registers.b.0 ^ operand;
                self.pc += 2
            }
            OpCode::Bst => {
                self.registers.b.0 = combo % 8;
                self.pc += 2;
            }
            OpCode::Jnz => {
                if self.registers.a.0 > 0 {
                    self.pc = operand as usize;
                } else {
                    self.pc += 2
                }
            }
            OpCode::Bxc => {
                self.registers.b.0 = self.registers.b.0 ^ self.registers.c.0;
                self.pc += 2;
            }
            OpCode::Out => {
                let output = combo % 8;
                self.output.push(output);
                self.pc += 2;
            }
            OpCode::Bdv => {
                // same as b = a / (2 ^ combo)
                self.registers.b.0 = self.registers.a.0 >> combo;
                self.pc += 2;
            }
            OpCode::Cdv => {
                // same as c = a / (2 ^ combo)
                self.registers.c.0 = self.registers.a.0 >> combo;
                self.pc += 2;
            }
        };
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);
    computer.run();
    let res = computer.output();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

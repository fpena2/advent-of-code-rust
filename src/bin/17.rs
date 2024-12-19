use rayon::slice::ParallelSliceMut;

advent_of_code::solution!(17);

#[derive(Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    programs: Vec<u32>,
    pc: u32,
    output: Vec<u32>,
}

impl Computer {
    fn run(&mut self) -> String {
        let programs = self.programs.clone();
        while self.pc < programs.len() as u32 {
            let opcode = programs[self.pc as usize];
            let operand = programs[self.pc as usize + 1];
            self.calculate(opcode, operand);
        }

        self.output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn calculate(&mut self, opcode: u32, operand: u32) {
        let combo = match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Combo operand 7 is reserved"),
            _ => unreachable!(),
        };

        match opcode {
            0 => {
                // self.a = self.a / 2u32.pow(combo);
                self.a = self.a >> combo;
                self.pc += 2;
            }
            1 => {
                self.b = self.b ^ operand;
                self.pc += 2
            }
            2 => {
                self.b = combo % 8;
                self.pc += 2;
            }
            3 => {
                if self.a > 0 {
                    self.pc = operand;
                } else {
                    self.pc += 2
                }
            }
            4 => {
                self.b = self.b ^ self.c;
                self.pc += 2;
            }
            5 => {
                let output = combo % 8;
                self.output.push(output);
                self.pc += 2;
            }
            6 => {
                // self.b = self.a / 2u32.pow(combo);
                self.b = self.a >> combo;
                self.pc += 2;
            }
            7 => {
                // self.b = self.a / 2u32.pow(combo);
                self.c = self.a >> combo;
                self.pc += 2;
            }
            _ => unreachable!(),
        };
    }

    pub fn get_programs(input: &str) -> Vec<u32> {
        let mut blocks = input.split("\n\n").skip(1);
        let prog_block = blocks.next().unwrap();
        prog_block
            .replace("Program: ", "")
            .split(",")
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect()
    }

    pub fn get_registers(input: &str) -> Vec<u32> {
        let mut blocks = input.split("\n\n");
        let regs_block = blocks.next().unwrap();
        regs_block
            .lines()
            .filter_map(|s| {
                let parts: Vec<&str> = s.split(":").collect();
                if parts.len() == 2 {
                    if let Ok(num) = parts[1].trim().parse::<u32>() {
                        Some(num)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn new(input: &str) -> Self {
        let registers = Self::get_registers(input);
        let programs = Self::get_programs(input);

        assert_eq!(registers.len(), 3);
        Computer {
            a: *registers.get(0).unwrap(),
            b: *registers.get(1).unwrap(),
            c: *registers.get(2).unwrap(),
            programs,
            pc: 0,
            output: vec![],
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);
    let output = computer.run();
    Some(output)
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

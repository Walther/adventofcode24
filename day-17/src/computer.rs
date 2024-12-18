use std::{collections::HashSet, fmt::Display, mem::transmute, str::FromStr};

#[allow(clippy::enum_glob_use)]
use Instruction::*;
#[allow(clippy::enum_glob_use)]
use Operand::*;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Computer {
    /// Register A
    a: u64,
    /// Register B
    b: u64,
    /// Register C
    c: u64,
    /// Instruction pointer
    p: usize,
    /// Jump flag, set when a `jnz` instruction modifies the instruction pointer. When set, prevents the instruction pointer from being incremented by one at the end of cycle. The flag is cleared before the next cycle.
    jmp: bool,
    /// Output buffer
    out: Vec<u8>,
    /// Loaded program
    program: Program,
}

impl Computer {
    #[must_use]
    pub fn new(a: u64, b: u64, c: u64, program: &str) -> Self {
        let mut computer = Computer {
            a,
            b,
            c,
            ..Default::default()
        };
        computer.load(program);
        computer
    }

    /// Reads the given program string and parses it into an executable `Program`,
    /// and loads the result for execution.
    ///
    /// # Panics
    ///
    /// Panics if the program contains steps that cannot be parsed as integer,
    /// or if the steps contain integers outside the range `0..=7`
    pub fn load(&mut self, program: &str) {
        let steps: Vec<u8> = program
            .trim()
            .split(',')
            .map(|c| c.parse().expect("Unable to parse program step as integer"))
            .collect();
        assert!(
            steps.iter().all(|&n| n <= 7),
            "Program contains invalid steps, outside the range `0..=7`"
        );
        let mut program = Vec::new();
        let mut iter = steps.chunks_exact(2);
        while let Some(&[instruction, operand]) = iter.next() {
            unsafe {
                program.push((
                    transmute::<u8, Instruction>(instruction),
                    transmute::<u8, Operand>(operand),
                ));
            }
        }
        self.program = program;
    }

    pub fn soft_reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.p = 0;
        self.jmp = false;
        self.out = Vec::new();
    }

    pub fn hard_reset(&mut self) {
        *self = Computer::default();
    }

    pub fn set_register(&mut self, register: char, value: u64) {
        match register {
            'a' => self.a = value,
            'b' => self.b = value,
            'c' => self.c = value,
            _ => (),
        }
    }

    /// Executes the program of this [`Computer`].
    ///
    /// # Errors
    ///
    /// Errors if the computer detects an infinite loop.
    pub fn run(&mut self, verbose: bool) -> Result<(), String> {
        let mut states: HashSet<(u64, u64, u64, usize)> = HashSet::new();
        states.insert((self.a, self.b, self.c, self.p));
        let mut cycle = 0;
        loop {
            if verbose {
                println!("Cycle      : {cycle}");
                println!("{self}");
            }
            let Some(&(instruction, operand)) = self.program.get(self.p) else {
                break;
            };
            if verbose {
                println!("{instruction}");
                println!("{operand}");
                println!();
            }
            self.cycle(instruction, operand);
            match self.jmp {
                true => self.jmp = false,
                false => self.p += 1,
            }
            let new_state = states.insert((self.a, self.b, self.c, self.p));
            if !new_state {
                return Err("Computer has detected a loop".to_string());
            }
            cycle += 1;
        }

        Ok(())
    }

    fn cycle(&mut self, instruction: Instruction, operand: Operand) {
        #[allow(clippy::cast_possible_truncation)]
        match instruction {
            adv => self.a /= 2u64.pow(self.combo(operand) as u32),
            bxl => self.b ^= Computer::literal(operand),
            bst => self.b = self.combo(operand) % 8,
            jnz => {
                if self.a != 0 {
                    self.p = Computer::literal(operand) as usize / 2;
                    self.jmp = true;
                }
            }
            bxc => self.b ^= self.c,
            out => self.out.push((self.combo(operand) % 8) as u8),
            bdv => self.b = self.a / (2u64.pow(self.combo(operand) as u32)),
            cdv => self.c = self.a / (2u64.pow(self.combo(operand) as u32)),
        }
    }

    #[must_use]
    pub fn print(&self) -> String {
        self.out
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    #[must_use]
    pub fn print_program(&self) -> String {
        self.program
            .iter()
            .map(|(i, o)| format!("{i},{o}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[must_use]
    /// Interprets the given combo operand
    ///
    /// # Panics
    /// This function will panic if the reserved combo operand `7` is encountered.
    pub fn combo(&self, operand: Operand) -> u64 {
        match operand {
            op0 => 0,
            op1 => 1,
            op2 => 2,
            op3 => 3,
            op4 => self.a,
            op5 => self.b,
            op6 => self.c,
            op7 => panic!("Reserved opcode 7 encountered"),
        }
    }

    #[must_use]
    pub fn literal(operand: Operand) -> u64 {
        operand as u64
    }
}

impl FromStr for Computer {
    type Err = ParseError;

    #[allow(clippy::many_single_char_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((registers, program)) = s.split_once("\n\n") else {
            return Err(ParseError::Register);
        };
        let mut registers = registers.lines();
        let a: u64 = parse_register(&mut registers);
        let b: u64 = parse_register(&mut registers);
        let c: u64 = parse_register(&mut registers);

        let Some((_, program)) = program.split_once(": ") else {
            return Err(ParseError::Program);
        };

        Ok(Computer::new(a, b, c, program))
    }
}

// FIXME: use an actual parser library like nom or winnow
fn parse_register(registers: &mut std::str::Lines<'_>) -> u64 {
    let e = ParseError::Register.to_string();
    registers
        .next()
        .expect(&e)
        .split_once(": ")
        .expect(&e)
        .1
        .parse()
        .expect(&e)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unable to parse register")]
    Register,
    #[error("Unable to parse program")]
    Program,
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str(&format!("Register A : {}\n", self.a));
        string.push_str(&format!("Register B : {}\n", self.b));
        string.push_str(&format!("Register C : {}\n", self.c));
        string.push_str(&format!("Pointer    : {}\n", self.p));
        string.push_str(&format!("Out        : {:?}", self.out));

        write!(f, "{string}")
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Instruction: {self:?}")
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Operand {
    op0 = 0,
    op1 = 1,
    op2 = 2,
    op3 = 3,
    op4 = 4,
    op5 = 5,
    op6 = 6,
    op7 = 7,
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Operand    : {self:?}")
    }
}

type Program = Vec<(Instruction, Operand)>;

use super::input;

use computation::{Computer, Instruction, Termination};

pub fn part_1() -> i32 {
    let program = input::read_lines(8)
        .iter()
        .map(|line| Instruction::from(line).expect("Invalid instruction in input file src/inputs/day_8.txt. Get original file from https://adventofcode.com/2020/day/8/input"))
        .collect();
    let mut computer = Computer::with(program);

    computer.run();
    computer.accumulator()
}

pub fn part_2() -> Option<i32> {
    let program = input::read_lines(8)
        .iter()
        .map(|line| Instruction::from(line).expect("Invalid instruction in input file src/inputs/day_8.txt. Get original file from https://adventofcode.com/2020/day/8/input"))
        .collect::<Vec<Instruction>>();
    let mut computer = Computer::new();

    // Try all possible alterations of swapping a jump with a noop instruction.
    for (i, instruction) in program.iter().enumerate() {
        let mut altered_program = program.clone();

        match instruction {
            Instruction::Jump(n) => altered_program[i] = Instruction::NoOperation(*n),
            Instruction::NoOperation(n) => altered_program[i] = Instruction::Jump(*n),
            Instruction::Accumulate(_) => continue,
        }

        computer.load(altered_program);
        computer.run();

        // If the alteration makes the program terminate normally, thats the correct fix.
        if computer.status() == Some(Termination::Normal) {
            return Some(computer.accumulator());
        }
    }

    None
}

mod computation {

    use std::convert::TryFrom;

    #[derive(Debug, Clone)]
    pub enum Instruction {
        Accumulate(i32),
        Jump(i32),
        NoOperation(i32),
    }

    impl Instruction {
        /// Parse instruction from strinig.
        ///
        /// An instruction has the form of
        /// ```
        ///     <instruction type> <integer value>
        /// ```
        /// where possible instruction types are `acc`, `jmp` and `nop`
        /// and integer values are always prefixed with either `+` or `-`.
        pub fn from(instruction: &str) -> Result<Instruction, &str> {
            match &instruction[..3] {
                "acc" => match instruction[4..].parse::<i32>() {
                    Ok(number) => return Ok(Instruction::Accumulate(number)),
                    Err(_) => return Err("Could not parse number from accumulate instruction."),
                },
                "jmp" => match instruction[4..].parse::<i32>() {
                    Ok(offset) => return Ok(Instruction::Jump(offset)),
                    Err(_) => return Err("Could not parse offset from jump instruction."),
                },
                "nop" => match instruction[4..].parse::<i32>() {
                    Ok(number) => return Ok(Instruction::NoOperation(number)),
                    Err(_) => return Err("Could not parse number from no operation instruction."),
                },
                _ => return Err("Unknown instruction. Must be acc, jmp or nop"),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum Termination {
        Normal,
        InfiniteLoop,
    }

    #[derive(Debug)]
    pub struct Computer {
        accumulator: i32,
        program: Vec<Instruction>,
        pointer: usize,
        status: Option<Termination>,
    }

    impl Computer {
        /// Create new computer.
        pub fn new() -> Computer {
            Computer {
                accumulator: 0,
                program: Vec::new(),
                pointer: 0,
                status: None,
            }
        }

        /// Create new computer with program already loaded.
        pub fn with(program: Vec<Instruction>) -> Computer {
            Computer {
                accumulator: 0,
                program,
                pointer: 0,
                status: None,
            }
        }

        /// Load a new program into the computer.
        ///
        /// Resets accumulator, program pointer and termination status.
        pub fn load(&mut self, program: Vec<Instruction>) {
            self.accumulator = 0;
            self.program = program;
            self.pointer = 0;
            self.status = None;
        }

        /// Execute a single instruction in the loaded program.
        ///
        /// The instruction pointer points to the instruction to be executed by this function.
        fn step(&mut self) {
            match self.program.get(self.pointer).expect("puzzles::day_8::computation::Computer tried to access invalid memory (instruction pointer out of bounds)") {
                // Add number to accumulator and increment instruction pointer.
                Instruction::Accumulate(number) => {
                    self.accumulator += number;
                    self.pointer += 1;
                },
                // Add offset to instruction pointer.
                Instruction::Jump(offset) => {
                    self.pointer = usize::try_from(self.pointer as i32 + offset).expect("Invalid jump. New instruction pointer out of bounds!");
                },
                // Do nothing and increment instrutcion pointer.
                Instruction::NoOperation(_) => {
                    self.pointer += 1;
                }
            }
        }

        /// Execute loaded program.
        ///
        /// The execution automatically halts when running into an infinite loop.
        /// Due to the limited instruction set this happens when executing an instruction for a second time.
        ///
        /// The instruction pointer going out of bounds of the program if considered normal termination,
        /// as the program simply runs out of instructions to execute.
        pub fn run(&mut self) -> Termination {
            let mut executed_instructions = vec![false; self.program.len()];

            // Watch if instructions in program get executed twice.
            // The pointer points to the next instruction to be executed.
            while !executed_instructions[self.pointer] {
                executed_instructions[self.pointer] = true;
                self.step();

                // Terminate execution if instruction pointer is outside of instructions.
                //
                // This represents normal termination, as the program runs out of
                // instructions to execute.
                if self.pointer >= self.program.len() {
                    self.status = Some(Termination::Normal);
                    return Termination::Normal;
                }
            }
            self.status = Some(Termination::InfiniteLoop);
            Termination::InfiniteLoop
        }

        /// Return the output of the program that has been previously run.
        pub fn accumulator(&self) -> i32 {
            self.accumulator
        }

        /// Return the termination status of the last program run.
        ///
        /// Returns None if the latest program has not been run.
        pub fn status(&self) -> Option<Termination> {
            self.status.clone()
        }
    }
}

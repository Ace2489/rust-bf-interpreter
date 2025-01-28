#[derive(Debug)]
struct Machine {
    instruction_pointer: usize,
    data: [u8; 10],
    data_pointer: usize,
    input: Vec<u8>,
    output: Vec<u8>,
}

impl Machine {
    fn run(&mut self, program: String) {
        let mut matched_brackets:u16 = 0;
        loop {
            let instruction = self.read_instruction(&program);
            if instruction == ';' {
                return;
            }

            if instruction == ']' && matched_brackets == 0 {
                panic!("An unpaired closing bracket(']') was found in the program at position {}", self.instruction_pointer)
            };

            if instruction == '[' {
                if matched_brackets == 0{
                    self.parse_for_closing_bracket(&program, &mut matched_brackets);
                }
                else{
                    matched_brackets -=1;
                }
            }
            match instruction {
                '>' => self.data_pointer += 1,
                '<' => self.data_pointer -= 1,
                '+' => self.data[self.data_pointer] += 1,
                '-' => self.data[self.data_pointer] -= 1,
                '.' => self.output.push(self.data[self.data_pointer]),
                ',' => {
                    let Some(element) = self.input.pop() else {
                        panic!("There is no input to read from")
                    };
                    self.data[self.data_pointer] = element
                }
                _ => {}
            };
            self.advance_instruction_pointer();
        }
    }
    #[inline]
    fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
    #[inline]
    fn parse_for_closing_bracket(&mut self, program: &String, matched_brackets:&mut u16) {
        let root = self.instruction_pointer;

        println!("parsing for closing brackets with an initial count of {}",matched_brackets);
        loop {
            self.advance_instruction_pointer();
            let instruction = self.read_instruction(program);
            if instruction == ';' {
                panic!("An unpaired opening bracket('[') was found in the program")
            }
            if instruction == '[' {self.parse_for_closing_bracket(program, matched_brackets)}
            if instruction == ']' {
                self.instruction_pointer = root;
                *matched_brackets +=1;
                break;
            }
        }
    }
    #[inline]
    fn read_instruction(&self, program: &String) -> char {
        let Some(instruction) = program.chars().nth(self.instruction_pointer) else {
            panic!("An error occured while reading the characters from the input")
        };
        return instruction;
    }
}

fn main() {
    let program = ">>+++-.,[[]++];".to_string();
    let input: Vec<u8> = vec![1, 2, 3];
    let output: Vec<u8> = Vec::new();

    let mut machine = Machine {
        instruction_pointer: 0 as usize,
        data: [0; 10],
        data_pointer: 0 as usize,
        input,
        output,
    };

    machine.run(program);
    // println!("{:#?}", machine)
}

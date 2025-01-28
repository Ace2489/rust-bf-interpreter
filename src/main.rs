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
        loop {
            let instruction = self.read_instruction(&program);

            if instruction == ';' {
                return;
            }

            // if instruction == ']' {panic!("An unpaired ']' was found in the program")};

            // if instruction == '['{
                
            // }
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
    fn parse_for_closing_bracket(&mut self){
        let root = self.instruction_pointer;
        loop{
            break;
        } 
    }
    #[inline]
    fn read_instruction(&self, program: &String)->char{
        let Some(instruction) = program.chars().nth(self.instruction_pointer) else {
            panic!("An error occured while reading the characters from the input")
        };
        return instruction;
    }
}

fn main() {
    let program = ">>+++-.,];".to_string();
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
    println!("{:#?}", machine)
}

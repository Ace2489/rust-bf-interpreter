struct Machine{
    instruction_pointer:usize,
    data: [u8;0x7530],
    data_pointer: usize,
    input: String,
    output: String

}

impl Machine{
    fn run(&self, program: String){
        let Some(instruction) = program.chars().nth(self.instruction_pointer) else {panic!("Something's wrong with the pointer for the instructions")};
        if instruction == ';'{return}
        match instruction{
            '>' =>{println!("The right angle is here with the data at {:?}", self.input.chars().nth(self.data_pointer))},
            _=>panic!("Something went wrong here")
        };
    }
}

fn main() {
    let program = "><+;".to_string();
    let input = String::from("in");
    let output = String::from("out");

    let machine = Machine{
        instruction_pointer: 0 as usize,
        data: [0;30000],
        data_pointer: 0 as usize,
        input,
        output
    };

    machine.run(program);


}


struct StdOut {
    first_element: bool,
    result: String,
}

impl StdOut {
    fn new() -> Self {
        StdOut {
            first_element: true,
            result: String::from(""),
        }
    }
    fn print(&mut self, value: u32) {
        if self.first_element {
            self.result.push_str(format!("{value}").as_str());
            self.first_element = false;
        } else {
            self.result.push_str(format!(",{value}").as_str());
        }
    }
}

enum Opcode {
    Adv(Operande),
    Bxl(Operande),
    Bst(Operande),
    Jnz(Operande),
    Bxc(),
    Out(Operande),
    Bdv(Operande),
    Cdv(Operande),
}

impl Opcode {
    fn evaluate(&self, register: &mut Register, stdout: &mut StdOut, pointer: usize) -> usize {
        match self {
            Opcode::Adv(operande) => {
                register.a /= 2_u32.pow(operande.combo(register));
            }
            Opcode::Bxl(operande) => {
                register.b ^= operande.literal();
            }
            Opcode::Bst(operande) => {
                register.b = operande.combo(register) % 8;
            }
            Opcode::Jnz(operande) => {
                if register.a != 0 {
                    return usize::try_from(operande.literal()).unwrap();
                }
            }
            Opcode::Bxc() => {
                register.b ^= register.c;
            }
            Opcode::Out(operande) => {
                let a = operande.combo(register);
                stdout.print(a % 8);
            }
            Opcode::Bdv(operande) => {
                register.b = register.a / 2_u32.pow(operande.combo(register));
            }
            Opcode::Cdv(operande) => {
                register.b = register.a / 2_u32.pow(operande.combo(register));
            }
        };
        pointer + 1
    }
}

impl From<(u32, u32)> for Opcode {
    fn from(value: (u32, u32)) -> Self {
        match value.0 {
            0 => Opcode::Adv(Operande::from(value.1)),
            1 => Opcode::Bxl(Operande::from(value.1)),
            2 => Opcode::Bst(Operande::from(value.1)),
            3 => Opcode::Jnz(Operande::from(value.1)),
            4 => Opcode::Bxc(),
            5 => Opcode::Out(Operande::from(value.1)),
            6 => Opcode::Bdv(Operande::from(value.1)),
            _ => Opcode::Cdv(Operande::from(value.1)),
        }
    }
}

enum Operande {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

impl Operande {
    fn literal(&self) -> u32 {
        match self {
            Operande::_0 => 0,
            Operande::_1 => 1,
            Operande::_2 => 2,
            Operande::_3 => 3,
            Operande::_4 => 4,
            Operande::_5 => 5,
            Operande::_6 => 6,
            Operande::_7 => 7,
        }
    }
    fn combo(&self, register: &Register) -> u32 {
        match self {
            Operande::_0 | Operande::_1 | Operande::_2 | Operande::_3 => self.literal(),
            Operande::_4 => register.a,
            Operande::_5 => register.b,
            Operande::_6 => register.c,
            Operande::_7 => todo!(),
        }
    }
}

impl From<u32> for Operande {
    fn from(value: u32) -> Self {
        match value {
            0 => Operande::_0,
            1 => Operande::_1,
            2 => Operande::_2,
            3 => Operande::_3,
            4 => Operande::_4,
            5 => Operande::_5,
            6 => Operande::_6,
            _ => Operande::_7,
        }
    }
}

struct Process<'a> {
    instructions: Vec<Opcode>,
    register: &'a mut Register,
    pointer: usize,
    stdout: StdOut,
}

impl<'a> Process<'a> {
    fn new(register: &'a mut Register, code: String) -> Self {
        let mut instructions: Vec<Opcode> = vec![];
        let code: Vec<&str> = code.split(",").collect();
        for element in code.chunks(2) {
            let opcode = Opcode::from((
                element[0].parse::<u32>().unwrap(),
                element[1].parse::<u32>().unwrap(),
            ));
            instructions.push(opcode);
        }

        Process {
            instructions,
            register,
            pointer: 0,
            stdout: StdOut::new(),
        }
    }

    fn run(mut self, a: u32, b: u32, c: u32) -> String {
        self.register.init(a, b, c);
        let iter = (&mut self).enumerate();
        for _ in iter {}
        self.stdout.result
    }
}

impl<'a> Iterator for Process<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.pointer);
        if let Some(opcode) = instruction {
            self.pointer = opcode.evaluate(self.register, &mut self.stdout, self.pointer);
            return Some(());
        }
        None
    }
}

struct Register {
    a: u32,
    b: u32,
    c: u32,
}

impl Register {
    fn new() -> Self {
        Register { a: 0, b: 0, c: 0 }
    }

    fn init(&mut self, a: u32, b: u32, c: u32) {
        self.a = a;
        self.b = b;
        self.c = c;
    }
}

fn main() {
    println!("Hello, world!");

    println!("end 6");
    let instructions = vec![(0, 1), (5, 4), (3, 0)];

    let instructions: Vec<Opcode> = instructions.into_iter().map(Opcode::from).collect();
    let mut register = Register::new();
    let process = Process {
        instructions,
        register: &mut register,
        pointer: 0,
        stdout: StdOut::new(),
    };
    let result = process.run(2024, 0, 0);
    println!("end7 {}", result);

    let process = Process::new(&mut register, String::from("0,1,5,4,3,0"));
    process.run(2024, 0, 0);
    println!("end8");
    let process = Process::new(&mut register, String::from("0,1,5,4,3,0"));
    let result = process.run(729, 0, 0);
    println!("{result}");

    // for i in 0..500_000_000 {
    //     let mut program = Programme::new(&mut register, String::from("0,1,5,4,3,0"));
    //     let result = program.run(i, 0, 0);
    //     if result == "0,1,5,4,3,0" {
    //         println!("{}", i);
    //     }
    //     if i % 1_000_000 == 0 {
    //         println!("{i}");
    //     }
    // }

    println!("end8");
    let program = Process::new(&mut register, String::from("4,6,3,5,6,3,5,2,1,0"));
    program.run(729, 0, 0);

    println!("fin");
}

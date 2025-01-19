#[derive(Debug)]
pub struct Machine {
    a: u64,
    b: u64,
    c: u64,
    inst_pointer: usize,
    output: Vec<u32>
}
impl Machine {
    pub fn new(a:u64, b:u64, c:u64) -> Machine {
        Machine {
            a, b, c,
            inst_pointer: 0,
            output: vec![]
        }
    }
    pub fn run(&mut self, program:&[u32]) -> Vec<u32> {
        loop {
            if self.inst_pointer >= program.len() {
                break;
            }
            let inst = program[self.inst_pointer] as usize;
            let arg =  program[self.inst_pointer + 1];
            match inst {
                0 => self.adv(arg),
                1 => self.bxl(arg),
                2 => self.bst(arg),
                3 => self.jnz(arg),
                4 => self.bxc(arg),
                5 => self.out(arg),
                6 => self.bdv(arg),
                7 => self.cdv(arg),
                _ => {}
            }
        }
        self.output.clone()
    }

    fn combo(&self, n:u32) -> u32 {
        if n <= 3 {
            n
        } else {
            [self.a, self.b, self.c][(n - 4) as usize] as u32
        }
    }
    fn adv(&mut self, n:u32) {
        self.a /= 2_u64.pow(self.combo(n));
        self.inst_pointer += 2;
    }
    fn bxl(&mut self, n:u32) {
        self.b ^= n as u64;
        self.inst_pointer += 2;
    }
    fn bst(&mut self, n:u32){
        self.b = (self.combo(n) % 8).into();
        self.inst_pointer += 2;
    }
    fn jnz(&mut self, n:u32) {
        if self.a != 0 {
            self.inst_pointer = n as usize;
        } else {
            self.inst_pointer += 2;
        }
    }
    fn bxc(&mut self, _n:u32) {
        self.b ^= self.c;
        self.inst_pointer += 2;
    }

    fn out(&mut self, n:u32) {
        self.output.push(self.combo(n) % 8);
        self.inst_pointer += 2;
    }

    fn bdv(&mut self, n:u32) {
        self.b = self.a / (2_u64.pow(self.combo(n)));
        self.inst_pointer += 2;
    }

    fn cdv(&mut self, n:u32) {
        self.c = self.a / (2_u64.pow(self.combo(n)));
        self.inst_pointer += 2;
    }
}
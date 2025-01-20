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

    /*
    The program is loop of 8 instructions. Each loop reduces 
    register 8 by half and outputs a digit. This means for 
    each three bit value in the output register A is three bits
    larger. We can solve by working backward and finding canditates
    for each three bits of register A that output the correct 
    corresponding output. 
    */
    pub fn search(program: &[u32]) -> Option<u64> {
        let mut current = vec![0];
        let mut next = Vec::new();

        for i in (0..program.len()).rev() {
            next.clear();    
            for &partial in &current {
                for d in 0..8 {
                    let candidate = (partial << 3) + d ;
                    if Machine::new(candidate, 0, 0).run(program) == program[i..] {
                        next.push(candidate);
                    }
                }
            }
            std::mem::swap(&mut current, &mut next);
        }
        current.into_iter().min()
    }

}
use aoc::get_input;

struct VM {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,

    pc: usize,
    instructions: Vec<u8>,
}

impl VM {
    fn get_combo_op(&self, op: i64) -> i64 {
        match op {
            0..=3 => op,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid combo op {op}"),
        }
    }

    fn adv(&mut self, op: i64) {
        self.reg_a /= 2i64.pow(self.get_combo_op(op) as u32);
        self.pc += 2;
    }
    fn bxl(&mut self, op: i64) {
        self.reg_b ^= op;
        self.pc += 2;
    }

    fn bst(&mut self, op: i64) {
        self.reg_b = self.get_combo_op(op) % 8;
        self.pc += 2;
    }

    fn jnz(&mut self, op: i64) {
        if self.reg_a != 0 {
            self.pc = op.try_into().unwrap();
        } else {
            self.pc += 2;
        }
    }

    fn bxc(&mut self, _op: i64) {
        self.reg_b ^= self.reg_c;
        self.pc += 2;
    }

    fn out(&mut self, op: i64) {
        print!("{},", self.get_combo_op(op) % 8);
        self.pc += 2;
    }

    fn bdv(&mut self, op: i64) {
        self.reg_b = self.reg_a / 2i64.pow(self.get_combo_op(op) as u32);
        self.pc += 2;
    }
    fn cdv(&mut self, op: i64) {
        self.reg_c = self.reg_a / 2i64.pow(self.get_combo_op(op) as u32);
        self.pc += 2;
    }

    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc];
            let op: i64 = self.instructions[self.pc + 1].into();

            match instruction {
                0 => self.adv(op),
                1 => self.bxl(op),
                2 => self.bst(op),
                3 => self.jnz(op),
                4 => self.bxc(op),
                5 => self.out(op),
                6 => self.bdv(op),
                7 => self.cdv(op),
                _ => panic!("invalid instruction {instruction}"),
            };
        }
    }
}

fn solve(input: String) -> i64 {
    let mut lines = input.lines();
    let reg_a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();
    let reg_b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<i64>()
        .unwrap();
    lines.next();
    let instructions: Vec<_> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|el| el.as_bytes()[0] - b'0')
        .collect();
    let mut vm = VM {
        reg_a,
        reg_b,
        reg_c,
        pc: 0,
        instructions,
    };
    vm.run();

    0
}

fn main() {
    solve(get_input());
}

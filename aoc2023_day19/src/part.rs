#[derive(Clone, Copy, Debug)]
pub struct Part {
    pub x: u16,
    pub m: u16,
    pub a: u16,
    pub s: u16,
}

impl Part {
    pub fn total_rating(&self) -> u128 {
        self.x as u128 + self.m as u128 + self.a as u128 + self.s as u128
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = &value[1..value.len() - 1];
        let mut s = value.split(',');
        let x = parse_num(s.next().unwrap());
        let m = parse_num(s.next().unwrap());
        let a = parse_num(s.next().unwrap());
        let s = parse_num(s.next().unwrap());

        Self { x, m, a, s }
    }
}

fn parse_num(s: &str) -> u16 {
    str::parse(&s[2..]).unwrap()
}

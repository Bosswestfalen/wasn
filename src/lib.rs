pub mod operators;

pub struct Equation {
    a: f64,
    b: f64,
    op: char,
}


impl Equation {
    pub fn build(a: f64, op: char, b: f64) -> Self {
        Equation{a, b, op}
    }

    pub fn calc(&self) -> f64 {
        match self.op {
            '+' => self.a + self.b,
            _ => panic!("Must not happen"),
        }
    }
}

pub mod part1;
pub mod part2;

#[derive(Debug)]
pub enum Operator {
    Add,
    Mult,
}

impl Operator {
    fn eval(&self, value1: u64, value2: u64) -> u64 {
        match self {
            Self::Add => value1 + value2,
            Self::Mult => value1 * value2,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Value(u64),
    Operation(Operator, Box<Expression>, Box<Expression>),
}

impl Expression {
    fn eval(&self) -> u64 {
        match self {
            Self::Value(value) => *value,
            Self::Operation(operator, expression1, expression2) => {
                operator.eval(expression1.eval(), expression2.eval())
            }
        }
    }
}

pub fn sum_all(expressions: &[Expression]) -> u64 {
    expressions.iter().map(|expr| expr.eval()).sum()
}

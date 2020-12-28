use std::collections::VecDeque;

pub fn part1(expressions: &[String]) -> Result<u64, String> {
    sum_all(expressions, true)
}

pub fn part2(expressions: &[String]) -> Result<u64, String> {
    sum_all(expressions, false)
}

fn sum_all(
    expressions: &[String],
    same_precedence: bool,
) -> Result<u64, String> {
    let mut sum = 0;
    for expression in expressions {
        sum += parse(expression, same_precedence)
            .map_err(|err| {
                format!("Failed to parse expression '{}': {}", expression, err)
            })?
            .eval()
    }
    Ok(sum)
}

#[derive(PartialEq)]
enum Operator {
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

enum Expression {
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

#[derive(PartialEq)]
enum Token {
    Operator(Operator),
    LeftParenthesis,
}

fn parse(s: &str, same_precedence: bool) -> Result<Expression, String> {
    let mut output: VecDeque<Expression> = VecDeque::new();
    let mut stack: VecDeque<Token> = VecDeque::new();

    for (ch, pos) in s.chars().zip(1..).filter(|(c, _)| !c.is_whitespace()) {
        match ch {
            '+' => {
                if same_precedence {
                    process_stack(&mut stack, &mut output)?;
                }
                stack.push_back(Token::Operator(Operator::Add));
            }
            '*' => {
                process_stack(&mut stack, &mut output)?;
                stack.push_back(Token::Operator(Operator::Mult));
            }
            '(' => {
                stack.push_back(Token::LeftParenthesis);
            }
            ')' => {
                process_stack(&mut stack, &mut output)?;
                if stack.pop_back() != Some(Token::LeftParenthesis) {
                    return Err(format!(
                        "Unbalanced parenthesis at position {}",
                        pos
                    ));
                }
            }
            d if d.is_digit(10) => {
                let digit = d.to_digit(10).unwrap();
                output.push_back(Expression::Value(digit.into()));
            }
            _ => {
                return Err(format!(
                    "Unexpected character '{}' at position {}",
                    ch, pos
                ))
            }
        }
    }
    process_stack(&mut stack, &mut output)?;

    match (output.pop_back(), output.pop_back(), stack.pop_back()) {
        (Some(expr), None, None) => Ok(expr),
        _ => Err("Invalid expression".to_string()),
    }
}

fn process_stack(
    stack: &mut VecDeque<Token>,
    output: &mut VecDeque<Expression>,
) -> Result<(), String> {
    while let Some(token) = stack.pop_back() {
        match token {
            Token::Operator(oper) => {
                match (output.pop_back(), output.pop_back()) {
                    (Some(expr2), Some(expr1)) => {
                        output.push_back(Expression::Operation(
                            oper,
                            Box::new(expr1),
                            Box::new(expr2),
                        ));
                    }
                    _ => {
                        return Err("Invalid expression".to_string());
                    }
                }
            }
            Token::LeftParenthesis => {
                stack.push_back(token);
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        let tests = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12_240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13_632),
        ];
        for (expr, result) in tests.iter() {
            assert_eq!(parse(expr, true).unwrap().eval(), *result);
        }
    }

    #[test]
    fn part2_examples() {
        let tests = [
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1_445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669_060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23_340),
        ];
        for (expr, result) in tests.iter() {
            assert_eq!(parse(expr, false).unwrap().eval(), *result);
        }
    }

    #[test]
    fn part2_more() {
        let tests = [
            ("1 + (2 * 3)", 7),
            ("1 + (2 * 3 * 4)", 25),
            ("2 * (1 * 3) + 4", 14),
            ("2 * (2 * 3) + (2 * 4)", 28),
            ("(9 + 4 * 9 * 4) + 3 + 7 * 8", 3_824),
            ("6 + ((8 + 2) * (2 * 6 * 9 * 6 * 5)) + 5", 32_411),
            ("4 + 9 * 2 + (2 + (6 * 5))", 442),
            ("9 * 2 + ((6 * 5) + 3)", 315),
            ("9 * 2 + (2 + (6 * 5) + 3)", 333),
            ("4 + 9 * 2 + (2 + (6 * 5) + 3)", 481),
            ("4 + 9 * 2 + (2 + (6 * 5) + 3 * 5)", 2_301),
            ("4 + 9 * 2 + (2 + (6 * 5) + 3 * 4 + 6)", 4_576),
            ("4 + 9 * 2 + (2 + (6 * 5 * 6) + 3 * 4 + 6)", 24_076),
        ];
        for (expr, result) in tests.iter() {
            assert_eq!(parse(expr, false).unwrap().eval(), *result);
        }
    }
}

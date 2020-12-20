use super::*;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
enum Expect {
    LeftExpr,
    Operator,
    RightExpr,
}

pub fn parse_expression_v2(s: &str) -> Result<Expression, String> {
    let mut operand: VecDeque<Expression> = VecDeque::new();
    let mut operator: VecDeque<Operator> = VecDeque::new();
    let mut expect: VecDeque<Expect> = VecDeque::new();
    let mut pending: VecDeque<usize> = VecDeque::new();
    let mut pending_ops = 0;

    expect.push_back(Expect::LeftExpr);

    for (ch, pos) in s.chars().zip(1..).filter(|(c, _)| !c.is_whitespace()) {
        match (ch, expect.pop_back()) {
            ('+', Some(Expect::Operator)) => {
                operator.push_back(Operator::Add);
                expect.push_back(Expect::RightExpr);
            }
            ('*', Some(Expect::Operator)) => {
                operator.push_back(Operator::Mult);
                expect.push_back(Expect::LeftExpr);
                pending_ops += 1;
            }
            ('(', Some(Expect::LeftExpr)) => {
                expect.push_back(Expect::LeftExpr);
                expect.push_back(Expect::LeftExpr);
                pending.push_back(pending_ops);
                pending_ops = 0;
            }
            ('(', Some(Expect::RightExpr)) => {
                expect.push_back(Expect::RightExpr);
                expect.push_back(Expect::LeftExpr);
                pending.push_back(pending_ops);
                pending_ops = 0;
            }
            (')', Some(Expect::Operator)) => {
                for _ in 0..pending_ops {
                    let right = operand.pop_back().unwrap();
                    let left = operand.pop_back().unwrap();
                    let expr = Expression::Operation(
                        operator.pop_back().unwrap(),
                        Box::new(left),
                        Box::new(right),
                    );
                    operand.push_back(expr);
                }
                match expect.pop_back() {
                    Some(Expect::LeftExpr) => {
                    }
                    Some(Expect::RightExpr) => {
                    let right = operand.pop_back().unwrap();
                    let left = operand.pop_back().unwrap();
                    let expr = Expression::Operation(
                        operator.pop_back().unwrap(),
                        Box::new(left),
                        Box::new(right),
                    );
                    operand.push_back(expr);
                    }
                    _ => panic!("shite!"),
                }
                expect.push_back(Expect::Operator);
                pending_ops = pending.pop_back().unwrap();
            },
            (d, Some(Expect::LeftExpr)) if d.is_digit(10) => {
                let digit = d.to_digit(10).unwrap();
                operand.push_back(Expression::Value(digit.into()));
                expect.push_back(Expect::Operator);
            }
            (d, Some(Expect::RightExpr)) if d.is_digit(10) => {
                let left = operand.pop_back().unwrap();
                let digit = d.to_digit(10).unwrap();
                let expr = Expression::Operation(
                    operator.pop_back().unwrap(),
                    Box::new(left),
                    Box::new(Expression::Value(digit.into())),
                );
                operand.push_back(expr);
                expect.push_back(Expect::Operator);
            }
            (ch, _) => {
                return Err(format!(
                    "Unexpected character '{}' at position {}",
                    ch, pos
                ))
            }
        }
    }

    match expect.pop_back() {
        Some(Expect::Operator) => (),
        _ => panic!("Shite"),
    };

    // println!("operand stack: {:?}", operand);
    // println!("operator stack: {:?}", operator);
    // println!("expect stack: {:?}", expect);

    for _ in 0..pending_ops {
        let right = operand.pop_back().unwrap();
        let left = operand.pop_back().unwrap();
        let expr = Expression::Operation(
            operator.pop_back().unwrap(),
            Box::new(left),
            Box::new(right),
        );
        operand.push_back(expr);
    }

    assert!(expect.is_empty(), "expression not fully evaluated");
    assert!(pending.is_empty(), "expression not fully evaluated");
    assert!(operator.is_empty(), "operator(s) not fully evaluated");
    assert_eq!(operand.len(), 1, "operand(s) not fully evaluated");

    operand
        .pop_back()
        .ok_or_else(|| "Invalid expression".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_examples() {
        let examples = [
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1_445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669_060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23_340),
        ];
        for (expr, result) in examples.iter() {
            assert_eq!(parse_expression_v2(expr).unwrap().eval(), *result);
        }
    }

    #[test]
    fn part2_other() {
        let examples = [
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
        for (expr, result) in examples.iter() {
            assert_eq!(parse_expression_v2(expr).unwrap().eval(), *result);
        }
    }
}

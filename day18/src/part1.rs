use super::*;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
enum Expect {
    LeftExpr,
    Operator,
    RightExpr,
}

pub fn parse_expression_v1(s: &str) -> Result<Expression, String> {
    let mut operand: VecDeque<Expression> = VecDeque::new();
    let mut operator: VecDeque<Operator> = VecDeque::new();
    let mut expect: VecDeque<Expect> = VecDeque::new();

    expect.push_back(Expect::LeftExpr);

    for (ch, pos) in s.chars().zip(1..).filter(|(c, _)| !c.is_whitespace()) {
        match (ch, expect.pop_back()) {
            ('+', Some(Expect::Operator)) => {
                operator.push_back(Operator::Add);
                expect.push_back(Expect::RightExpr);
            }
            ('*', Some(Expect::Operator)) => {
                operator.push_back(Operator::Mult);
                expect.push_back(Expect::RightExpr);
            }
            ('(', Some(Expect::LeftExpr)) => {
                expect.push_back(Expect::LeftExpr);
                expect.push_back(Expect::LeftExpr);
            }
            ('(', Some(Expect::RightExpr)) => {
                expect.push_back(Expect::RightExpr);
                expect.push_back(Expect::LeftExpr);
            }
            (')', Some(Expect::Operator)) => match expect.pop_back() {
                Some(Expect::LeftExpr) => {
                    expect.push_back(Expect::Operator);
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
                    expect.push_back(Expect::Operator);
                }
                _ => {
                    return Err(format!(
                        "Unmatched parentheses at position {}",
                        pos
                    ))
                }
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

    assert_eq!(
        (expect.pop_back(), expect.pop_back()),
        (Some(Expect::Operator), None),
        "expression not fully evaluated"
    );
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
    fn part1_examples() {
        let examples = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12_240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13_632),
        ];
        for (expr, result) in examples.iter() {
            assert_eq!(parse_expression_v1(expr).unwrap().eval(), *result);
        }
    }
}

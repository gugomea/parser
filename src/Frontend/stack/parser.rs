use std::collections::VecDeque;

use crate::Frontend::{error::*, tokens::*};

#[derive(Debug, PartialEq, Eq)]
enum ExpressionToken {
    E(Expression),
    Union,
}

pub fn parse(input: &str) -> Result<Expression, ParsingError> {
    let mut it = input.chars().enumerate();

    //The stack where we are going to store the expressions while constructing them,
    //when the process ends, this should only have one expression, the final one.
    let mut expressions = vec![];

    //we have this, so we can keep trak of how many expressions there are on each level,
    //this helps when we want to unroll the unions to create the Vec<union>, since we have
    //to crete one expression for each element on a union.
    //Example => a|b|c|d.   number_of_expressions = vec[4]
    let mut number_of_expressions = vec![0_usize];

    while let Some((idx, current)) = it.next() {
        match current {
            '[' => {
                return Err(ParsingError::new(("Error Construyendo rango").into(), ErrorType::range, idx));
            }

            '|' => expressions.push(ExpressionToken::Union),

            quantifier @ ('*' | '+' | '?') => {
                let new_exp = match expressions.pop() {
                    Some(ExpressionToken::E(Expression::optional(_))) | Some(ExpressionToken::E(Expression::one_or_more(_))) | Some(ExpressionToken::E(Expression::zero_or_more(_))) => return Err(ParsingError::new("no puedes poner dos cuantificadores seguidos".into(), ErrorType::unexpected, idx)),
                    Some(ExpressionToken::E(exp)) => match quantifier {
                        '*' => Expression::zero_or_more(Box::new(exp)),
                        '+' => Expression::one_or_more(Box::new(exp)),
                        '?' => Expression::optional(Box::new(exp)),
                        _ => unreachable!(),
                    }
                    None => return Err(ParsingError::new("Cuantificador sobre expresión vacía".into(), ErrorType::unexpected, idx)),
                    _ => return Err(ParsingError::new("una expresión fue esperada antes".into(), ErrorType::unexpected, idx)),
                };
                expressions.push(ExpressionToken::E(new_exp));
            }

            '(' => {
                match number_of_expressions.last_mut() {
                    Some(n) => *n += 1,
                    None => return Err(ParsingError::new("Unión encontrada en sitio inesperado".into(), ErrorType::union, idx)),
                }
                number_of_expressions.push(0);
            }

            ')' => {
                unroll_expressions(&mut expressions, &mut number_of_expressions)?;
                if let Some(ExpressionToken::E(exp)) = expressions.pop() {
                    expressions.push(ExpressionToken::E(Expression::group(Box::new(exp))));
                }
            }

            ch => {
                let ch_value = match ch {
                    '\\' => match it.next() {
                        Some((_, car)) => Literal::atom(car),
                        None => return Err(ParsingError::new("Se esperaba otro caracter".into(), ErrorType::union, idx)),
                    }
                    '.' => Literal::anyLiteral,
                    other => Literal::atom(other),
                };
                match number_of_expressions.last_mut() {
                    Some(n) => *n += 1,
                    None => return Err(ParsingError::new("Unión encontrada en sitio inesperado".into(), ErrorType::union, idx)),
                }
                expressions.push(ExpressionToken::E(Expression::l(ch_value)));
            }
        }
    };

    match unroll_expressions(&mut expressions, &mut number_of_expressions) {
        Ok(()) => match expressions.pop() {
            Some(ExpressionToken::E(exp)) => Ok(exp),
            _ => Err(ParsingError::new("Se esperaba al menos, una expresión".into(), ErrorType::unexpected, 0)),
        }
        Err(e) if e.typ() == ErrorType::emptyExpression => Ok(Expression::empty),
        Err(other) => Err(other),
    }
}

fn unroll_expressions(expressions: &mut Vec<ExpressionToken>, depth: &mut Vec<usize>) -> Result<(), ParsingError> {
    let mut n = match depth.pop() {
        Some(0) => return Err(ParsingError::new("Expresión vacía".into(), ErrorType::emptyExpression, 0)),
        Some(value) => value,
        None => return Err(ParsingError::new("se esperaba otra expresión".into(), ErrorType::unexpected, 0)),
    };
    let mut finale = VecDeque::from(vec![VecDeque::new()]);
    while n != 0 {
        match expressions.pop() {
            Some(ExpressionToken::E(expr)) => {
                if let Some(v) = finale.get_mut(0) { v.push_front(expr); }
                n -= 1;
            }
            Some(ExpressionToken::Union) => finale.push_front(VecDeque::new()),
            None => return Err(ParsingError::new("No quedan expresiones".into(), ErrorType::unexpected, 0)),
        }
    }
    let mut finale = finale.into_iter().map(|mut x| match x.len() {
        1 => x.pop_front().unwrap(),
        _ => Expression::concatenation(x.into())
    }).collect::<VecDeque<Expression>>();
    if finale.len() == 1 {
        let exp = ExpressionToken::E(finale.pop_front().unwrap());
        expressions.push(exp);
    } else if finale.len() > 1 {
        let unions = finale.into_iter().collect();
        expressions.push(ExpressionToken::E(Expression::union(unions)));
    }
    Ok(())
}

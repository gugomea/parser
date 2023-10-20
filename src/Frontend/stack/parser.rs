use std::collections::VecDeque;

use crate::Frontend::{error::*, tokens::*};

#[derive(Debug)]
enum ExpressionToken {
    E(Expression),
    Union,
    empty,
}

pub fn parse(input: &str) -> Result<Expression, ParsingError> {
    let mut it = input.chars().enumerate();

    //The stack where we are going to store the expressions while constructing them,
    //when the process ends, this should only have one expression, the final one.
    let mut expressions = vec![ExpressionToken::empty];

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

            '|' => {
                expressions.push(ExpressionToken::Union);
                expressions.push(ExpressionToken::empty);
            }

            _quantifier @ ('*' | '+' | '?') => {
                return Err(ParsingError::new("Añadiendo cuantificadores".into(), ErrorType::unexpected, idx));
            }

            '(' => {
                match number_of_expressions.last_mut() {
                    Some(n) => *n+= 1,
                    None => return Err(ParsingError::new("Unión encontrada en sitio inesperado".into(), ErrorType::union, idx)),
                }

                number_of_expressions.push(0);
                if let Some(ExpressionToken::empty) = expressions.last() {
                    continue;
                } else  {
                    expressions.push(ExpressionToken::empty); 
                }
            }

            ')' => {
                unroll_expressions(&mut expressions, &mut number_of_expressions)?;
                if let Some(ExpressionToken::E(exp)) = expressions.pop() {
                    expressions.push(ExpressionToken::E(Expression::group(Box::new(exp))));
                }

                if let Some(ExpressionToken::empty) = expressions.last() {
                    continue;
                } else  {
                    expressions.push(ExpressionToken::empty); 
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
                let updated_expression = match expressions.pop() {
                    Some(ExpressionToken::empty) => {
                        if let Some(value) = number_of_expressions.last_mut() { *value += 1; }
                        Expression::concatenation(vec![Expression::l(ch_value)])
                    }
                    Some(ExpressionToken::E(Expression::concatenation(mut con))) => {
                        con.push(Expression::l(ch_value));
                        Expression::concatenation(con)
                    }
                    Some(ExpressionToken::E(other)) => Expression::concatenation(vec![other, Expression::l(ch_value)]),
                    _ => return Err(ParsingError::new("Una expresión anterior fue esperada".into(), ErrorType::unexpected, idx)),
                };
                expressions.push(ExpressionToken::E(updated_expression));
            }
        }
    };

    unroll_expressions(&mut expressions, &mut number_of_expressions)?;
    match expressions.pop() {
        Some(ExpressionToken::E(exp)) => Ok(exp),
        _ => Err(ParsingError::new("Se esperaba al menos, una expresión".into(), ErrorType::unexpected, 0)),
    }
}

fn unroll_expressions(expressions: &mut Vec<ExpressionToken>, depth: &mut Vec<usize>) -> Result<(), ParsingError> {
    let mut n = match depth.pop() {
        Some(value) => value,
        None => return Err(ParsingError::new("se esperaba otra expresión".into(), ErrorType::unexpected, 0)),
    };
    let mut finale = VecDeque::from(vec![Expression::concatenation(vec![])]);
    while n != 0 {
        match expressions.pop() {
            Some(ExpressionToken::E(expr)) => {
                if let Some(Expression::concatenation(mut v)) = finale.pop_front() {
                    let mut flatten_concat = match expr {
                        Expression::concatenation(v) => v,
                        other => vec![other],
                    };
                    flatten_concat.append(&mut v);
                    finale.push_front(Expression::concatenation(flatten_concat));
                }
                n -= 1;
            }
            Some(ExpressionToken::Union) => finale.push_front(Expression::concatenation(vec![])),
            Some(ExpressionToken::empty) => {}
            _ => return Err(ParsingError::new("No quedan expresiones".into(), ErrorType::unexpected, 0)),
        }
    }
    if finale.len() == 1 {
        let exp = match finale.pop_front().unwrap() {
            Expression::concatenation(mut v) if v.len() == 1 => ExpressionToken::E(v.pop().unwrap()),
            other => ExpressionToken::E(other),
        };
        expressions.push(exp);
    } else if finale.len() > 1 {
        expressions.push(ExpressionToken::E(Expression::union(finale.into())));
    }
    Ok(())
}

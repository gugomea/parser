use crate::Frontend::{error::*, tokens::*};
use std::str::Chars;
use std::iter::Enumerate;

pub fn parse_recursivo(input: &str) -> Result<Expression, ParsingError> {
    let mut it = input.chars().enumerate();
    return parse_rec(&mut it);
}

fn parse_rec(it: &mut Enumerate<Chars>) -> Result<Expression, ParsingError> {
    //Cada componente del vector se corresponde a cada expresión que queda separada por las uniones.
    //Por ejemplo: abc|(efg) => expresiones = [ Concatenacion(a, b, c), Grupo(Concatenacion(e, f, g)) ]
    let mut expresiones: Vec<Vec<Expression>> = vec![vec![]];

    //while(true)
    loop {
        // el iterador es de la forma (índice, caracter)
        match it.next() {
            // si me encuentro un grupo, lo calculo(recursivamente) y lo concateno con mi última expresión.
            Some((i, '(')) => {
                // last_mut me da una referencia mutable a la cima de la pila, y como en rust no
                // hay nulos devuelve un Option<T>, por si la pila estaba vacía.
                if let Some(exp) = expresiones.last_mut() {
                    exp.push(Expression::group(Box::new(parse_rec(it)?)));
                } else {
                    return Err(ParsingError::new(format!("error: {}", i), ErrorType::group, i));
                }
            }
            //Si me encuentro un cierre de paréntesis o he acabado la string, ya he terminado de procesar la expresión, así que lo devuelvo.
            Some((_, ')')) | None => {

                //Por ahora tenemos un Vec<Vec<Expression>>, pero necesitamos un Vec<Expression>
                //Entonces cada componente del vector(que es del tipo Vec<Expression>) la transformamos en una concatenación, si su
                //longitud es > 1. Si la longitud es 1, simplemente dejamos esa expresión por si
                //sola(para evitar concatenaciones de una componente)
                let mut aux = expresiones.into_iter().map(|mut x| match x.len() {
                    1 => x.pop().unwrap(),
                    _ => Expression::concatenation(x),
                }).collect::<Vec<Expression>>();

                // Si después de procesar el Vector tenemos una expresión, significa que no ha
                // habido uniones, así que devolvemos la primera(y ultima) expresión.
                // Sino, devolvemos la union del vector de expresiones.
                if aux.len() == 1 {
                    return Ok(aux.pop().unwrap());
                } else {
                    return Ok(Expression::union(aux));
                }
            }

            //La anterior expresión ya la hemos terminado, porque queda separada por la unión,
            //así que apilamos un vec<Expression> vacío.
            Some((_, '|')) => expresiones.push(vec![]),

            Some((i, quantifier @ ('*' | '+' | '?'))) => {
                match expresiones.last_mut() {
                    Some(actual) => {
                        let actualizado = match actual.pop() {
                            Some(ultimo) => match quantifier {
                                '*' => Expression::zero_or_more(Box::new(ultimo)),
                                '+' => Expression::one_or_more(Box::new(ultimo)),
                                '?' => Expression::optional(Box::new(ultimo)),
                                _ => unreachable!(),
                            }
                            None => return Err(ParsingError::new("Cuantificador sobre expresión vacía".into(), ErrorType::unexpected, i)),
                        };
                        actual.push(actualizado);
                    }
                    None => return Err(ParsingError::new("Cuantificador sobre expresión vacía".into(), ErrorType::unexpected, i)),
                };
            }

            //procesar un caracter
            Some((i, ch)) => {
                //añadimos a la última expresión el átomo.
                if let Some(exp) = expresiones.last_mut() {
                    exp.push(Expression::l(Literal::atom(ch)));
                } else {
                    return Err(ParsingError::new(format!("error: {}", i), ErrorType::unexpected, i));
                }
            }
        }
    }
}


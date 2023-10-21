use ExpresionesRegulares::Frontend::*;

#[test]
fn test() {
    let input = "a((d|ef|gh))j";
    let ast_regex = stack::parser::parse(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex);

    let input = "a?|b+c*d|(e)+";
    let ast_regex = stack::parser::parse(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex);

    let input = "a|";
    let ast_regex = stack::parser::parse(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex);

    let input = "a(";
    let ast_regex = stack::parser::parse(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex);
}

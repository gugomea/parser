use ExpresionesRegulares::Frontend::*;

#[test]
fn test() {
    let input = "a((d|ef|gh))j|k";
    //let ast_regex = stack::parser::parse(input);
    //println!("input: {}", input);
    //println!("{:#?}", ast_regex);


    let input = "a?|b+c*d|(e)+";
    //let ast_regex = stack::parser::parse(input);
    //println!("input: {}", input);
    //println!("{:#?}", ast_regex);

    let ast_regex_recursivo = stack::parser_recursivo::parse_recursivo(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex_recursivo);

    //let input = "(a|)";
    //let ast_regex = stack::parser::parse(input);
    //println!("input: {}", input);
    //println!("{:#?}", ast_regex);

    //let input = "a(a()))";
    //let ast_regex = stack::parser::parse(input);
    //println!("input: {}", input);
    //println!("{:#?}", ast_regex);

    //let input = "a|(b|)";
    //let ast_regex = stack::parser::parse(input);
    //println!("input: {}", input);
    //println!("{:#?}", ast_regex);
}

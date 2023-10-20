use ExpresionesRegulares::Frontend::*;

#[test]
fn test() {
    //let input = "a(b)((c))";
    //let ast_regex = stack::parser::parse("Hello, World!");
    let input = "ab(c((d|ef|gh))i)j";
    let ast_regex = stack::parser::parse(input);
    println!("input: {}", input);
    println!("{:#?}", ast_regex);
}

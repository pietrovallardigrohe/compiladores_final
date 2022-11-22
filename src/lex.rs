use std::collections::HashSet;

use pest::Parser;

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)]
#[grammar = "Grammar/Lexer.pest"]
pub struct Lexer;

/*
 * Token -> struct
 * input -> raw input parsed
 * line -> line of the token
 * col -> start column of the token
 * rule -> Rule that accepted the input
 */
struct Token {
    input: &str,
    line: usize,
    col: usize,
    rule: Rule
}

impl Token {
   fn new(&str, usize, usize, Rule) {
       todo!();
   }
}

/*
 * Removes newlines and apprends the line count to it 
 *
 * Raw Input
 *
 * if(a == 0) {
 *
 *     int b = 1;
 *      
 * } else {
 *
 *     float c = 1.0;
 *     // a
 * }
 * 
 * Precompilation
 * 
 * [1]if(a == 0) {[3]int b = 1;[5]} else {[7]float c = 1.0;[8]// a[9]}$end$
 */
fn precompile(input: &str) -> String {
    let mut result = String::new();

    input.lines().enumerate()
        .filter(|(_, e)| !e.trim().is_empty() || !e.starts_with("//"))
        .for_each(|(index, element)| result += &format!("[{}]{}", (index+1).to_string(), element.trim()));
    result.push_str("$end$");

    result
}

/*
 * Identifica os tokens e formata o resultado para a análize sintática
 * [1]if(a == 0) {[3]int b = 1;[5]}$end$
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES END
 * Retorna um vetor contendo os tokens reconhecidos e outro contendo os erros
 */
pub fn get_tokens(input: &str) -> (Vec<Token>, Vec<Token>) {
    todo!()
    let mut current_line: usize = 0;
    
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<Token> = vec![];

    let precompiled_input: String = precompile(input);
    println!("{}", precompiled_input);

    // Faz o parsing da entrada e formatação de cada token
    match Lexer::parse(Rule::TOKEN, &precompiled_input) {
        Ok(pairs) => {
            for pair in pairs {
                let _token = Token::new();
                match &pair.as_rule() {
                    Rule::LINE => {
                        if let Ok(line) = pair.as_str().trim_matches(|c| c == '[' || c == ']').parse::<usize>() {
                            current_line = line;
                        } else {
                            println!("UNEXPECTED LINE PARSING BEHAVIOR");
                        } 
                    },
                    Rule::ERROR => {
                        result.push_str(&format!("ERROR ")); 
                        errors.insert(current_line);
                    },
                    _ => {
                        result.push_str(&format!("{current_line} {:?} ", pair.as_rule()));
                    }
                }
            }
        },
        Err(err) => println!("{err}") 
    } 

    // println!("{result}");

    (errors, result)  
}

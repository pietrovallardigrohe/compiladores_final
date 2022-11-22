use std::collections::HashSet;

use pest::Parser;

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)]
#[grammar = "Grammar/Lexer.pest"]
pub struct Lexer;

// struct Token {
//     rule: pest::Rule
// }

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
 * [1]if(a == 0) {[3]int b = 1;[5]} else {[7]float c = 1.0;[8]// a[9]}
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
 * [1]if(a == 0) {[3]int b = 1;[5]}
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES
 * Retorna um Set contendo os erros e o resultado da formatação
 */
pub fn get_tokens(input: &str) -> (HashSet<usize>, String) {

    let mut result = String::new();
    let mut current_line: usize = 0;
    let mut errors: HashSet<usize> = HashSet::new();

    let precompiled_input: String = precompile(input);
    println!("{}", precompiled_input);

    // Faz o parsing da entrada e formatação de cada token
    match Lexer::parse(Rule::TOKEN, &precompiled_input) {
        Ok(pairs) => {
            for pair in pairs {
                match &pair.as_rule() {
                    Rule::LINE => {
                        if let Ok(line) = pair.as_str().trim_matches(|c| c == '[' || c == ']').parse::<usize>() {
                            current_line = line;
                        } else {
                            println!("UNEXPECTED LINE PARSING BEHAVIOR");
                        } 
                    },
                    Rule::TYPE => {
                        result.push_str(&format!("{} ", pair.as_str().to_uppercase()));
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
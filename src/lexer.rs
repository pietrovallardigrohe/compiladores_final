use std::collections::HashSet;

use pest::Parser;

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
    result.push_str("$");

    return result
}

/*
 * Identifica os tokens e formata o resultado para a análize sintática
 * [1]if(a == 0) {[3]int b = 1;[5]}
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES
 * Retorna um Set contendo os erros e o resultado da formatação
 */
pub fn lexer(input: &str) -> (HashSet<usize>, String) {
    // Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
    mod lexer_func {
        #[derive(Parser)] 
        #[grammar = "Grammar/Lexer.pest"] 
        pub struct Lexer;
    }
    use lexer_func::*;

    let mut result = String::new();
    let mut line_count: usize = 0;
    let mut errors: HashSet<usize> = HashSet::new();

    // Faz o parsing da entrada e formatação de cada token
    match Lexer::parse(Rule::TOKEN, &precompile(input)) {
        Ok(pairs) => {
            for pair in pairs {
                match &pair.as_rule() {
                    Rule::LINE => {
                        if let Ok(line) = pair.as_str().trim_matches(|c| c == '[' || c == ']').parse::<usize>() {
                            line_count = line;
                        } else {
                            println!("UNEXPECTED LINE PARSING BEHAVIOR");
                        }
                    },
                    Rule::IF | Rule::ELSE | Rule::SWITCH  => {
                        result.push_str(&format!("<{line_count}> {:?} ", pair.as_rule()));
                    },
                    Rule::TYPE => {
                        result.push_str(&format!("{} ", pair.as_str().to_uppercase()));
                        
                    },
                    Rule::ERROR => {
                        result.push_str(&format!("ERROR")); 
                        errors.insert(line_count);
                    },
                    Rule::END => {
                        result.push_str(&format!("<{line_count}> {:?} ", pair.as_rule()));
                    }
                    _ => {
                        result.push_str(&format!("{:?} ", pair.as_rule()));
                    }
                }
            }
        },
        Err(err) => println!("{err}") 
    } 

    (errors, result)  
}
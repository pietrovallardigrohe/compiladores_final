/*
 * Pietro Vallardi Grohe
 * Compiladores
 * Etapa 2
 * 15/11/2022
 * 
 * Dentro do cmd utilize dentro da pasta compiladores_final
 * "cargo build" para compilar e "cargo run" para rodar o programa 
 * Arquivo "tests.txt" contém diversas palavras que aceitam e rejeitam as regras de produção
 * BNF Encontrada no arquivo Grammar/Lexer.pest e Grammar/Syntax.pest
 */
use std::{fs, path::Path, io::BufRead};
use pest::Parser;
use std::collections::HashSet;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    //  Define o limite de chamadas das regras não terminais
    // pest::set_call_limit(NonZeroUsize::new(5000));
    
    print!("\n");

    // Lê o caminho do arquivo
    println!("File Path");
    let mut file_path = String::new(); 
    std::io::stdin().lock().read_line(&mut file_path).expect("CANNOT READ INPUT");
    println!("\n\n");
    
    // Arquivo utilizado em testes
    // let file_path = String::from("src\\tests.txt"); 
    
    // Tenta ler o arquivo para a variável 'file'
    let Ok(raw_input) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };

    let mut filtered = String::new();

    let precompilation = precompile(&raw_input.to_owned());
    let (lex_errors, lex_input) = lexer(&precompilation);
    // println!("{lex_input}");

    // Se for válido a análize printa o arquivo indicando onde estão os erros
    match syntax(&lex_input) {
        Some(mut errors) => {
            errors.extend(lex_errors);
            errors.remove(&0);
            for (index, line) in raw_input.lines().enumerate() {
                if errors.contains(&(index+1)) {
                    filtered.push_str(&format!("ERROR-----> {line}"));
                } else {
                    filtered.push_str(&format!("{line}"));
                }
                filtered.push_str("\n");
            }
            
            println!("{}", filtered);
            if errors.len() == 0 {
                println!("No errors found");
            } 
            else {
                println!("Error at line {:?}", errors);
            }
        },
        None => {
            println!("ERROR INVALID BRACES");
        }
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
 * [1]if(a == 0) {[3]int b = 1;[5]} else {[7]float c = 1.0;[8]// a[9]}
 */ 
fn precompile(input: &str) -> String {
    let mut result = String::new();

    input.lines().enumerate()
        .filter(|(_, e)| !e.trim().is_empty() || !e.starts_with("//"))
        .for_each(|(index, element)| result += &format!("[{}]{}", (index+1).to_string(), element.trim()));
    
    return result
}

/*
 * Identifica os tokens e formata o resultado para a análize sintática
 * [1]if(a == 0) {[3]int b = 1;[5]}
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES
 * Retorna um Set contendo os erros e o resultado da formatação
 */

fn lexer(input: &str) -> (HashSet<usize>, String) {
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
    match Lexer::parse(Rule::TOKEN, input) {
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
                        result.push_str(&format!("<{}> {:?} ", line_count, pair.as_rule()));
                    },
                    Rule::TYPE => {
                        result.push_str(&format!("{} ", pair.as_str().to_uppercase()));
                        
                    },
                    Rule::ERROR => {
                        result.push_str(&format!("ERROR")); 
                        errors.insert(line_count);
                    },
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

/*
 * Analiza a sintaxe do lexer
 * Input:
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES
 * retorna os erros sintáticos ou None se tiverem erros de { }
 */
fn syntax(input: &str) -> Option<HashSet<usize>> {
    // Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
    mod syntax_func {
        #[derive(Parser)]
        #[grammar = "Grammar/Syntax.pest"]
        pub struct Syntax;
    }
    use syntax_func::*;

    let mut current_line: usize = 0;
    let mut block: String = String::new();
    let mut blocks: Vec<(usize, String)> = Vec::new();
    let mut open_blocks: i32 = 0;
    let mut errors: HashSet<usize> = HashSet::new();

    // Separa os tokens em blocos IF/ELSE/SWITCH{...}
    /*
     * TODO
     * Utilize a stack to determine block start and endings, return an error if stack is not full when the current token is IF or ELSE or EOI
     * Proto:
     * On IF/ELSE/EOI if BRACE STACK is not empty add line to error stack 
     */
    for token in input.split_whitespace() {
        if let Ok(line) = token.trim_matches(|c| c == '<' || c == '>').parse::<usize>() {
            current_line = line;
        } else {
            block += &format!("{token} ");
            match token {
                "OPEN_BRACES" => {
                    open_blocks += 1;
                },
                "CLOSE_BRACES" => {
                    open_blocks -= 1;
                    if open_blocks == 0 {
                        // println!("\nBlock\nLINE: {current_line}\n{block}");
                        blocks.push((current_line, block));
                        block = String::from("");
                    } else if open_blocks == -1 {
                        // println!("\nBlock\nLINE: {current_line}\n{block}");
                        blocks.push((current_line, block));
                        block = String::from("");      
                    }
                },
                _ => ()
            }
        }
    } 

    // Se os blocos não forem fechados retorna None, invalidando o parse
    if open_blocks != 0 {
        return None
    }

    // Faz a analize sintática em cada um dos blocos, coletando os erros
    for (line, block) in blocks {
        match Syntax::parse(Rule::IF_ELSE_SWITCH, &block) {
            Ok(_) => errors.insert(0),//println!("\n\nPARSING ACCEPTED \n\n{:?}", pairs.as_str()),
            Err(_) => errors.insert(line)
        };
    }

    Some(errors)
}

use std::collections::HashSet;
use pest::Parser;

/*
 * Analiza a sintaxe do lexer
 * Input:
 * <1> IF OPEN_PARENTHESES ID CONDITION CLOSE PARENTHESES OPEN_BRACES <3> INT ID ATTRIBUTION NUM COMMA <5> CLOSE_BRACES
 * retorna os erros sintáticos ou None se tiverem erros de { }
 */
pub fn syntax(input: &str) -> Option<HashSet<usize>> {
    // Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
    mod syntax_func {
        #[derive(pest_derive::Parser)]
        #[grammar = "Grammar/Syntax.pest"]
        pub struct Syntax;
    }
    use syntax_func::*;

    let mut old_line: usize = 0;
    let mut current_line: usize = 0;
    let mut block: String = String::new();
    let mut blocks: Vec<(usize, String)> = Vec::new();
    let mut errors: HashSet<usize> = HashSet::new();
    let mut braces_stack: Vec<i32> = Vec::new();

    // Separa os tokens em blocos IF/ELSE/SWITCH{...}
    /*
     * TODO
     * Utilize a stack to determine block start and endings, return an error if stack is not full when the current token is IF or ELSE or EOI
     * Proto:
     * On IF/ELSE/EOI if BRACE STACK is not empty add line to error stack 
     */
    for token in input.split_whitespace() {
        if let Ok(line) = token.trim_matches(|c| c == '<' || c == '>').parse::<usize>() {
            old_line = current_line;
            current_line = line;
        } else {
            match token {
                "OPEN_BRACES" => {
                    braces_stack.push(1);
                },
                "CLOSE_BRACES" => {
                    braces_stack.pop();
                }
                "IF" | "ELSE" | "SWITCH" | "END" => {                    
                    if braces_stack.len() == 0 {
                        // println!("{:?}", braces_stack);
                        if block.len() != 0 {
                            blocks.push((current_line, block));
                        }
                        block = String::from("");
                    } else {
                        errors.insert(old_line);
                    }
                    // if braces_stack.len() != 0 {
                    //     errors.insert(current_line);
                    // } 
                },
                // "CLOSE_BRACES" => {
                //     braces_stack.pop();
                //     if braces_stack.len() == 0 {
                //         // println!("\nBlock\nLINE: {current_line}\n{block}");
                //         blocks.push((current_line, block));
                //         block = String::from("");
                //     } else if open_blocks == -1 {
                //         // println!("\nBlock\nLINE: {current_line}\n{block}");
                //         blocks.push((current_line, block));
                //         block = String::from("");      
                //     }
                // },
                _ => ()
            }
            block += &format!("{token} ");
        }
    } 

    // println!("{:?}", blocks);

    // Se os blocos não forem fechados retorna None, invalidando o parse
    // if open_blocks != 0 {
    //     return None
    // }

    // Faz a analize sintática em cada um dos blocos, coletando os erros
    for (line, block) in blocks {
        let parse = Syntax::parse(Rule::IF_ELSE_SWITCH, &block);
        match parse {
            Ok(_) => {
                errors.insert(0);
            },//println!("\n\nPARSING ACCEPTED \n\n{:?}", pairs.as_str()),
            Err(error) => {
                errors.insert(line);
                println!("========\n\n{:#?} \n{:#?}", error.location, error.variant);
            }
        }
    }

    Some(errors)
}

#![allow(unused_imports)]
use pest::Parser;
use std::{vec, env::current_exe};

use crate::lex::{Token, self};

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)]
#[grammar = "Grammar/Syntax.pest"]
pub struct Syntax;

#[derive(Debug)]
pub struct SyntaxError<'a> {
    pub token: Token<'a>,
    pub message: String
}
impl<'a> SyntaxError<'a> {
    fn new(token: Token, message: impl Into<String>) -> SyntaxError {
        SyntaxError {
            token,
            message: message.into()
        }
    }
}

/*
 * Entrada: Sequencia de tokens
 * [
 *    Token { input: "if", line: 1, column: 0, rule: IF },
 *    Token { input: "(", line: 1, column: 2, rule: OPEN_PARENTHESES },
 *    Token { input: "a", line: 1, column: 3, rule: IDENTIFIER }
 *    ...
 * ]
 * Saída: Sequencia de Erros sintáticos 
 */
pub fn get_syntax_errors(tokens: Vec<Token>) -> Vec<SyntaxError> {
    let mut index = 0;
    let end = *tokens.last().unwrap();
    let mut is_if: bool = false;
    let mut errors: Vec<SyntaxError> = vec![];
    
    let mut token = *tokens.get(index).unwrap_or_else(|| &end);
    index += 1;
    // println!("{:?}", token);
    // Separa os tokens em vetores, de chamada da função e outro para o corpo da função
    'function: loop {
        let mut func_call = vec![];
        let mut func_body = vec![];

        // Function Call Tokens
        match token.rule {
            lex::Rule::IF | lex::Rule::SWITCH | lex::Rule::WHILE_FUNCTION | lex::Rule::FOR_FUNCTION => {
                if token.rule == lex::Rule::IF {
                    is_if = true;
                } else {
                    is_if = false;
                }
                func_call.push(token);
                token = *tokens.get(index).unwrap_or_else(|| &end);
                index += 1;
                // func_call.push(token);
                // println!("{:?}", token);
                if token.rule == lex::Rule::OPEN_PARENTHESES {
                    func_call.push(token);
                    token = *tokens.get(index).unwrap_or_else(|| &end);
                    index += 1;
                    'call: loop  {
                        match token.rule {
                            lex::Rule::CLOSE_PARENTHESES => {
                                // Fim da chamada
                                func_call.push(token);
                                break 'call;
                            },
                            lex::Rule::OPEN_BRACES => {
                                errors.push(SyntaxError::new(token, "EXPECTED CLOSING PARENTHESIS"));
                                break 'call;
                            }
                            lex::Rule::END => {
                                // Fim da entrada
                                break 'function;
                            }
                            _ => (),
                        }
                        func_call.push(token);
                        token = *tokens.get(index).unwrap_or_else(|| &end);
                        index += 1;
                    } 
                } else {
                    errors.push(SyntaxError::new(token, "EXPECTED PARENTHESIS"));
                }

            },
            lex::Rule::ELSE => {
                if is_if {
                    func_call.push(token);
                } else {
                    errors.push(SyntaxError::new(token, "EXPECTED IF FUNCTION PRIOR"));
                }
            },
            lex::Rule::END => {
                break;
            }
            _ => {
                errors.push(SyntaxError::new(token, "CALL UNRECOGNIZED"))
            }
        }

        token = *tokens.get(index).unwrap_or_else(|| &end);
        index += 1;
        func_body.push(token);
        
        // Body of the function
        match token.rule {
            lex::Rule::OPEN_BRACES => {
                let mut open_braces: i32 = 1;  

                token = *tokens.get(index).unwrap_or_else(|| &end);
                index += 1;
                'body: loop {
                    match token.rule {
                        lex::Rule::OPEN_BRACES => open_braces += 1,
                        lex::Rule::CLOSE_BRACES => {
                            open_braces -= 1;
                            if open_braces == 0 {
                                func_body.push(token);
                                break;
                            }
                        },
                        lex::Rule::IF | lex::Rule::ELSE | lex::Rule::SWITCH => {
                            if open_braces != 0 {
                                errors.push(SyntaxError::new(token, "BRACES ERROR"));
                            }
                            continue 'function;
                        },
                        lex::Rule::END => {
                            if open_braces != 0 {
                                errors.push(SyntaxError::new(token, "BRACES ERROR"));
                                break 'body;
                            }
                            // errors.push(SyntaxError::new(token, "EARLY END OF STREAM"));
                            break;
                        },
                        _ => {
                            // println!("{:?}", token);
                        }
                    }
                    func_body.push(token);
                    token = *tokens.get(index).unwrap_or_else(|| &end);
                    index += 1;
                }

            },
            _ => {
                errors.push(SyntaxError::new(token, "FAILED TO OPEN BRACES"));
                continue 'function;
            }
        }
        token = *tokens.get(index).unwrap_or_else(|| &end);
        index += 1;

        let mut is_switch = false;

        // Function call parsing
        if func_call.len() > 3 {
            let rule = func_call[0].rule;
            func_call = func_call[2..].to_vec();
            func_call = func_call[..func_call.len()-1].to_vec();

            let call = func_call.iter().map(|ele| format!("{:?} ", ele.rule)).collect::<String>();
            // println!("{call}");

            match rule {
                lex::Rule::IF => {
                    is_switch = false;
                    let result = Syntax::parse(Rule::IF_CALL, &call);
                    match result {
                        Ok(_) => (),
                        Err(err) => {
                            if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                let token_index = err.line().get(col..).unwrap().split_whitespace().count() +1 ;
                                errors.push(SyntaxError::new(func_call[token_index], format!("IF CALL ERROR")));
                                // println!("IF Parsing error, {:?}\n str: {:?} col: {:?}", func_call[token_index], err.line(), col);
                            }
                        }
                    }
                },
                lex::Rule::SWITCH => {
                    is_switch = true;
                    let result = Syntax::parse(Rule::SWITCH_CALL, &call);
                    match result {
                        Ok(_) => (),
                        Err(err) => {
                            if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                let token_index = err.line().get(col..).unwrap().split_whitespace().count();
                                errors.push(SyntaxError::new(func_call[token_index], format!("SWITCH CALL ERROR")));
                                // println!("IF Parsing error, {:?}\n str: {:?} col: {:?}", func_call[token_index], err.line(), col);
                            }
                        }
                    }
                },
                lex::Rule::WHILE_FUNCTION => {
                    is_switch = false;
                    let result = Syntax::parse(Rule::WHILE_CALL, &call);
                    match result {
                        Ok(_) => (),
                        Err(err) => {
                            if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                let token_index = err.line().get(col..).unwrap().split_whitespace().count() +1 ;
                                errors.push(SyntaxError::new(func_call[token_index], format!("WHILE CALL ERROR")));
                                // println!("IF Parsing error, {:?}\n str: {:?} col: {:?}", func_call[token_index], err.line(), col);
                            }
                        }
                    }                   
                },
                lex::Rule::FOR_FUNCTION => {
                    is_switch = false;
                    let result = Syntax::parse(Rule::FOR_CALL, &call);
                    match result {
                        Ok(_) => (),
                        Err(err) => {
                            if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                let token_index = err.line().get(col..).unwrap().split_whitespace().count() +1 ;
                                errors.push(SyntaxError::new(func_call[token_index], format!("FOR CALL ERROR")));
                                // println!("IF Parsing error, {:?}\n str: {:?} col: {:?}", func_call[token_index], err.line(), col);
                            }
                        }
                    }  
                },
                _ => { 
                    println!("parse error");
                }
            }
        } else if func_call.len() == 1 {    
            let rule = func_call[0].rule;
            if rule != lex::Rule::ELSE {
                errors.push(SyntaxError::new(func_call[0], "ELSE CALL ERROR"));
            }
        }

        // Body parsing
        // Parsing de cada commando
        func_body.remove(0);
        func_body.pop().unwrap();
        // Tratamento diferente para função switch
        if is_switch {
            let mut case: Vec<Token> = vec![];
            let mut cases: Vec<Vec<Token>> = vec![];

            for token in func_body {
                match token.rule {
                    lex::Rule::CLOSE_BRACES => {
                        case.push(token);
                        cases.push(case.clone());
                        case.clear();
                        
                    }
                    lex::Rule::COLON => {
                        case.push(token);
                    },
                    _ => {
                        case.push(token);
                    }
                }
            }

            // cases.iter().for_each(|_token| println!("{:#?}", _token));
            for case in cases {
                // let mut case_iter = case.into_iter().clone();
                let token = *case.iter().next().unwrap();
                let mut default_amount: i32 = 0;
                match token.rule {
                    lex::Rule::CASE | lex::Rule::DEFAULT => {
                        if token.rule == lex::Rule::DEFAULT { 
                            default_amount += 1;
                            if default_amount > 1 {
                                errors.push(SyntaxError::new(token, "TOO MANY DEFAULT CASES"));
                            }
                        }
                        // Parse Case Body
                        let _case: Vec<&Token> = case.iter().take_while(|token| token.rule != lex::Rule::OPEN_BRACES).collect();
                        let case_body = _case.iter().map(|token| format!("{:?}", token.rule)).collect::<String>();
                        let result = Syntax::parse(Rule::CASE_BODY, &case_body);                    
                        match result {
                            Ok(_) => (),
                            Err(err) => {
                                if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                    let token_index = err.line().get(col..).unwrap().split_whitespace().count();
                                    errors.push(SyntaxError::new(*_case[token_index], "FAILED TO PARSE CASE"));
                                }
                            }
                        }
                        // println!("{:#?}", case[_case.len()..].to_vec());
                        // Trim braces from block 
                        let mut body = case.clone()[_case.len()..].to_vec();
                        body.remove(0);
                        let closing_braces = body.pop().unwrap();
                        if body.last().unwrap().rule != lex::Rule::COMMA {
                            errors.push(SyntaxError::new(closing_braces, "COMMAND ERROR, EXPECTED BRACES"));
                            break;
                        }
                        let _body = body.iter().map(|token| format!("{:?} ", token.rule)).collect::<String>();
                        let commands: Vec<&str> = _body.split_inclusive("COMMA ").collect();
                        for command in commands {
                            // println!("{command}");
                            match Syntax::parse(Rule::COMMAND, command) {
                                Ok(_) => (),
                                Err(err) => {
                                    if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                        // println!("{} {col}", err.line());
                                        let token_index = err.line().get(col..).unwrap_or_else(|| err.line()).split_whitespace().count();
                                        errors.push(SyntaxError::new(body[token_index - 1], "FAILED TO PARSE COMMAND"));
                                    }
                                }
                            }
                        }
                        
                        // println!("{:?}", commands);

                    },
                    _ => errors.push(SyntaxError::new(token, "CASE ERROR, EXPECTED CASE | DEFAULT"))
                }
            }

        } else {
            // println!("{:?}", func_body);
            
            let mut comms: Vec<Vec<Token>> = vec![];
            let mut comm: Vec<Token> = vec![];
            for token in &func_body {
                if token.rule == lex::Rule::COMMA {
                    comm.push(token.clone());
                    comms.push(comm.clone());
                    comm = vec![];
                } else if token == func_body.last().unwrap() {
                    comm.push(token.clone());
                    comms.push(comm.clone());
                    comm = vec![];
                } else {
                    comm.push(token.clone());
                }
            }

            let rules = func_body.iter().map(|ele| format!("{:?} ", ele.rule)).collect::<String>();
            
            let commands: Vec<&str> = rules.split_inclusive("COMMA ").collect();
            // println!("{:#?}\n\n{:#?}", comms, commands);
            let mut command_counter = 0;
            for command in commands {
                // println!("{command} ");
                match Syntax::parse(Rule::COMMAND, command) {
                    Ok(_) => (),
                    Err(err) => {
                        if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                            // current_pos += col;
                            // println!("{:?} {col}", err.line());
                            let token_index = comms[command_counter].len() - err.line().get(col..).unwrap_or_else(|| err.line()).split_whitespace().count();
                            // println!("{}", token_index-1);
                            errors.push(SyntaxError::new(comms[command_counter][token_index], "FAILED TO PARSE COMMAND"));
                        }
                    }
                }
                command_counter += 1;
            }
        }

        // println!("\nFUNCTION CALL==============");
        // func_call.iter().for_each(|token| println!("{:?}", token));
        // println!("\nFUNCTION BODY==============");
        // func_body.iter().for_each(|token| println!("{:?}", token));
    }
    // println!("\nErrors==============");
    // errors.iter().for_each(|err| println!("{:#?}\n{:?}\n", err.message, err.token));
    errors
}
#![allow(unused_imports)]
use pest::{Parser, iterators::Tokens, unicode::CLOSE_PUNCTUATION};
use std::{vec, env::current_exe, rc::Rc, io::stdin};

use crate::lex::{Token, self};

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)]
#[grammar = "Grammar/Syntax.pest"]
pub struct Syntax;

#[derive(Debug, Clone)]
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

fn get_tokens_rule(tokens: &[Token]) -> String {
    let mut tokens_combined = String::new();

    for token in tokens {
        tokens_combined.push_str(&format!("{:?} ", token.rule));
    }

    tokens_combined.trim().to_string()
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
pub fn syntax_test<'a>(tokens: &[Token<'a>]) -> Vec<SyntaxError<'a>> {

    let mut token_index: usize = 0;
    let mut errors: Vec<SyntaxError> = vec![];
    let mut is_if: bool = false;
    while let Some(tks) = tokens.get(token_index..) {
        // let mut tokens_iter = match tokens.get(token_index..) {
        //     Some(tks) => tks.iter(),
        //     None => break
        // };

        // let mut index: usize = 0;
        // let mut parse_errors: Vec<SyntaxError> = vec![];
        let mut tokens_iter = tks.iter();
        let token: &Token = tokens_iter.next().unwrap();
        
        println!("Token Index: {token_index}\n");
        let function_errors: Vec<SyntaxError> = match token.rule {
            lex::Rule::IF | lex::Rule::WHILE_FUNCTION => {
                println!("IF/WHILE");

                if token.rule == lex::Rule::IF {
                    is_if = true;
                }

                let (index, mut parse_errors) = parse_call(&tokens_iter.clone().copied().collect::<Vec<Token>>(), Rule::CONDITION);
                token_index += index;
                // println!("{:#?}", &tokens.get(token_index..).unwrap().to_vec());
                let (last_token,mut body_errors) = parse_body(tokens.get(token_index..).unwrap(), Rule::COMMAND);
                token_index += last_token;
                parse_errors.append(&mut body_errors);
                // println!("{last_token_line}\n {:#?}", tokens.get(last_token_line..).unwrap());
                parse_errors
            },
            lex::Rule::ELSE => {
                println!("ELSE");

                // println!("{tokens_iter:#?}");
                let (last_token, mut body_errors) = parse_body(&tokens_iter.clone().copied().collect::<Vec<Token>>(), Rule::COMMAND);
                token_index += last_token;
                token_index += 1;
                // println!("Token Index: {token_index}");
                
                if !is_if{
                    body_errors.push(SyntaxError::new(*token, "ELSE WITHOUT IF"));
                }
                else {
                    // token_index += 1;
                    is_if = false;
                }

                // std::thread::sleep(std::time::Duration::new(10, 0));

                body_errors
            }
            lex::Rule::FOR_FUNCTION => {
                println!("FOR");

                is_if = false;

                let (index, mut parse_errors) = parse_call(&tokens_iter.clone().copied().collect::<Vec<Token>>(), Rule::FOR_CALL);
                token_index += index;
                
                let (last_token, mut body_errors) = parse_body(tokens.get(token_index..).unwrap(), Rule::COMMAND);
                token_index += last_token;
                parse_errors.append(&mut body_errors);
                
                // println!("{last_token_line}\n {:#?}", tokens.get(last_token_line..).unwrap());           
                parse_errors
            },
            lex::Rule::SWITCH => {
                println!("SWITCH");

                is_if = false;

                let (index, mut parse_errors) = parse_call(&tokens_iter.clone().copied().collect::<Vec<Token>>(), Rule::ID);
                token_index += index;

                let (last_token, mut body_errors) = parse_body(tokens.get(token_index..).unwrap(), Rule::CASE_BODY);
                token_index += last_token;
                parse_errors.append(&mut body_errors);
                
                // println!("{token_index}\n {:#?}", tokens.get(token_index..).unwrap());
                parse_errors
            },
            _ => {
                is_if = false;

                // println!("{tokens_iter:#?}\n{token:#?}");
                if token.rule == lex::Rule::END || tokens_iter.len() == 0 {
                    // std::thread::sleep(std::time::Duration::new(10, 0));
                    break;
                } else {
                    println!("NOT A FUNCTION CALL");

                    // break;
                    // std::thread::sleep(std::time::Duration::new(10, 0));
                    token_index += 1;
                    vec![]
                }
            }
        };
        errors.extend(function_errors);
    }

    errors
}

fn parse_call<'a>(tokens: &[Token<'a>], rule: Rule) -> (usize, Vec<SyntaxError<'a>>) {
    println!("================\nPARSING CALL");

    let mut last_token_index: usize = 0;

    let mut tokens_iter = tokens.iter();
    let mut token = tokens_iter.next().unwrap();
    last_token_index += 1;

    let mut errors = vec![];

    // OPEN PARENTHESES PARSE
    if token.rule == lex::Rule::OPEN_PARENTHESES {
        token = tokens_iter.next().unwrap();
        last_token_index += 1;
    } else {
        errors.push(SyntaxError::new(*token, "OPEN PARENTHESES EXPECTED"));
    }

    // INNER CALL TOKENS
    let mut inner_call: Vec<Token> = vec![];
    loop {
        match token.rule {
            lex::Rule::CLOSE_PARENTHESES => {
                last_token_index += 1;
                break;
            },
            lex::Rule::OPEN_BRACES | lex::Rule::END => {
                errors.push(SyntaxError::new(*token, "EXPECTED CLOSE PARENTHESES"));
                break;
            },
            _ => {
                inner_call.push(*token);
                token = tokens_iter.next().unwrap();
                last_token_index += 1;
            }
        }
    }
    
    // INNER CALL PARSE
    match Syntax::parse(rule, &get_tokens_rule(&inner_call)) {
        Ok(_) => println!("RULE {:?} ACCEPTED", rule),
        Err(_) => {
            println!("RULE {:?} REJECTED", rule);
            errors.push(SyntaxError::new(*token, "EXPECTED CONDITION"))
        }
    }

    // println!("Current Token: {:?} , Iter: {:#?}", *token, tokens_iter);
    println!("ERRORS=======\n{:#?}", errors);
    println!("PARSE CALL END\n");

    (last_token_index, errors)
}

fn parse_body<'a>(tokens: &[Token<'a>], rule: Rule) -> (usize, Vec<SyntaxError<'a>>) {
    println!("PARSING BODY");

    let mut last_token_index: usize = 0;

    let mut tokens_iter = tokens.iter();
    let mut token = tokens_iter.next().unwrap();
    last_token_index += 1;

    let mut errors = vec![];

    // OPEN BRACES PARSE
    if token.rule == lex::Rule::OPEN_BRACES {
        token = tokens_iter.next().unwrap();
        last_token_index += 1;
    } else {
        errors.push(SyntaxError::new(*token, "OPEN BRACES EXPECTED"));
    }

    // INNER BODY
    if rule == Rule::COMMAND {
        let mut inner_body: Vec<Vec<Token>> = vec![];
        let mut command: Vec<Token> = vec![];
        loop {
            match token.rule {
                lex::Rule::CLOSE_BRACES => {
                    // last_token_index += 1;
                    break;
                },
                lex::Rule::OPEN_BRACES => {
                    errors.push(SyntaxError::new(*token, "CLOSE BRACES EXPECTED"));
                    break;
                },
                lex::Rule::IF | lex::Rule::ELSE | lex::Rule::SWITCH | lex::Rule::WHILE_FUNCTION | lex::Rule::FOR_FUNCTION => {
                    errors.push(SyntaxError::new(*token, "CLOSE BRACES EXPECTED"));
                    break;
                },
                lex::Rule::COMMA => {
                    command.push(*token);
                    inner_body.push(command);
                    command = vec![];
                    token = tokens_iter.next().unwrap();
                },
                _ => {
                    command.push(*token);
                    token = tokens_iter.next().unwrap();
                }
            }
            last_token_index += 1;
        }

        if !command.is_empty(){
            errors.push(SyntaxError::new(*command.last().unwrap(), "EXPECTED COMMA"));
        }

        // let tokens: Vec<&str> = body.split(' ')
        //                             .filter(|ele| !ele.is_empty())
        //                             .collect();
        // println!("{:#?}", &commands);
        for command in &inner_body {
            match Syntax::parse(rule, &get_tokens_rule(command)) {
                Ok(_) => (),//println!("COMMAND ACCEPTED: {:?}", &get_tokens_rule(&command)),
                Err(error) => {
                    if let pest::error::LineColLocation::Pos((_, col)) = error.line_col {
                        // current_pos += col;
                        // println!("{:?} {col}", err.line());
                        let token_index = error.line().get(..col).unwrap_or_else(|| error.line()).split_whitespace().count();
                        // println!("{:#?} {}", &command, token_index-1);
                        errors.push(SyntaxError::new(command[token_index-1], "FAILED TO PARSE COMMAND"));
                    }
                }
            }
        }
        
        // println!("Current Token: {:?} , Iter: {:#?}", *token, tokens_iter);
        // println!("Inner Body \n{:#?}", inner_body);
        // println!("Index: {last_token_index} LEN: {:#?}", tokens_iter.len());

    }
    else if rule == Rule::CASE_BODY {
        
        let mut inner: Vec<Token> = tokens_iter.clone().copied().collect();
        inner.insert(0, *token);
        // println!("{token:#?}");
        
        let mut switch_body: Vec<Token> = vec![];
        let mut open_braces: i32 = 1;
        let mut open_block = false;
        for token in &inner {
            match token.rule {
                lex::Rule::OPEN_BRACES => {
                    open_braces += 1;
                    open_block = true;
                    switch_body.push(*token);
                },
                lex::Rule::CLOSE_BRACES => {
                    switch_body.push(*token);
                    open_braces -= 1;
                    if open_braces == 0 {
                        // last_token_index += 1;
                        if open_block {
                            break;
                        } else {
                            errors.push(SyntaxError::new(*token, "BRACES ERROR"));
                            
                            last_token_index -= 1;
                            return (last_token_index, errors);

                        }
                    }
                },
                lex::Rule::IF | lex::Rule::ELSE | lex::Rule::SWITCH | lex::Rule::WHILE_FUNCTION | lex::Rule::FOR_FUNCTION => {
                    errors.push(SyntaxError::new(*token, "CLOSE BRACES EXPECTED"));
                    last_token_index += 1;
                    break;
                },
                _ => {
                    switch_body.push(*token);
                }
            }
            last_token_index += 1;
        }
        
        // inner.iter().for_each(|token| println!("{token:?}"));
        
        // switch_body.pop().unwrap(); // Removes the $end$ Token
        
        let last_token = switch_body.pop().unwrap();
        if last_token.rule != lex::Rule::CLOSE_BRACES {
            errors.push(SyntaxError::new(last_token, "EXPECTED CLOSING BRACES"));
        }
        
        let mut cases: Vec<Vec<Token>> = vec![];
        let mut case: Vec<Token> = vec![];
        for token in &switch_body {
            match token.rule {
                lex::Rule::CLOSE_BRACES => {
                    case.push(*token);
                    cases.push(case.clone());
                    case.clear();
                }
                lex::Rule::COLON => {
                    case.push(*token);
                },
                _ => {
                    case.push(*token);
                }
            }
            // last_token_index += 1;
        }

        for (index, case) in cases.clone().into_iter().enumerate() {
            if case.is_empty() {
                errors.push(SyntaxError::new(*case.last().unwrap(), "EXPECTED CASE BODY"));
                cases.remove(index);
            }
        }

        // switch_body.iter().for_each(|token| println!("{token:?}"));
        // println!("Cases: {cases:#?}");

        // for case in cases {
        //     let mut case_iter = case.iter();
        //     let mut token = case_iter.next().unwrap();
            
        //     let mut _case: Vec<Token> = vec![]; 
        //     if token.rule == lex::Rule::CASE {
        //         token = case_iter.next()
        //     } else {
        //         errors.push(SyntaxError::new(*token, "EXPECTED CASE | DEFAULT"));
        //     }
        // } 
        for case in cases {
            // let mut case_iter = case.into_iter().clone();
            let token = case.first().unwrap();
            let mut default_amount: i32 = 0;
            match token.rule {
                lex::Rule::CASE | lex::Rule::DEFAULT => {
                    if token.rule == lex::Rule::DEFAULT { 
                        default_amount += 1;
                        if default_amount > 1 {
                            errors.push(SyntaxError::new(*token, "TOO MANY DEFAULT CASES"));
                        }
                    }
                    // Parse Case Body
                    // println!("{case:#?}, len: {:#?}", case.len());
                    let _case: Vec<&Token> = case.iter().take_while(|token| token.rule != lex::Rule::OPEN_BRACES).collect();
                    // let mut has_braces: bool = false;  
                    // _case.iter().for_each(|token| {
                    //     if token.rule == lex::Rule::OPEN_BRACES { has_braces = true; }
                    // });

                    // if has_braces == false {
                    //     errors.push(SyntaxError::new(**_case.last().unwrap(), "EXPECTED OPEN BRACES"));
                    //     continue;
                    // }

                    let case_body = _case.iter().map(|token| format!("{:?}", token.rule)).collect::<String>();
                    let result = Syntax::parse(Rule::CASE_BODY, &case_body);                    
                    match result {
                        Ok(_) => (),
                        Err(err) => {
                            if let pest::error::LineColLocation::Pos((_, col)) = err.line_col {
                                let token_index = err.line().get(col..).unwrap().split_whitespace().count();
                                errors.push(SyntaxError::new(*_case[token_index], "FAILED TO PARSE CASE"));
                                continue;
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
                    for command in &commands {
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
                _ => errors.push(SyntaxError::new(*token, "CASE ERROR, EXPECTED CASE | DEFAULT"))
            }
        }
        // last_token_index += 1;
    }
    
    println!("Index: {last_token_index}");
    println!("ERRORS=======\n{:#?}", errors);
    println!("PARSE BODY END\n================");

    (last_token_index, errors)
}
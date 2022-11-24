use pest::Parser;

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)]
#[grammar = "Grammar/Lexer.pest"]
pub struct Lexer;

/*
 * Token
 * input -> raw input parsed
 * line -> line of the token
 * col -> start column of the token
 * rule -> Rule that accepted the input
 */
#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub input: &'a str,
    pub line: usize,
    pub column: usize,
    pub rule: crate::lex::Rule
}

impl Token<'_> {
    pub fn new(input: &'_ str, line: usize, column: usize, rule: Rule) -> Token {
        Token { 
            input: input.into(),
            line,
            column,
            rule
        }
    }
}

// impl Copy for Token {

//     fn clone(&self) -> Token {
//         *self;
//     }

// }

/*
 * Entrada:
 * if(a == b) {
 *     a = 2;
 *     int a = 2;#asdasd
 *     a++;
 *     b+=1;
 * }
 * 
 * Retorno: Vec<Token> todos os Tokens
 * Token { input: "if", line: 1, column: 0, rule: IF }
 * Token { input: "(", line: 1, column: 2, rule: OPEN_PARENTHESES }
 * Token { input: "a", line: 1, column: 3, rule: IDENTIFIER }
 * Token { input: "==", line: 1, column: 5, rule: CONDITIONAL }
 * Token { input: "b", line: 1, column: 8, rule: IDENTIFIER }
 * Token { input: ")", line: 1, column: 9, rule: CLOSE_PARENTHESES }
 * Token { input: "{", line: 1, column: 11, rule: OPEN_BRACES }
 * Token { input: "a", line: 2, column: 17, rule: IDENTIFIER }
 * Token { input: "=", line: 2, column: 19, rule: ATTRIBUTION }
 * Token { input: "2", line: 2, column: 21, rule: NUM }
 * Token { input: ";", line: 2, column: 22, rule: COMMA }
 * Token { input: "int", line: 3, column: 28, rule: TYPE }
 * Token { input: "a", line: 3, column: 32, rule: IDENTIFIER }
 * Token { input: "=", line: 3, column: 34, rule: ATTRIBUTION }
 * Token { input: "2", line: 3, column: 36, rule: NUM }
 * Token { input: ";", line: 3, column: 37, rule: COMMA }
 * Token { input: "#asdasd", line: 3, column: 38, rule: ERROR }
 * Token { input: "a", line: 4, column: 50, rule: IDENTIFIER }
 * Token { input: "++", line: 4, column: 51, rule: INCREMENT_DECREMENT }
 * Token { input: ";", line: 4, column: 53, rule: COMMA }
 * Token { input: "b", line: 5, column: 59, rule: IDENTIFIER }
 * Token { input: "+=", line: 5, column: 60, rule: ATTRIBUTION }
 * Token { input: "1", line: 5, column: 62, rule: NUM }
 * Token { input: ";", line: 5, column: 63, rule: COMMA }
 * Token { input: "}", line: 6, column: 66, rule: CLOSE_BRACES }
 * 
 * Vec<Token> Erros
 * Token { input: "#asdasd", line: 3, column: 38, rule: ERROR }
 */
pub fn get_tokens(input: &str) -> (Vec<Token>, Vec<Token>) {
    // let mut current_line: usize = 1;
    
    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<Token> = vec![];

    // let precompiled_input: String = precompile(input);
    // println!("{}", precompiled_input);

    // Faz o parsing da entrada e formatação de cada token
    match Lexer::parse(Rule::TOKEN, input) {
        Ok(pairs) => {
            for pair in pairs {
                let pos = pair.as_span().start_pos();
                //FIX TYPE
                let token = match &pair.as_rule() {
                    Rule::TYPE => Token::new(pair.as_str(), pos.line_col().0, pair.as_span().start(), pair.as_str().to_uppercase()),
                    Rule::ERROR => {
                        errors.push(Token::new(pair.as_str(), pos.line_col().0, pair.as_span().start(), pair.as_rule()));
                        Token::new(pair.as_str(), pos.line_col().0, pair.as_span().start(), pair.as_rule())
                    },
                    _ => Token::new(pair.as_str(), pos.line_col().0, pair.as_span().start(), pair.as_rule())
                }
                // println!("{:?}", token);
                tokens.push(token);
                if pair.as_rule() == Rule::ERROR {
                    errors.push(token);
                }

            }
        },
        Err(err) => println!("{err}") 
    } 

    // println!("{result}");

    // (errors, result)  
    (tokens, errors)
}

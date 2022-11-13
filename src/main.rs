/*
 * Pietro Vallardi Grohe
 * Compiladores
 * Etapa 2
 * //TODO Data
 * 
 * Dentro do cmd utilize dentro da pasta compiladores_final
 * "cargo build" e "cargo run" para compilar e rodar o programa 
 * Arquivo "tests.txt" contém diversas palavras que aceitam e rejeitam as regras de produção
 * BNF Encontrada no arquivo Grammar/etapa2.pest
 */
use std::{fs, num::NonZeroUsize, path::Path};
use pest::{Parser};

extern crate pest;
#[macro_use]
extern crate pest_derive;

// Macro para gerar o Parser e o Enum com as regras automáticamente a partir de um arquivo .pest
#[derive(Parser)] 
#[grammar = "Grammar/Grammar.pest"] 
pub struct Compiler;
//  Struct que conterá apenas a função parser

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    //  Define o limite de chamadas das regras não terminais
    pest::set_call_limit(NonZeroUsize::new(5000));
    
    print!("\n\n");

    /*
     * // Lê o caminho do arquivo
     * println!("File Path");
     * let mut file_path = String::new(); 
     * std::io::stdin().read_line(&mut file_path).expect("CANNOT READ INPUT");
     */
    let file_path = String::from("src\\tests.txt"); 
    // Tenta ler o arquivo para a variável 'file'

    let Ok(file_string) = fs::read_to_string(Path::new(&file_path.trim_end())) else {
        return println!("CANNOT OPEN FILE");
    };
    let precompilation = precompile(&file_string);
    let lexical = lexer(&precompilation);
    let syntactical = syntax(&lexical);
    println!("Precompiled: \n{}\n", precompilation);
    println!("Lexer: \n{}", lexical);
    println!("Syntax: \n{}", syntactical);

    // if let Ok(file) = fs::read_to_string(Path::new(&file_path.trim_end())) {

        

    //     println!("\n---------------------\n");
    //     // Separa o arquivo em linhas
    //     for (line_count, raw_token) in file.lines().enumerate() {  
    //         /*
    //         * Faz o parse da linha e retorna um Resultado.
    //         * Caso aceite a gramática retorna os pares reconhecidos e se rejeita retorna um erro
    //         */
    //         let parsed_line: Result<Pairs<Rule>, Error<Rule>> = Compiler::parse(Rule::Token, raw_token);
    //         match parsed_line {
    //             Ok(pairs) => {
    //                 for pair in pairs {
    //                     match pair.as_rule() {
    //                         Rule::Variable  
    //                         | Rule::Identifier
    //                         | Rule::Type 
    //                         | Rule::Char 
    //                         | Rule::Int 
    //                         | Rule::Float => println!("Line: {} = {:?}: \"{}\"", line_count+1, pair.as_rule(), pair.as_str()),
    //                         _ => println!("Não reconhecido \"{}\" Rule: {:?} Line: {}", pair.as_str(), pair.as_rule(), line_count+1)
    //                     }
    //                 }
    //             },
    //             Err(error) => {
    //                 println!("ERROR: Line: {}, ---> \"{}\"", line_count+1, error.line().to_owned());
    //             }
    //         }
    //     }
    //     print!("\n---------------------\n");

    // } else {
    //     println!("\nCANNOT OPEN FILE");
    // }

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
 * 
 */ 
fn precompile(input: &str) -> String {

    let mut result = String::new();

    input.lines().enumerate()
        .filter(|(_, e)| !e.trim().is_empty())
        .for_each(|(index, element)| result += &format!("[{}]{}", (index+1).to_string(), element.trim()));
    
    return result;
    
}

fn lexer(input: &str) -> String {

    mod lexer_func {
        #[derive(Parser)] 
        #[grammar = "Grammar/Lexer.pest"] 
        pub struct Lexer;
    }

    use lexer_func::*;

    let mut result: String = String::new();
    
    let parsed = Lexer::parse(Rule::TOKEN, input);
    match parsed {
        Ok(pairs) => {
            for pair in pairs {
                match &pair.as_rule() {
                    Rule::IDENTIFIER => result.push_str("<ID>"),
                    Rule::CONDITIONAL => result.push_str("<CND>"),
                    Rule::ATTRIBUTION => result.push_str("<ATTRIBUTION>"),
                    Rule::COMMA => result.push_str("<COMMA>"),
                    Rule::NUM => result.push_str("<NUM>"),
                    Rule::CHAR => result.push_str("<STRING>"),
                    Rule::ERROR => result.push_str("<ERROR>"),
                     _ => result.push_str(&format!("<{}>", pair.as_str().to_uppercase()))
                }
            }
        }
        Err(e) => println!("ERROR\n\n {}", e)
    }

    result
    
}

fn syntax(input: &str) -> String {

    //TODO ERROR HANDLING

    mod syntax_func {
        #[derive(Parser)]
        #[grammar = "Grammar/Syntax.pest"]
        pub struct Syntax;
    }
    use syntax_func::*;

    // let mut result = String::new();

    // <[1]><IF><(><ID><CND><ID><)><{><[2]><ID><ATTRIBUTION><NUM><COMMA><[3]><INT><[4]><ID><ATTRIBUTION><NUM><COMMA><[6]><}>
    let result = match Syntax::parse(Rule::IF_BLOCK, input) {
        Ok(_) => String::from("ACCEPTED"),
        Err(e) => e.to_string(),
    };

    result
}
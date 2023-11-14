use std::collections::HashMap;
use std::process::exit;

use ansi_term::Color;
use linefeed::{Interface, ReadResult};

use crate::interpreting::interpreter::interpret;
use crate::lexing::lexer::lex;
use crate::parsing::ast::Parameters;
use crate::parsing::parser::CalcParser;

mod interpreting;
mod lexing;
mod parsing;

fn main() {
    let message = Color::Blue.paint("Welcome to calc v2.0.0-rc1 by Charlotte Thomas \ntype help for getting help for the commands\n");
    println!("{}", message.to_string());

    let interface = Interface::new("calc").unwrap();
    let style = Color::Cyan;
    let text = "> ";
    let mut verbose = true;

    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{text}\x01{suffix}\x02",
            prefix = style.prefix(),
            text = text,
            suffix = style.suffix()
        ))
        .unwrap();
    let mut ram: HashMap<String, Parameters> = HashMap::new();
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        match line.as_str().trim() {
            "info" => {
                let message = Color::Purple.paint(" Calc v2.0.0-rc1 \n Author: Charlotte Thomas \n Written in Rust \n Repo: https://github.com/coco33920/calc\n");
                println!("{}", message)
            }
            "exit" => break,
            "help" => {
                let message = Color::Purple.paint(
                    " Calc v2.0.0-rc1 Help \n > info : show infos \n > exit : exit the program \n > help : print this help \n > verbose : toggle the verbose \n > version : prints the version \n"
                );
                println!("{}", message)
            }
            "version" => {
                let message = Color::Purple.paint(" Calc v2.0.0-rc1\n");
                println!("{}", message)
            }
            "verbose" => {
                verbose = !verbose;
                let message = Color::Purple.paint("You toggled the verbose : ");
                let message2 = Color::Red.paint(if verbose { "on" } else { "off" });
                println!("{}{}", message, message2)
            }
            str => {
                let a = lex(str.to_string());
                let parser: &mut CalcParser = &mut parsing::parser::init_calc_parser(&a);
                let p = parser.parse();
                if verbose {
                    println!("Lexing of line: {str}");
                    println!("{:?}", &a);
                    println!("Parsing of line: {str}");
                    println!("{:#?}", p);
                    println!()
                }
                let result = interpret(p.clone(), &mut ram);
                if result != Parameters::Null {
                    result.pretty_print(Some(&ram))
                }
            }
        }
        interface.add_history_unique(line);
    }
    exit(0);
}

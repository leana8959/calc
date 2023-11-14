use std::collections::HashMap;
use std::f64::consts::{E, PI};
use std::process::exit;

use crate::configuration::loader::{load, load_config, Loaded};
use ansi_term::Color;
use confy::ConfyError;
use linefeed::{Interface, ReadResult};

use crate::interpreting::interpreter::interpret;
use crate::lexing::lexer::lex;
use crate::parsing::ast::Parameters;
use crate::parsing::parser::CalcParser;

mod configuration;
mod interpreting;
mod lexing;
mod parsing;

fn main() {
    let config = match load() {
        Ok(config) => config,
        Err(e) => {
            println!("{e}");
            exit(1)
        }
    };

    let loaded: Loaded = load_config(config);


    let message = loaded.greeting_message;
    println!("{}", message.to_string());

    let interface = Interface::new("calc").unwrap();
    let style = loaded.prompt_style;
    let text = loaded.prompt;
    let mut verbose = false;

    interface
        .set_prompt(&format!(
            "\x01{prefix}\x02{text}\x01{suffix}\x02",
            prefix = style.prefix(),
            text = text,
            suffix = style.suffix()
        ))
        .unwrap();
    let mut ram: HashMap<String, Parameters> = HashMap::new();
    ram.insert("pi".to_string(), Parameters::Float(PI));
    ram.insert("e".to_string(), Parameters::Float(E));
    while let ReadResult::Input(line) = interface.read_line().unwrap() {
        match line.as_str().trim() {
            "info" => {
                let message = Color::Purple.paint(" Calc v2.1.1 \n Author: Charlotte Thomas \n Written in Rust \n Repo: https://github.com/coco33920/calc\n");
                println!("{}", message)
            }
            "exit" => break,
            "help" => {
                let message = Color::Purple.paint(
                    " Calc v2.1.0 Help \n > info : show infos \n > exit : exit the program \n > help : print this help \n > verbose : toggle the verbose \n > version : prints the version \n"
                );
                println!("{}", message)
            }
            "version" => {
                let message = Color::Purple.paint(" Calc v2.1.1\n");
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

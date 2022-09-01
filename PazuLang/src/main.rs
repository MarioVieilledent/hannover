use std::collections::HashMap;
use std::fs;

use regex::Regex;

const LANG_EXTENTION: &str = "pz";
const SEPARATOR: &str = ";";

#[derive(Debug)]
enum Type {
    B(bool),
    C(char),
    F(f64),
    I(isize),
    S(String),
    U(u8),
}

struct Interpretor {
    memory: HashMap<String, Type>,
}

impl Interpretor {
    fn interpret(&mut self, code: &String) {
        let code = code.replace("\n", "");
        let instructions = code.split(SEPARATOR);
        for (index, instr) in instructions.enumerate() {
            match self.interpret_line(index, instr) {
                Ok(_) => (),
                Err(s) => println!("Erreur : {}", s),
            };
        }
    }

    fn interpret_line(&mut self, index: usize, instr: &str) -> Result<String, String> {
        let words: Vec<&str> = instr.split(" ").collect::<Vec<&str>>();
        match check_syntax(&words) {
            Ok(_) => match words[0] {
                "v" => {
                    self.declare_variable(
                        words[1],
                        make_value(words[2].chars().next().unwrap(), words[4])?,
                    );
                    Ok(format!("Ok"))
                }
                "f" => self.declare_function("test", "test"),
                "p" => {
                    self.printer(get_params(words[0])?)?;
                    Ok(format!("Ok"))
                }
                other => {
                    return Err(format!(
                        "Syntaxe \"{}\" non reconnue {}",
                        other,
                        stack_trace(index)
                    ));
                }
            },
            Err(e) => Err(e),
        }
    }

    fn declare_variable(&mut self, name: &str, value: Type) {
        self.memory.insert(name.to_string(), value);
    }

    fn declare_function(&mut self, name: &str, content: &str) -> Result<String, String> {
        Ok("test".to_owned())
    }

    fn printer(&self, words: Vec<&str>) -> Result<(), String> {
        for param in words {
            match self.memory.get(param) {
                Some(v) => {
                    match v {
                        Type::I(i) => println!("{}", i),
                        Type::U(u) => println!("{}", u),
                        Type::C(c) => println!("{}", c),
                        Type::S(s) => println!("{}", s),
                        Type::F(f) => println!("{}", f),
                        Type::B(b) => {
                            if *b {
                                println!("t");
                            } else {
                                println!("f");
                            }
                        }
                    };
                }
                None => return Err(format!("Aucune variable {} n'existe en mémoire.", param)),
            }
        }
        Ok(())
    }

    fn _display_memory(&self) {
        for (key, value) in &self.memory {
            println!("{}: {:?}", key, value);
        }
    }
}

fn main() {
    let mut interpretor = Interpretor {
        memory: HashMap::new(),
    };
    interpretor.interpret(&read_file("test.pz").unwrap());
}

fn check_syntax(_instr: &Vec<&str>) -> Result<(), String> {
    Ok(())
}

fn stack_trace(index: usize) -> String {
    format!("ligne {}", index)
}

fn read_file(file_name: &str) -> Result<String, String> {
    if file_name.ends_with(format!(".{}", LANG_EXTENTION).as_str()) {
        Ok(fs::read_to_string(file_name).expect("Erreur lecture fichier"))
    } else {
        Err(format!(
            "Seul les fichiers .{} peuvent être interprétés.",
            LANG_EXTENTION
        ))
    }
}

fn get_params(instr: &str) -> Result<Vec<&str>, String> {
    let re = Regex::new(r"\w*?\((.*?)\)").unwrap();
    let cap = re.captures(instr);
    match cap {
        Some(c) => {
            let str = c.get(1).map_or("", |m| m.as_str());
            Ok(str.split(",").collect::<Vec<&str>>())
        }
        None => Err(format!("tkt")),
    }
}

fn make_value(t: char, v: &str) -> Result<Type, String> {
    let v = v.trim();
    match t {
        'i' => Ok(Type::I(v.parse::<isize>().unwrap())),
        'u' => Ok(Type::U(v.parse::<u8>().unwrap())),
        'c' => Ok(Type::C(v.parse::<char>().unwrap())),
        'b' => match v {
            "t" => return Ok(Type::B(true)),
            "f" => return Ok(Type::B(false)),
            v => {
                return Err(format!(
                    "{} n'est pas un booléen, utiliser t pour true et f pour false.",
                    v
                ));
            }
        },
        t => Err(format!("{} n'est pas un type.", t)),
    }
}

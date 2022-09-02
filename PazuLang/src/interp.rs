mod interpret;

use std::collections::HashMap;
use std::fs;

use regex::Regex;

const LANG_EXTENTION: &str = "pz";
const SEPARATOR: &str = "\r\n";

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
    output: String,
}

impl Interpretor {
    fn interpret(&mut self, code: &String) {
        // let code = code.replace("\r\n\r\n", "\r\n");
        let instructions = code.split(SEPARATOR);
        for (index, instr) in instructions.enumerate() {
            match self.interpret_line(index, instr) {
                Ok(_) => (),
                Err(s) => println!("{}", s),
            };
        }
    }

    fn interpret_line(&mut self, index: usize, instr: &str) -> Result<String, String> {
        let words: Vec<&str> = instr.trim().split(" ").collect::<Vec<&str>>();
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
                "//" => Ok(format!("Ok")),
                other => {
                    let re = Regex::new(r"^.*?\(.*?\)$").unwrap();
                    if re.is_match(&words[0]) {
                        self.printer(get_params(words[0])?)?;
                        Ok(format!("Ok"))
                    } else if words[0] == "" {
                        Ok(format!("Ok"))
                    } else {
                        return Err(format!(
                            "Syntax error: \"{}\" on line {}",
                            other,
                            stack_trace(index)
                        ));
                    }
                }
            },
            Err(e) => Err(e),
        }
    }

    fn declare_variable(&mut self, name: &str, value: Type) {
        self.memory.insert(name.to_string(), value);
    }

    fn declare_function(&mut self, _name: &str, _content: &str) -> Result<String, String> {
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
                None => return Err(format!("The variable {} does not exist.", param)),
            }
        }
        Ok(())
    }

    fn _display_memory(&self) {
        for (key, value) in &self.memory {
            println!("{}: {:?}", key, value);
        }
    }

    fn log(&mut self, log: &str) {
        self.output.push_str(log);
        self.output.push_str("\n");
    }
}

pub fn interpret_script(script: &str) -> String {
    let mut interpretor = Interpretor {
        memory: HashMap::new(),
        output: String::new(),
    };
    interpretor.interpret(&script.to_owned());
    return interpretor.output.to_owned();
}

fn check_syntax(_instr: &Vec<&str>) -> Result<(), String> {
    Ok(())
}

fn stack_trace(index: usize) -> String {
    format!("ligne {}", index + 1)
}

fn read_file(file_name: &str) -> Result<String, String> {
    if file_name.ends_with(format!(".{}", LANG_EXTENTION).as_str()) {
        Ok(fs::read_to_string(file_name).expect("Read file error."))
    } else {
        Err(format!(
            "Only .{} files can be interpreted.",
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
        None => Err(format!("Error trying to get params of function.")),
    }
}

fn make_value(t: char, v: &str) -> Result<Type, String> {
    let v: &str = v.trim();
    match t {
        'i' => Ok(Type::I(v.parse::<isize>().unwrap())),
        'u' => Ok(Type::U(v.parse::<u8>().unwrap())),
        'f' => Ok(Type::F(v.parse::<f64>().unwrap())),
        'c' => {
            if v.chars().nth(0).unwrap() == '\'' && v.chars().nth(2).unwrap() == '\'' {
                Ok(Type::C(v.chars().nth(1).unwrap()))
            } else {
                Err(format!("Syntax error declaring char."))
            }
        }
        's' => {
            if v.chars().nth(0).unwrap() == '"' && v.chars().nth(v.len() - 1).unwrap() == '"' {
                Ok(Type::S(format!("{}", &v[1..v.len() - 1])))
            } else {
                Err(format!("Syntax error declaring string."))
            }
        }
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

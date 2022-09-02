mod utils;

fn main() {
    let mut interpretor = Interpretor {
        memory: HashMap::new(),
        output: String::new(),
    };
    interpretor.interpret(&read_file("test.pz").unwrap());
}

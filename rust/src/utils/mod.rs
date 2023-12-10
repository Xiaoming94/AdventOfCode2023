use std::io::BufRead;

pub fn read_input() -> String {
    let input_io = std::io::stdin();
    let mut input_lines: Vec<String> = Vec::<String>::new();
    for line in input_io.lock().lines() {
        input_lines.push(line.unwrap());
    }

    input_lines.join("\n")
}

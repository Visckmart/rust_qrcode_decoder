pub fn visualize_code(code: &Vec<Vec<bool>>, color: bool) {
    let dimension = code.len();
    // println!("abcd");
    for _ in 0..(dimension+2)/2 {
        print!("━━┄┄");
    }
    println!();
    for (l, line) in code.iter().enumerate() {
        for (col, pixel) in line.iter().enumerate() {
            if col == 0 { print!("{}", if l % 2 == 0 { "\\ " } else { "/ "}); }
            if color {
                if col >= dimension - 2 && l >= dimension - 2 {
                    print!("\x1b[101m");
                } else if col >= dimension - 2 && l >= dimension - 2 - 4 {
                    print!("\x1b[104m");
                } else if (col + 1) % 4 <= 1 && !((l < 9 && (col < 9 || col >= code.len() - 8) || (l >= code.len() - 8 && col < 9)) || (col > code.len() - 7 - 3 && col < code.len() - 4 && l > code.len() - 7 - 3 && l < code.len() - 4) || (l == 6 || col == 6)) {
                    print!("\x1b[37m");
                }
            }
            print!("{}\x1b[0m", if *pixel { "▉▉" } else { "  " });
        }
        print!("{}", if l % 2 == 0 { " ┃" } else { " ┊" });
        println!();
    }
    // print!("  ");
    for _ in 0..(dimension+2)/2 {
        print!("  ~~");
    }
    println!();
}

pub fn matrix_from_str(src: String) -> Vec<Vec<bool>> {
    let width = (src.len() as f64).sqrt().floor() as usize;
    // println!("{}", width);
    let mut code: Vec<Vec<bool>> = Vec::new();
    let mut line: Vec<bool> = Vec::new();
    for (i, c) in src.chars().enumerate() {
        if i % width == 0 && i > 0 {
            code.push(line.clone());
            line.clear();
        }
        line.push(c == '1');
    }
    code.push(line.clone());
    line.clear();
    code
}
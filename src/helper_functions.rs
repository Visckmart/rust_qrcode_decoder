pub fn visualize_code(code: &Vec<Vec<bool>>) {
    let dimension = code.len();
    print!(" ");
    for _ in 0..dimension+2 {
        print!("-");
    }
    println!();
    for line in code {
        for (col, pixel) in line.iter().enumerate() {
            if col == 0 { print!("| "); }
            print!("{}", if *pixel { "▉" } else { " " });
        }
        print!(" |");
        println!();
    }
    print!(" ");
    for _ in 0..dimension+2 {
        print!("-");
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
mod helper_functions;

use std::ops::BitXor;
use crate::helper_functions::*;

fn main() {
    // let code = matrix_from_str(String::from("\
    // 11111110110000011001001111111\
    // 10000010101111000111101000001\
    // 10111010111100010011001011101\
    // 10111010110101011010101011101\
    // 10111010110100011011001011101\
    // 10000010010001011111101000001\
    // 11111110101010101010101111111\
    // 00000000100000100101000000000\
    // 01101011011101000101101011111\
    // 11011000111010010010101000101\
    // 11001011101111111010100001111\
    // 11000101101100010101111101010\
    // 00000010011010100100101100000\
    // 01111000011111011110111100011\
    // 00111010101111010000001100111\
    // 10100000000111101100111100001\
    // 10000110110101010100011101010\
    // 00110000101001010100111000111\
    // 10101011100011100000110111011\
    // 01110101010011110111101100011\
    // 10001011110101001100111110010\
    // 00000000100111001101100010011\
    // 11111110111000101001101010111\
    // 10000010011011101110100010001\
    // 10111010111110011010111111001\
    // 10111010010111111100110011000\
    // 10111010100111111111000110001\
    // 10000010111010001100100100010\
    // 11111110001011111111111010011"));
    // let code = matrix_from_str("1111111011000010011000111111110000010100101011010001000001101110101101111101110010111011011101010001101111000101110110111010101011000010101011101100000100110010101010010000011111111010101010101010111111100000000100101101111000000000011010110001011000100010111110000100100001100111100010101010100010101010100010011111100111100000111100101110111010100111111001111000101001101110100000100000000100010100000010011001111100000101011101001101011010111001011000011100101110000111011101100000110011010010011000000110100011100101101010011001011010010111100001101100101110111110000101010111101100111001001010011111101000000000011111111110110001000011111110111111111010101010001100000100001100011101000101001011101010011111101011111100110111010011001111111100111000101110101010011110111111011011000001010010110111001011100111111110001101110010111111111".to_string());
    // let code = matrix_from_str("1111111010101111001110111111110000010100111001100001000001101110100110000010110010111011011101010111011010000101110110111010011110101101001011101100000100011110110011010000011111111010101010101010111111100000000110010011110100000000101101110111010111110010010110001100010010111001111111001100110010001111001100000001110101101011111000010110110000110000011011000011010000000011001011001001100111101001000101011011111111110000011011000110010110000000011000000001100010011010101000100010110110000010010011010101100001001010001001101011110111101011011100000100000000101100110110101110011001100001111011111111111010000000011101000101010001001111111110101100101101101011110100000101110011010101000100001011101001011001010011111011010111010101101011101110110101101110101010000000010001000011000001000001011000011011101011111110110001000011110100010".to_string());
    // let code = matrix_from_str("1111111010101111001110111111110000010100111001100001000001101110100110000010110010111011011101010111011010000101110110111010011110101101001011101100000100011110110011010000011111111010101010101010111111100000000110010011110100000000101101110111010111110010010110001100010010111001111111001100110010001111001100000001110101101011111000010110110000110000011011000011010000000011001011001001100111101001000101011011111111110000011011000110010110000000011000000001100010011010101000100010110110000010010011010101100001001010001001101011110111101011011100000100000000101100110110101110011001100001111011111111111010000000011101000101010001001111111110101100101101101011110100000101110011010101000100001011101001011001010011111011010111010101101011101110110101101110101010000000010001000011000001000001011000011011101011111110110001000011110100010".to_string());
    let code = matrix_from_str("1111111001000100100110111111110000010110111111000101000001101110100111001000100010111011011101011010111010100101110110111010001010101111101011101100000101010010100000010000011111111010101010101010111111100000000000010010001000000000111110111110111101001101010100001110001000010011111111000110000010010111011100100010100001101011111000100011001110001100111011011111011101010011000101001011010001011111111011100000111100001100101100010000011110100100010101111010101001100011011011100100000000110111110000101001010111111111011000001010101011110000111010010111101110000001010101111000100101111010011001011111111100000000011010000100010001101011111110111010011001101010000100000100111001100011000110011011101010011110010011111010110111010110101001111010001011101110101000001111000111110101000001010000010001011011101011111110111011110101010100100".to_string());
    visualize_code(&code, false);
    run_decoding_algorithm(&code);
    let v1 = vec![6, 22, 38];
    let combs = comb(&v1, 2);
    let dimension = 45;
    let check = |x: usize, y: usize| {
        let finder_patterns = y < 9 && (x < 9 || x >= dimension - 8) || (y >= dimension - 8 && x < 9);
        // let small_finder_pattern = x > dimension - 7 - 3 && x < dimension - 4 && y > dimension - 7 - 3 && y < dimension - 4;
        let dots = y == 6 || x == 6;
        // println!("\t\t{}, {}\t{}", x, y, finder_patterns);
        // println!("\t\t{}, {}\t{}", x, y, dots);
        if finder_patterns {
            return false;
        }
        return true;
    };

    // for i in v1 {
    //     println!("({}, {})\t{}", i, i, check(i, i));
    // }
    // for (ind, i) in combs.iter().enumerate() {
    //     // if () {
    //         println!("({}, {})\t{}", i[0], i[1], check(i[0], i[1]));
    //     // }
    //     // if () {
    //         println!("({}, {})\t{}", i[1], i[0], check(i[1], i[0]));
    //     // }
    //     // println!("\t{}", ind);
    //     // println!("{}\t{}", v1[ind], v1[ind]);
    //     // for j in i.clone() {
    //     //     print!("{}\t", j)
    //     // }
    //     // println!("{:?}", i);
    //     // println!();
    //     // for j in i.iter().rev() {
    //     //     print!("{}\t", j)
    //     // }
    //     // println!();
    // }
    // println!("{:?}", get_alignment_patterns());
}

enum RestrictedArea {
    FinderPattern, AlignmentPattern, Dots, Outside
}

fn get_alignment_patterns(version: usize) -> Vec<(usize, usize)> {
    let alignment_module_coordinates = [
        vec![6, 18],
        vec![6, 22],
        vec![6, 26],
        vec![6, 30],
        vec![6, 34],
        vec![6, 22, 38],
        vec![6, 24, 42],
        vec![6, 26, 46],
        vec![6, 28, 50],
        vec![6, 30, 54],
        vec![6, 32, 58],
        vec![6, 34, 62],
        vec![6, 26, 46, 66],
        vec![6, 26, 48, 70],
        vec![6, 26, 50, 74],
        vec![6, 30, 54, 78],
        vec![6, 30, 56, 82],
        vec![6, 30, 58, 86],
        vec![6, 34, 62, 90],
        vec![6, 28, 50, 72, 94],
        vec![6, 26, 50, 74, 98],
        vec![6, 30, 54, 78, 102],
        vec![6, 28, 54, 80, 106],
        vec![6, 32, 58, 84, 110],
        vec![6, 30, 58, 86, 114],
        vec![6, 34, 62, 90, 118],
        vec![6, 26, 50, 74, 98, 122],
        vec![6, 30, 54, 78, 102, 126],
        vec![6, 26, 52, 78, 104, 130],
        vec![6, 30, 56, 82, 108, 134],
        vec![6, 34, 60, 86, 112, 138],
        vec![6, 30, 58, 86, 114, 142],
        vec![6, 34, 62, 90, 118, 146],
        vec![6, 30, 54, 78, 102, 126],
        vec![6, 24, 50, 76, 102, 128],
        vec![6, 28, 54, 80, 106, 132],
        vec![6, 32, 58, 84, 110, 136],
        vec![6, 26, 54, 82, 110, 138],
        vec![6, 30, 58, 86, 114, 142]];

    let coordinates = alignment_module_coordinates.get(version - 2).unwrap();
    let coord_combinations = comb(&coordinates, 2);
    let mut pattern_coordinates = Vec::new();
    for i in coordinates {
        pattern_coordinates.push((*i, *i));
    }
    for i in coord_combinations {
        pattern_coordinates.push((*i.get(0).unwrap(), *i.get(1).unwrap()));
        pattern_coordinates.push((*i.get(1).unwrap(), *i.get(0).unwrap()));
    }

    let mut finished_coordinates = Vec::new();
    for coord in pattern_coordinates {
        for x_offset in -2..=2 {
            for y_offset in -2..=2 {
                let x_new = coord.0 as isize;
                let y_new = coord.0 as isize;
                finished_coordinates.push(((x_new + x_offset) as usize, (y_new + y_offset) as usize));
            }
        }
    }
    finished_coordinates
}

fn is_restricted_position(x: usize, y: usize, dimension: usize) -> Option<RestrictedArea> {
    if x < 0 || x > dimension || y < 0 || y > dimension {
        return Some(RestrictedArea::Outside);
    }

    let in_finder_patterns = (y < 9 && (x < 9 || x >= dimension - 8))
                             || (y >= dimension - 8 && x < 9);
    if in_finder_patterns {
        return Some(RestrictedArea::FinderPattern);
    }

    let in_dots = x == 6 || y == 6;
    if in_dots {
        return Some(RestrictedArea::Dots);
    }
    let alignment_patterns = get_alignment_patterns(3);
    if alignment_patterns.contains(&(x, y)) {
        return Some(RestrictedArea::AlignmentPattern);
    }
    None
}
fn comb<T>(slice: &[T], k: usize) -> Vec<Vec<T>>
    where
        T: Copy,
{
    // If k == 1, return a vector containing a vector for each element of the slice.
    if k == 1 {
        return slice.iter().map(|x| vec![*x]).collect::<Vec<Vec<T>>>();
    }
    // If k is exactly the slice length, return the slice inside a vector.
    if k == slice.len() {
        return vec![slice.to_vec()];
    }

    // Make a vector from the first element + all combinations of k - 1 elements of the rest of the slice.
    let mut result = comb(&slice[1..], k - 1)
        .into_iter()
        .map(|x| [&slice[..1], x.as_slice()].concat())
        .collect::<Vec<Vec<T>>>();

    // Extend this last vector with the all the combinations of k elements after from index 1 onward.
    result.extend(comb(&slice[1..], k));
    // Return final vector.
    return result;
}

fn codeword_trail(dimension: usize) -> Vec<(usize, usize)> {
    let mut trail: Vec<(usize, usize)> = Vec::new();
    let check = |x: usize, y: usize| {
        let finder_patterns = y < 9 && (x < 9 || x >= dimension - 8) || (y >= dimension - 8 && x < 9);
        let small_finder_pattern = x > dimension - 7 - 3 && x < dimension - 4 && y > dimension - 7 - 3 && y < dimension - 4;
        let dots = y == 6 || x == 6;

        if finder_patterns || small_finder_pattern || dots {
            return false;
        }
        return true;
    };

    let leftmost_codewords_x = (0..5).step_by(2);
    let general_codewords_x = (7..dimension).step_by(2);
    let all_codewords_x = leftmost_codewords_x.chain(general_codewords_x);

    for x in all_codewords_x.rev() {
        let y_range: Vec<usize> =
            if ((x < 5) && (x % 4 == 0)) || ((x > 5) && (x % 4 == 1)) {
                (0..dimension).collect()
            } else {
                (0..dimension).rev().collect()
            };
        for y in y_range {
            // Checar se pode colocar na posição atual, senão pula
            if is_restricted_position(x, y, dimension).is_some() { continue; }
            // Checar se pode inserir na posição à direita, se sim, insere
            if is_restricted_position(x + 1, y, dimension).is_none() { trail.push((x + 1, y)); }
            // Inserir na posição atual
            trail.push((x, y));
        }
    }
    trail.remove(0);
    trail.remove(0);
    trail.remove(0);
    trail.remove(0);
    // for (x, y) in trail.iter() {
    //     println!("({}, {})", x, y);
    // }
    trail
}

fn run_decoding_algorithm(code: &Vec<Vec<bool>>) {
    let check = check_finder_patterns(code);
    println!("Check {}", check);
    if !check {
        println!("QR code is invalid.");
        return;
    }

    println!("{:?}", get_error_correction_level(&code));
    let mask_pattern = get_mask_pattern(&code);
    println!("{:?}", mask_pattern);
    let mask_pattern_checker = get_mask_checker(mask_pattern);
    let unmasked_code = remove_mask(&code, mask_pattern_checker);
    visualize_code(&unmasked_code, true);
    let (size, content) = read_codewords(&unmasked_code);
    println!("{}\n{}", size, content);
}

fn check_finder_patterns(code: &Vec<Vec<bool>>) -> bool {
    let dimension = code.len();
    let ending_limit = dimension - 7;
    let finder_pattern = matrix_from_str(
        String::from("\
                    1111111\
                    1000001\
                    1011101\
                    1011101\
                    1011101\
                    1000001\
                    1111111")
    );

    // TODO: Poderia ir logo para as linhas que interessam
    for (line_index, line) in code.iter().enumerate() {
        for (column_index, pixel) in line.iter().enumerate() {
            let mut x = column_index;
            let mut y = line_index;

            if column_index >= ending_limit { x -= ending_limit; }
            else if line_index >= ending_limit { y -= ending_limit; }

            if x < 7 && y < 7 {
                let expected_pixel = finder_pattern[y][x];
                if *pixel != expected_pixel { return false; }
            }
        }
    }
    return true;
}

#[derive(Debug)]
enum ErrorCorrectionLevel {
    Low, Medium, Quartile, High
}
fn get_error_correction_level(code: &Vec<Vec<bool>>) -> ErrorCorrectionLevel {
    let error_correction_bits = (&code[8][0], &code[8][1]);
    match error_correction_bits {
        (true, true) => ErrorCorrectionLevel::Low,
        (true, false) => ErrorCorrectionLevel::Medium,
        (false, true) => ErrorCorrectionLevel::Quartile,
        (false, false) => ErrorCorrectionLevel::High,
    }
}

fn get_mask_pattern(code: &Vec<Vec<bool>>) -> (bool, bool, bool) {
    (*&code[8][2], *&code[8][3], *&code[8][4])
}

fn quadriculado(x: usize, y: usize) -> bool { (x + y)%2 == 0 }
fn pula_linhas(_x: usize, y: usize) -> bool  { y % 2 == 0 }
fn pula_colunas(x: usize, _y: usize) -> bool { x % 3 == 0 }
fn diagonais(x: usize, y: usize) -> bool    { (x + y)%3 == 0 }
fn quadriculado_maior(x: usize, y: usize) -> bool { (y/2 + x/3)%2 == 0 }
fn flor(x: usize, y: usize) -> bool               { (x*y)%2 + (x*y)%3 == 0 }
fn elipses(x: usize, y: usize) -> bool            { ((x * y)%3 + (x * y))%2 == 0 }
fn faixas_diagonais(x: usize, y: usize) -> bool   { ((x * y)%3 + (x + y))%2 == 0 }

fn get_mask_checker(pattern: (bool, bool, bool)) -> impl Fn(&Vec<Vec<bool>>, usize, usize) -> bool {
    let correct_mask = match pattern {
        (true, true, true) => pula_colunas,
        (true, true, false) => diagonais,
        (true, false, true) => quadriculado,
        (true, false, false) => pula_linhas,
        (false, true, true) => elipses,
        (false, true, false) => faixas_diagonais,
        (false, false, true) => quadriculado_maior,
        (false, false, false) => flor
    };

    return move |code: &Vec<Vec<bool>>, x: usize, y: usize| {
        let finder_patterns = y < 9 && (x < 9 || x >= code.len() - 8) || (y >= code.len() - 8 && x < 9);
        let small_finder_pattern = x > code.len() - 7 - 3 && x < code.len() - 4 && y > code.len() - 7 - 3 && y < code.len() - 4;
        let dots = y == 6 || x == 6;

        if finder_patterns || small_finder_pattern || dots {
            return false;
        }
        correct_mask(x, y)
    }

}

#[allow(unused_variables)]
fn remove_mask(code: &Vec<Vec<bool>>, mask: impl Fn(&Vec<Vec<bool>>, usize, usize) -> bool) -> Vec<Vec<bool>> {
    let mut unmasked_code = code.clone();

    for (y, line) in code.iter().enumerate() {
        for (x, pixel) in line.iter().enumerate() {
            unmasked_code[y][x] = mask(&code, x, y).bitxor(code[y][x])
        }
    }
    unmasked_code
}

fn read_codewords(code: &Vec<Vec<bool>>) -> (usize, String) {
    let trail = codeword_trail(code.len());
    visualize_trail(&code, &trail);

    let mut value_list = Vec::new();
    let mut value = 0;

    for (i, (x, y)) in trail.iter().enumerate() {
        let pot = (7 - (i % 8)) as u32;
        if code[*y][*x] {
            value += (2 as u32).pow(pot);
        }
        if pot == 0 {
            value_list.push(value);
            value = 0;
        }
    }
    let total = value_list[0] as usize;
    let mut content = String::new();
    for val in &value_list[1..total+1] {
        content.push_str(((*val as u8) as char).to_string().as_str());
    }
    (total, content)
}
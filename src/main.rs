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
    let code = matrix_from_str("1111111010101111001110111111110000010100111001100001000001101110100110000010110010111011011101010111011010000101110110111010011110101101001011101100000100011110110011010000011111111010101010101010111111100000000110010011110100000000101101110111010111110010010110001100010010111001111111001100110010001111001100000001110101101011111000010110110000110000011011000011010000000011001011001001100111101001000101011011111111110000011011000110010110000000011000000001100010011010101000100010110110000010010011010101100001001010001001101011110111101011011100000100000000101100110110101110011001100001111011111111111010000000011101000101010001001111111110101100101101101011110100000101110011010101000100001011101001011001010011111011010111010101101011101110110101101110101010000000010001000011000001000001011000011011101011111110110001000011110100010".to_string());
    visualize_code(&code, false);
    run_decoding_algorithm(&code);
}

fn run_decoding_algorithm(code: &Vec<Vec<bool>>) {
    let check = check_finder_patterns(code);
    println!("Check {}", check);
    println!("{:?}", get_error_correction_level(&code));
    // println!("{:?}", get_mask_pattern(&code));
    let unmasked_code = remove_mask(&code, get_mask_pattern(&code));
    visualize_code(&unmasked_code, true);
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

fn quadriculado(x: usize, y: usize) -> bool { (x + y)%2 == 0 }
fn pula_linhas(x: usize, y: usize) -> bool  { y % 2 == 0 }
fn pula_colunas(x: usize, y: usize) -> bool { x % 3 == 0 }
fn diagonais(x: usize, y: usize) -> bool    { (x + y)%3 == 0 }
fn quadriculado_maior(x: usize, y: usize) -> bool { (y/2 + x/3)%2 == 0 }
fn flor(x: usize, y: usize) -> bool               { (x*y)%2 + (x*y)%3 == 0 }
fn elipses(x: usize, y: usize) -> bool            { ((x * y)%3 + (x * y))%2 == 0 }
fn faixas_diagonais(x: usize, y: usize) -> bool   { ((x * y)%3 + (x + y))%2 == 0 }

fn get_mask_pattern(code: &Vec<Vec<bool>>) -> impl Fn(&Vec<Vec<bool>>, usize, usize) -> bool {
    let correct_mask = match (*&code[8][2], *&code[8][3], *&code[8][4]) {
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
        for (x, mut pixel) in line.iter().enumerate() {
            unmasked_code[y][x] = mask(&code, x, y).bitxor(code[y][x])
        }
    }
    unmasked_code
}
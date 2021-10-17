mod helper_functions;

use std::ops::BitXor;
use crate::helper_functions::*;

fn main() {
    // Exemplo cujo conteúdo é 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' (34 caracteres 'a')
    let exemplo_a = matrix_from_str("1111111010101111001110111111110000010100111001100001000001101110100110000010110010111011011101010111011010000101110110111010011110101101001011101100000100011110110011010000011111111010101010101010111111100000000110010011110100000000101101110111010111110010010110001100010010111001111111001100110010001111001100000001110101101011111000010110110000110000011011000011010000000011001011001001100111101001000101011011111111110000011011000110010110000000011000000001100010011010101000100010110110000010010011010101100001001010001001101011110111101011011100000100000000101100110110101110011001100001111011111111111010000000011101000101010001001111111110101100101101101011110100000101110011010101000100001011101001011001010011111011010111010101101011101110110101101110101010000000010001000011000001000001011000011011101011111110110001000011110100010".to_string());
    // Exemplo cujo conteúdo é 'Aparentemente o algoritmo funciona corretamente'
    let exemplo_funcionamento = matrix_from_str("1111111001000100100110111111110000010110111111000101000001101110100111001000100010111011011101011010111010100101110110111010001010101111101011101100000101010010100000010000011111111010101010101010111111100000000000010010001000000000111110111110111101001101010100001110001000010011111111000110000010010111011100100010100001101011111000100011001110001100111011011111011101010011000101001011010001011111111011100000111100001100101100010000011110100100010101111010101001100011011011100100000000110111110000101001010111111111011000001010101011110000111010010111101110000001010101111000100101111010011001011111111100000000011010000100010001101011111110111010011001101010000100000100111001100011000110011011101010011110010011111010110111010110101001111010001011101110101000001111000111110101000001010000010001011011101011111110111011110101010100100".to_string());

    let escolhido = exemplo_funcionamento;
    visualize_code(&escolhido, false);
    run_decoding_algorithm(&escolhido);
}

fn run_decoding_algorithm(code: &Vec<Vec<bool>>) {
    // Validar as finder patterns
    if check_finder_patterns(code) {
        println!("Validated the finder patterns");
    } else {
        println!("There are invalid finder patterns");
        return;
    }

    // Obter o nível de correção de erros
    let error_correction_level = get_error_correction_level(&code);
    println!("Error correction level is {:?}", error_correction_level);

    // Obter o padrão da máscara
    let mask_pattern = get_mask_pattern(&code);
    println!("The mask pattern is {:?}", mask_pattern);

    // Obter a máscara
    let mask_pattern_checker = get_mask_checker(mask_pattern);
    // Remover a máscara do código
    let unmasked_code = remove_mask(&code, mask_pattern_checker);

    // Visualizar o código sem máscara
    visualize_code(&unmasked_code, true);

    // Decodificar e exibir o conteúdo
    let (size, content) = read_codewords(&unmasked_code);
    println!("Tamanho: {}", size);
    println!("Conteúdo: {}", content);
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

fn is_at_finder_pattern(x: usize, y: usize, dimension: usize) -> bool {
    (y < 9 && (x < 9 || x >= dimension - 8)) || (y >= dimension - 8 && x < 9)
}

const DIMENSIONS: [usize; 40] = [
    21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61, 65, 69, 73, 77, 81, 85, 89, 93, 97, 101, 105, 109,
    113, 117, 121, 125, 129, 133, 137, 141, 145, 149, 153, 157, 161, 165, 169, 173, 177
];
fn get_version(dimension: usize) -> usize {
    let index = DIMENSIONS.iter()                        // Para cada dimensão
                    .position(|dim| *dim == dimension)    // Comparar com a dimensão recebida
                    .expect("Invalid code dimension.");  // Se todas são diferentes, parar a execução
    index + 1   // Deslocar o resultado e retornar
}

const ALIGNMENT_MODULE_COORDINATES: [&[usize]; 39] = [
    &[6, 18], &[6, 22],     &[6, 26], &[6, 30],     &[6, 34], &[6, 22, 38], &[6, 24, 42],
    &[6, 26, 46],           &[6, 28, 50],           &[6, 30, 54],           &[6, 32, 58],
    &[6, 34, 62],           &[6, 26, 46, 66],       &[6, 26, 48, 70],       &[6, 26, 50, 74],
    &[6, 30, 54, 78],       &[6, 30, 56, 82],       &[6, 30, 58, 86],       &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],   &[6, 26, 50, 74, 98],   &[6, 30, 54, 78, 102],  &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],  &[6, 30, 58, 86, 114],  &[6, 34, 62, 90, 118],  &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126], &[6, 26, 52, 78, 104, 130], &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138], &[6, 30, 58, 86, 114, 142], &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126], &[6, 24, 50, 76, 102, 128], &[6, 28, 54, 80, 106, 132],
    &[6, 32, 58, 84, 110, 136], &[6, 26, 54, 82, 110, 138], &[6, 30, 58, 86, 114, 142]
];

fn is_at_alignment_pattern(x: usize, y: usize, dimension: usize) -> bool {
    // Obter a versão do código
    let version = get_version(dimension);

    // Obter uma referência às coordenadas da versão apropriada
    let coordinates = ALIGNMENT_MODULE_COORDINATES[version - 2];

    // Obter as combinações 2 a 2 das coordenadas
    let point = (x as isize, y as isize);
    for (module_x, module_y) in coordinates.iter().zip(coordinates) {
        // Se o centro do módulo estiver em uma alignment pattern, então devemos ignorar ele
        if is_at_finder_pattern(*module_x, *module_y, dimension) { continue; }

        // Transformando em isize para somar com os valores dos loops
        let module_x = *module_x as isize;
        let module_y = *module_y as isize;
        for x_offset in -2..=2 {
            for y_offset in -2..=2 {
                if point == ((module_x + x_offset), (module_y + y_offset)) { return true; }
                if point == ((module_y + x_offset), (module_x + y_offset)) { return true; }
            }
        }
    }

    return false;
}

fn is_restricted_position(x: usize, y: usize, dimension: usize) -> bool {
    // Checar se a posição está dentro da área do código
    if x > dimension || y > dimension { return true; }

    // Checar se a posição está em cima de uma finder pattern
    if is_at_finder_pattern(x, y, dimension) { return true; }

    // Checar se a posição está em cima das linhas pontilhadas
    let in_alignment_dots = x == 6 || y == 6;
    if in_alignment_dots { return true; }

    // Checar se a posição está em cima das alignment patterns
    if is_at_alignment_pattern(x, y, dimension) { return true; }

    return false;
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

fn get_mask_checker(pattern: (bool, bool, bool)) -> impl Fn(usize, usize, usize) -> bool {
    let correct_mask = match pattern {
        (true,  true,  true) => pula_colunas,
        (true,  true,  false) => diagonais,
        (true,  false, true) => quadriculado,
        (true,  false, false) => pula_linhas,
        (false, true,  true) => elipses,
        (false, true,  false) => faixas_diagonais,
        (false, false, true) => quadriculado_maior,
        (false, false, false) => flor
    };

    return move |x: usize, y: usize, dimension: usize| -> bool {
        if is_restricted_position(x, y, dimension) { return false; }
        return correct_mask(x, y);
    }
}

fn remove_mask(code: &Vec<Vec<bool>>, mask: impl Fn(usize, usize, usize) -> bool) -> Vec<Vec<bool>> {
    let mut unmasked_code = code.clone();
    for (y, line) in code.iter().enumerate() {
        for x in 0..line.len() {
            unmasked_code[y][x] = mask(x, y, code.len()).bitxor(code[y][x])
        }
    }
    unmasked_code
}

fn codeword_trail(dimension: usize) -> Vec<(usize, usize)> {
    let mut trail: Vec<(usize, usize)> = Vec::new();

    // Intervalo das coordenadas x das codewords da esquerda
    let leftmost_codewords_x = (0..5).step_by(2);
    // Intervalo das coordenadas x das codewords do resto do código
    let general_codewords_x = (7..dimension).step_by(2);
    // Intervalo das coordenadas x de todas as codewords
    let all_codewords_x = leftmost_codewords_x.chain(general_codewords_x);

    // Para cada x da lista de coordenadas x codewords de trás pra frente
    for x in all_codewords_x.rev() {
        // O intervalo de coordenadas y é para cima ou para baixo dependendo da situação
        let y_range: Vec<usize> =
            if ((x < 5) && (x % 4 == 0)) || ((x > 5) && (x % 4 == 1)) {
                (0..dimension).collect()
            } else {
                (0..dimension).rev().collect()
            };

        // Para cada y do intervalo de coordenadas y
        for y in y_range {
            // Checar se pode colocar na posição atual, senão pula
            if is_restricted_position(x, y, dimension) { continue; }
            // Checar se pode inserir na posição à direita, se sim, inserir
            if !is_restricted_position(x + 1, y, dimension) { trail.push((x + 1, y)); }
            // Inserir na posição atual
            trail.push((x, y));
        }
    }

    // Remover os 4 primeiros elementos da lista (bits de codificação)
    for _ in 0..4 {
        trail.remove(0);
    }
    // for (x, y) in trail.iter() {
    //     println!("({}, {})", x, y);
    // }
    trail
}

fn read_codewords(code: &Vec<Vec<bool>>) -> (usize, String) {
    // Obter a trilha de codewords
    let trail = codeword_trail(code.len());
    // Mostrar a trilha de codewords
    visualize_trail(&code, &trail);

    let mut value_list = Vec::new();
    let mut value = 0;

    for (i, (x, y)) in trail.iter().enumerate() {
        // Obter o valor da posição
        let pot = (7 - (i % 8)) as u32;
        // Se o pixel estiver ativo, somar o valor dele ao acumulador
        if code[*y][*x] {
            value += (2 as u32).pow(pot);
        }
        // Ao fim de cada codeword adicionar o acumulador na lista e reiniciá-lo
        if pot == 0 {
            value_list.push(value);
            value = 0;
        }
    }
    // Obter o total de bytes
    let total = value_list[0] as usize;
    // Converter a lista de inteiros em uma string com o conteúdo
    let mut content = String::new();
    for val in &value_list[1..total+1] {
        let char_from_val = char::from_u32(*val).unwrap();
        content.push_str(&char_from_val.to_string());
    }
    // Retornar a tupla com o total e o conteúdo
    (total, content)
}
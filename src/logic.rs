use rand::Rng;
#[derive(Clone, Copy, Debug)]
pub enum EndGame {
    Win,
    Lose,
    Draw,
    None,
}

pub fn checker(mut struct1: EndGame, vector: &Vec<char>, turn: usize) -> EndGame {
    if vector[0] == 'X' && vector[1] == 'X' && vector[2] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[3] == 'X' && vector[4] == 'X' && vector[5] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[6] == 'X' && vector[7] == 'X' && vector[8] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[0] == 'X' && vector[3] == 'X' && vector[6] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[1] == 'X' && vector[4] == 'X' && vector[7] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[2] == 'X' && vector[5] == 'X' && vector[8] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[0] == 'X' && vector[4] == 'X' && vector[8] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[2] == 'X' && vector[4] == 'X' && vector[6] == 'X' {
        struct1 = EndGame::Win;
        return struct1;
    } else if vector[0] == 'O' && vector[1] == 'O' && vector[2] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[3] == 'O' && vector[4] == 'O' && vector[5] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[6] == 'O' && vector[7] == 'O' && vector[8] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[0] == 'O' && vector[3] == 'O' && vector[6] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[1] == 'O' && vector[4] == 'O' && vector[7] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[2] == 'O' && vector[5] == 'O' && vector[8] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[0] == 'O' && vector[4] == 'O' && vector[8] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if vector[2] == 'O' && vector[4] == 'O' && vector[6] == 'O' {
        struct1 = EndGame::Lose;
        return struct1;
    } else if turn == 5 {
        struct1 = EndGame::Draw;
        return struct1;
    } else {
        struct1 = EndGame::None;
        return struct1;
    };
}

pub fn cpu_player(vector: &Vec<char>) -> usize {
    loop {
        let random = rand::thread_rng().gen_range(1..=9);
        if vector[random - 1] == ' ' {
            break random;
        }
    }
}

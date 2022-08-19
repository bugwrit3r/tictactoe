use ncurses::*;
use std::fmt;

use tictactoe::logic::*;

fn main() {
    let mut field_vec = vec![' '; 9];
    let init_board = print_board(&field_vec);
    let mut start = false;
    let mut ypos = 0;
    let mut turn = 0;
    let mut field_num = 0;
    let mut game_result = EndGame::None;
    initscr();
    cbreak();

    noraw();

    addstr("Welcome to TicTacToe.\nType 's' to play, type 'q' to quit at any point.");
    refresh();

    mvaddstr(2, 0, "Input: ");
    mvaddstr(4, 1, init_board.as_str());

    refresh();

    loop {
        if turn == 0 || turn == 1 {
            ypos = 2;
        } else {
            ypos = 1;
        }
        let input_win = newwin(1, 2, ypos, 7);
        let key = wgetch(input_win);

        match key as u8 as char {
            '1' => field_num = 1,
            '2' => field_num = 2,
            '3' => field_num = 3,
            '4' => field_num = 4,
            '5' => field_num = 5,
            '6' => field_num = 6,
            '7' => field_num = 7,
            '8' => field_num = 8,
            '9' => field_num = 9,
            'q' => break,
            's' => {
                if start == false {
                    start = true;
                    clear();
                } else {
                    continue;
                }
            }
            _ => continue,
        };

        if start {
            if turn == 0 {
                mvaddstr(0, 0, "You play as X");
                mvaddstr(1, 0, "Enter a number between 1 and 9.  ");
                mvaddstr(2, 0, "Input: ");
                mvaddstr(4, 1, &init_board);
            }

            mv(1, 7);
            refresh();

            if turn == 1 {
                clear();
                mvaddstr(0, 0, "Enter a number between 1 and 9.  ");
                mvaddstr(1, 0, "Input: ");
                mv(1, 7);
                refresh();
            }

            if turn != 0 {
                if &field_vec[field_num - 1] != &'X' && &field_vec[field_num - 1] != &'O' {
                    field_vec[field_num - 1] = 'X';

                    if turn != 5 {
                        let cpu_move = cpu_player(&field_vec);
                        field_vec[cpu_move - 1] = 'O';
                    }

                    let board = print_board(&field_vec);
                    mvaddstr(4, 1, board.as_str());
                    mv(1, 7);
                    refresh();
                } else if key as u8 as char == 'q' || key as u8 as char == 's' {
                    continue;
                } else {
                    let warn_win = newwin(1, 150, 15, 0);
                    waddstr(warn_win, "Field already taken. Press enter to continue.");

                    wrefresh(warn_win);

                    wgetch(warn_win);
                    wgetch(warn_win);
                    wclear(warn_win);
                    wrefresh(warn_win);
                    continue;
                };
            }

            game_result = checker(game_result, &field_vec, turn);

            turn += 1;

            match game_result {
                EndGame::Draw => {
                    mvaddstr(16, 0, "It's a draw.");
                    getch();
                    getch();
                    break;
                }
                EndGame::Lose => {
                    mvaddstr(16, 0, "You lost. :(");
                    getch();
                    getch();
                    break;
                }
                EndGame::Win => {
                    mvaddstr(16, 0, "You won!!!");
                    getch();
                    getch();
                    break;
                }
                EndGame::None => continue,
            }
        }
    }

    endwin();

}

fn print_board(occupy: &Vec<char>) -> String {
    let string = fmt::format(format_args!(
        "\n\n\n    {:^3}|{:^3}|{:^3}     
    {:^3}|{:^3}|{:^3}     
    {:^3}|{:^3}|{:^3}     
    {:^3}|{:^3}|{:^3}     
    {:^3}|{:^3}|{:^3}         
       |   |   \n\n\n",
        occupy[0],
        occupy[1],
        occupy[2],
        "___",
        "___",
        "___",
        occupy[3],
        occupy[4],
        occupy[5],
        "___",
        "___",
        "___",
        occupy[6],
        occupy[7],
        occupy[8]
    ));
    string
}

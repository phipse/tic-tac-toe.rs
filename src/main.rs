use std::io;

struct Field {
    field: String,
}

#[derive(PartialEq)]
enum Player {
    No, // no player
    Px,
    Po,
}

fn player_marker(player: &Player) -> char {
    if player == &Player::Px {
        'X'
    } else {
        'O'
    }
}

impl Field {
    fn new() -> Field {
        Field { field: String::from("---------") }
    }

    fn print(&self) {
        println!("|{}|\t|012|", self.field.get(0..3).unwrap());
        println!("|{}|\t|345|", self.field.get(3..6).unwrap());
        println!("|{}|\t|678|", self.field.get(6..9).unwrap());
    }

    fn val_c(&self, idx: usize) -> char {
        self.field.chars().nth(idx).unwrap()
    }

    fn check_win_p(&self, player: &Player) -> bool {
        let marker = player_marker(&player);
        let win_opts = [(0,1,2), (3,4,5), (6,7,8), // lines
                        (0,3,6), (1,4,7), (2,5,8), // cols
                        (0,4,8), (2,4,6)]; // diags

        for x in win_opts.iter() {
            let mut chars = self.field.chars();
            // XXX nth advances iterator
            // FIX 1: compute the distance to the previous field in win_opts and
            //        substract one to start from zero.
            let a = chars.nth(x.0);
            let b = chars.nth(x.1 - x.0 - 1);
            let c = chars.nth(x.2 - x.1 - 1);
            if a == b && b == c && c == Some(marker) {
                return true;
            }
        }
        return false;
    }

    fn check_win(&self) -> Player {
        if self.check_win_p(&Player::Px) {
            Player::Px
        }
        else if self.check_win_p(&Player::Po) {
            Player::Po
        }
        else {
            Player::No
        }
    }

    fn make_move(&mut self, pl: &Player, idx: usize) -> bool {
        let marker = player_marker(&pl);

        if self.val_c(idx) == '-' {
            self.field.replace_range(idx..idx+1, &marker.to_string());
//            self.print();
            return true;
        }
        else {
            return false;
        }
    }

    fn move_left(&self) -> bool {
        for n in self.field.chars() {
            if n == '-' {
                return true;
            }
        }

        return false;
    }
}

fn read_move() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line( &mut input)
            .ok()
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                if num < 9 { return num; }
                else { println!("Input a value between 0 and 8!"); continue; }
            }
            Err(_) => continue,
        }
    }
}

fn players_turn(pl: &Player) {
    println!("Player's {} turn: ", player_marker(&pl));
}

fn main() {
    println!("Hello, tic-tac-toe!");

    let mut play_field = Field::new();
    let mut current_player = Player::Px;

    loop {
        play_field.print();
        players_turn(&current_player);
        let move_idx = read_move();

        if !play_field.make_move(&current_player, move_idx) {
            println!("invalid move; try again!");
            continue;
        }

        match play_field.check_win() {
            Player::No => {
                if !play_field.move_left() {
                    play_field.print();
                    println!("No player wins!");
                    break;
                }
            }
            pl @ _ => {
                play_field.print();
                println!("Player {} wins!", player_marker(&pl)); 
                break;
            }
        }

        current_player = if current_player == Player::Px { Player::Po } 
                         else { Player::Px };
    }
}

mod state;
use state::{play_state, game_state};
use std::io::{self, Write};

pub struct TicTacToe {
    play_state: play_state::PlayState,
    game_state: game_state::GameState,
    map: Vec<char>
}

impl TicTacToe {

    pub fn new() -> Self {
        let play_state = play_state::PlayState::new();
        let game_state = game_state::GameState::new();
        let map = vec![' '; 9];
        
        Self {
            play_state,
            game_state,
            map,
        }
    }

    fn checker(&mut self) {
        let (first, second) = self.play_state.get_icons();
        let mut winner = |x: &str| {
            println!("{} Win the Game!!!", x);
            self.game_state.stop_playing();
        };
        let map = &self.map;
        let get_xy = |x: usize, y: usize| -> char {
            match map.iter().nth(y*3 + x) {
                Some(c) => c.clone(),
                None => ' '
            }
        };
        

        if get_xy(0, 0) == first && get_xy(0, 1) == first && get_xy(0, 2) == first {winner("Player One");}
        if get_xy(1, 0) == first && get_xy(1, 1) == first && get_xy(1, 2) == first {winner("Player One");}
        if get_xy(2, 0) == first && get_xy(2, 1) == first && get_xy(2, 2) == first {winner("Player One");}

        if get_xy(0, 0) == first && get_xy(1, 0) == first && get_xy(2, 0) == first {winner("Player One");}
        if get_xy(0, 1) == first && get_xy(1, 1) == first && get_xy(2, 1) == first {winner("Player One");}
        if get_xy(0, 2) == first && get_xy(1, 2) == first && get_xy(2, 2) == first {winner("Player One");}

        if get_xy(0, 0) == first && get_xy(1, 1) == first && get_xy(2, 2) == first {winner("Player One");}
        if get_xy(2, 0) == first && get_xy(1, 1) == first && get_xy(0, 2) == first {winner("Player One");}


        if get_xy(0, 0) == second && get_xy(0, 1) == second && get_xy(0, 2) == second {winner("Player Two");}
        if get_xy(1, 0) == second && get_xy(1, 1) == second && get_xy(1, 2) == second {winner("Player Two");}
        if get_xy(2, 0) == second && get_xy(2, 1) == second && get_xy(2, 2) == second {winner("Player Two");}

        if get_xy(0, 0) == second && get_xy(1, 0) == second && get_xy(2, 0) == second {winner("Player Two");}
        if get_xy(0, 1) == second && get_xy(1, 1) == second && get_xy(2, 1) == second {winner("Player Two");}
        if get_xy(0, 2) == second && get_xy(1, 2) == second && get_xy(2, 2) == second {winner("Player Two");}

        if get_xy(0, 0) == second && get_xy(1, 1) == second && get_xy(2, 2) == second {winner("Player Two");}
        if get_xy(2, 0) == second && get_xy(1, 1) == second && get_xy(0, 2) == second {winner("Player Two");}

        if self.map.iter().fold(true,|x, &y| (y != ' ') && x) {self.game_state.stop_playing(); println!("Draw!!!");}
    }

    fn assign_val(&mut self) {
        let (x, y) = self.get_point();
        if self.map[y*3 + x] != ' ' {println!("space allready taken!");self.display_map(); self.assign_val();}
        else {self.map[y*3 + x] = self.play_state.play_round();}
    }

    fn display_map(&self) {
        println!(" _ _ _ ");
        for y in 0..3 {
            print!("|");
            for x in 0..3 {
                print!("{}", self.map[y*3 + x]);
                print!("|");
            }
            print!("\n");
            println!(" _ _ _ ");
        }
    }

    pub fn game_loop(&mut self) {
        while self.game_state.is_playing() {
            self.display_map();
            //println!("already displayed");
            self.assign_val();
            //println!("already assign");
            self.checker();
            //println!("already checked");
            if self.game_state.restart() {
                self.map = vec![' ';9];
                self.play_state = play_state::PlayState::new();
            };
            //println!("try to restart");
        }
    }

    fn get_point(&mut self) -> (usize, usize) {
        let mut x = 10usize;
        let mut y = 10usize;
    
        while y*3 + x > 8 {
            print!("Input cordinate (x y): ");
    
            io::stdout().flush().unwrap();
            let mut buf = String::new();
            let _ = io::stdin().read_line(&mut buf).unwrap();
            let inp = buf.trim().split_ascii_whitespace()
                .filter(|&x| !x.is_empty())
                .map(|x| match x.parse::<usize>() {Ok(i) => i, _ => 10usize})
                .collect::<Vec<usize>>();
            if inp.len() < 2 {println!("Wrong Input!!!");self.display_map(); continue;}
            x = inp[0]; y = inp[1];
            if (y*3 + x) > 8 {println!("Wrong Input!!!");} 
        }
    
        (x, y)
    }

}
pub mod play_state {
    enum PlayerTurn {
        First,
        Second
    }
    pub struct PlayState {
        state: PlayerTurn,
        icons: (char, char)
    }

    impl PlayState {
        pub fn new() -> Self {
            Self { state: PlayerTurn::First, icons: ('x', 'o') }
        }

        pub fn play_round(&mut self) -> char {
            let char = self.get_cicon();
            self.switch_state();
            char
        }

        pub fn get_icons(&mut self) -> (char, char) {
            self.icons
        }

        fn get_cicon(&self) -> char{
            match self.state {
                PlayerTurn::First => self.icons.0,
                PlayerTurn::Second => self.icons.1
            }
        }

        fn switch_state(&mut self) {
            match self.state {
                PlayerTurn::First => self.state = PlayerTurn::Second,
                PlayerTurn::Second => self.state = PlayerTurn::First
            }
        }
    }
}

pub mod game_state {
    use std::io;
    pub struct GameState {
        playing: bool
    }

    impl GameState {
        pub fn new() -> Self {
            Self {playing: true}
        }

        pub fn is_playing(&self) -> bool {
            self.playing
        }

        pub fn stop_playing(&mut self) {
            self.playing = false;
        }

        pub fn restart(&mut self) -> bool {
            if self.is_playing() {
                return false;
            }

            println!("Do you wanna play again? (y/n): ");
            let mut buf = String::new();
            let _ = io::stdin().read_line(&mut buf).unwrap();
            buf = buf.trim().to_string();

            if buf == "y" || buf == "Y" {self.playing = true;return true;}
            if buf == "n" || buf == "N" {return false;}
            else {println!("Wrong Input!"); return self.restart();}
        }
    }
}
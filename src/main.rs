mod game;

fn main() {
    let mut k = game::TicTacToe::new();
    k.game_loop();
}
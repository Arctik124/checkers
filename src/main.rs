use checkers;

fn main() {
    let p1 = "Player 1".to_string();
    let p2 = "Player 2".to_string();

    let game = checkers::Game::new(p1, p2);

    if let Err(error) = checkers::run(game) {
        eprintln!("Error while running game occured: {}", error);
    };
}

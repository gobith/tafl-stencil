use super::brandubh::Brandubh;

#[test]
fn tfl_move_pieve() {
    let mut tafl = Brandubh::new();
    tafl.move_piece(26, 5);
}

#[test]
fn tfl_start_game() {
    let mut tafl = Brandubh::new();
    tafl.start_game(1, 0, 2, 1);
    println!("{}", tafl)
}

use crate::minimax::do_move;

#[cfg(test)]
#[test]
fn tfl_move_pieve() {
    use super::brandubh::Brandubh;
    let mut tafl = Brandubh::new();
    tafl.move_piece(26, 5);
}

#[test]
fn tfl_start_game() {
    use super::brandubh::Brandubh;
    let mut tafl = Brandubh::new();
    tafl.start_game(1, 0, 2, 1);
    println!("{}", tafl)
}

#[test]
fn test_do_move() {
    use super::tafl::brandubh;
    let tafl = brandubh();

    match do_move(tafl.state, 1, 1) {
        Ok(board) => {
            println!("{:?}", board)
        }
        Err(()) => {
            println!("test")
        }
    }
}

#[test]
fn test_range() {
    for piece_index in 0..49 {
        println!("{}" , piece_index)
    }
}

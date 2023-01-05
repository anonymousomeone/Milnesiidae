pub mod eval;
pub mod search;

use std::io;
use cozy_chess::BitBoard;
use crate::search::searcher::Engine;
fn main() {
    println!("Hello, world!");

    // println!("{}", evaluate(board.clone(), false));
    // println!("{:#?}", board);
    // let (mv, score) = search(board, 1, i32::MIN, i32::MAX, 0);
    // println!("{}", mv.unwrap());
    let mut engine = Engine::new();
    engine.depth = 4;
    let mv = engine.go();
    println!("{}", mv);

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        input.pop();

        if input == "board" {
          println!("{:#?}", engine.board.occupied());
        } else {
        let input = input.parse();
      let mv = match input {
        Ok(mv) => mv,
        Err(_) => { println!("skill issue\n try again."); continue }
      };
      
        engine.board.play(mv);

        let mv = engine.go();

        println!("{}", mv);
    }
    }
}

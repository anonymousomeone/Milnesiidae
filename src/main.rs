pub mod eval;
pub mod search;

use std::io::{self, Write};
use cozy_chess::BitBoard;
use crate::search::searcher::Engine;
fn main() {
    println!("Hello, world!");

    // println!("{}", evaluate(board.clone(), false));
    // println!("{:#?}", board);
    // let (mv, score) = search(board, 1, i32::MIN, i32::MAX, 0);
    // println!("{}", mv.unwrap());
    let mut engine = Engine::new();
    engine.depth = 5;
    let (mv, eval) = engine.go();
    println!("Engine move: {}, eval: {}", mv, eval);

    loop {
        print!("Your move: ");
        io::stdout().flush();
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
        Err(_) => { println!("skill issue\ntry again."); continue }
      };

      if engine.board.try_play(mv).unwrap() {
  
        let (mv, eval) = engine.go();
  
        println!("Engine move: {}, eval: {}", mv, eval);
      } else {
        println!("invalid move (skill issue)\ntry again.");
        continue;
      }
      
    }
    }
}

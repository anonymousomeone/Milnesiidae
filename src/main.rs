pub mod eval;
pub mod search;
pub mod uci;
pub mod engine;

use std::io::{self, Write};
use cozy_chess::Board;
use crate::eval::evaluator;
use crate::engine::engine::Engine;
use crate::uci::uci::Uci;
fn main() {
  // let mut uci = Uci::new();
  // println!("{:#?}", uci.handle_cmd("position startpos moves e2e4 e7e5".to_string()));
  //   println!("Hello, world!");

    // println!("{}", evaluate(board.clone(), false));
    // println!("{:#?}", board);
    // let (mv, score) = search(board, 1, i32::MIN, i32::MAX, 0);
    // println!("{}", mv.unwrap());
    // let (mv, eval) = engine.go();
    // println!("Engine move: {}, eval: {}", mv, eval);

    let mut uci = Uci::new();

    loop {
    //     print!("Your move: ");
    //     io::stdout().flush();
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        // input.pop();
        // input.pop();

        uci.handle_cmd(input.trim().to_string());

    //     if input == "board" {
    //       println!("{:#?}", engine.board.occupied());
    //     } else {
    //     let input = input.parse();
    //   let mv = match input {
    //     Ok(mv) => mv,
    //     Err(_) => { println!("skill issue\ntry again."); continue }
    //   };

    //   if engine.board.try_play(mv).unwrap() {
  
    //     let (mv, eval) = engine.go();
  
    //     println!("Engine move: {}, eval: {}", mv, eval);
    //   } else {
    //     println!("invalid move (skill issue)\ntry again.");
    //     continue;
    //   }
      
    // }
    }
}

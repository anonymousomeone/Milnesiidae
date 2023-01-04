pub mod eval;
pub mod search;

use std::io;

use crate::search::searcher::Engine;
fn main() {
    println!("Hello, world!");

    // println!("{}", evaluate(board.clone(), false));
    // println!("{:#?}", board);
    // let (mv, score) = search(board, 1, i32::MIN, i32::MAX, 0);
    // println!("{}", mv.unwrap());
    let mut engine = Engine::new();
    engine.depth = 5;
    let mv = engine.go();
    println!("{}", mv);

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input);
        input.pop();
        input.pop();

        engine.board.play(input.parse().unwrap());

        let mv = engine.go();

        println!("{}", mv);
    }
    
}



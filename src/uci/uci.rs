use crate::Engine;
use cozy_chess::*;
// http://wbec-ridderkerk.nl/html/UCIProtocol.html

use std::{process, num::ParseIntError};

pub struct Uci {
  engine: Engine,
  ready: bool,
}

impl Uci {
  pub fn new() -> Uci {
    let engine = Engine::new();
    Uci {
      engine,
      ready: false
    }
  }

  fn startpos(&mut self, mut msg: Vec<&str>) {
    if msg.len() == 2 {
      self.engine.board = Board::default();
    } else {
      self.engine.board = Board::default();
      Uci::parsemoves(self, msg.split_off(3));
    }
  }

  pub fn handle_cmd(&mut self, msg: String) {
    let mut msg = msg.split(" ").collect::<Vec<&str>>();

    let command = msg[0];
    let args = msg[1..].to_vec();

    match command {
      "isready" => println!("readyok"),
      "ucinewgame" => self.engine = Engine::new(),
      
      "uci" => {
        println!("id name Milnesiidae");
        println!("id author Zalander");

        println!("uciok");
      },

      "position" => {
        if msg[1] == "startpos" {
          self.startpos(msg.clone());
        } else if msg[1] == "fen" {
          let fen = &msg[2..8].join(" ");

          self.engine.board = Board::from_fen(fen, false).unwrap();
          
          if msg.len() > 8 {
            Uci::parsemoves(self, msg.split_off(9));
          }
        }
        
      },

      "go" => {
        if args.len() > 0 {
          for i in 0..args.len() {
            let arg = args[i];

            match arg {
              "wtime" => {
                let wtime: u32 = args[i + 1].parse().expect("bad wtime");
                
                self.engine.wtime = wtime;
                continue;
              },
              "btime" => {
                let btime: u32 = args[i + 1].parse().expect("bad btime");

                self.engine.btime = btime;
                continue;
              },
              "movestogo" => {
                let togo: u16 = args[i + 1].parse().expect("bad movestogo");

                self.engine.movestogo = togo;
                continue;
              }

              _ => {}
            }
          }
        }

        // println!("wtime: {}", self.engine.wtime);
        // println!("btime: {}", self.engine.btime);
        // println!("movestogo: {}", self.engine.movestogo);


        let mv = self.engine.go();

        println!("bestmove {}", mv);
      }

      "quit" => {
        process::exit(0);
      }

      // debug only !!
      "board" => {
        println!("{:#?}", self.engine.board.occupied());
      }

      "piece" => {
        let square: Result<Square, SquareParseError> = msg[1].parse();

        match square {
          Ok(square) => { 
            println!("{:#?}", self.engine.board.piece_on(square));
           }

          Err(_) => println!("skill issue (not a square)")
        }
      }

      _ => {}
    }
  }
  
  // castling parsing
  pub fn parsemoves(&mut self, msg: Vec<&str>) {

    for str in msg {
      let mv: Move = str.parse().unwrap();
      
      let mv = Uci::castle(&self.engine.board, mv);
      
      self.engine.board.play(mv);
    }
  }

  fn castle(board: &Board, mut mv: Move) -> Move {
    // white castling
    if mv.from == Square::E1 && 
    (mv.to == Square::C1 || mv.to == Square::G1) && 
     board.piece_on(mv.from).unwrap().eq(&Piece::King)
    {
      if mv.to == Square::C1 {
        mv.to = Square::A1;
      } else if mv.to == Square::G1 {
        mv.to = Square::H1;
      }
    } // black castling
    else if mv.from == Square::E8 && 
          (mv.to == Square::C8 || mv.to == Square::G8) && 
          board.piece_on(mv.from).unwrap() == Piece::King

    { 
      if mv.to == Square::C8 {
        mv.to = Square::A8;
      } else if mv.to == Square::G8 {
        mv.to = Square::H8;
      }
    }

    return mv;
  }
}

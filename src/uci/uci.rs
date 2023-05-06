use crate::Engine;
use cozy_chess::*;
// http://wbec-ridderkerk.nl/html/UCIProtocol.html

use std::process;

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
      let moves = Uci::parsemoves(self, msg.split_off(3));

      for mv in moves {
        self.engine.board.play(mv);
      }
    }
  }

  pub fn handle_cmd(&mut self, msg: String) {
    let msg = msg.split(" ").collect::<Vec<&str>>();

    let command = msg[0];

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
          self.startpos(msg);
        } else {
          self.engine.board = Board::from_fen(msg[2], false).unwrap();
        }
      },

      "go" => {
        let side = self.engine.board.side_to_move();
        let (mv, eval, nodes, nps) = self.engine.go();

        let cp;
        match side {
          Color::Black => { cp = -eval }
          Color::White => { cp = eval }
        }

        println!("info score cp {} nodes {} nps {}", cp, nodes, nps);
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
  pub fn parsemoves(&self, msg: Vec<&str>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let board = &self.engine.board;

    for str in msg {
      let mv: Move = str.parse().unwrap();
      
      let mv = Uci::castle(board, mv);
      
      moves.push(mv);
    }

    moves
  }

  fn castle(board: &Board, mut mv: Move) -> Move {
    // white castling
    if mv.from == Square::E1 && 
    (mv.to == Square::C1 || mv.to == Square::G1) && 
     board.piece_on(mv.from).unwrap().eq(&Piece::King)
    {
      let h1 = board.piece_on(Square::H1);
      let a1 = board.piece_on(Square::A1);

      match h1 {
        Some(piece) => {
          if piece != Piece::Rook {
            return mv;
          }
        },
        None => return mv
      }

      match a1 {
        Some(piece) => {
          if piece != Piece::Rook {
            return mv;
          }
        },
        None => return mv
      }
      if mv.to == Square::C1 {
        mv.to = Square::A1;
      } else {
        mv.to = Square::H1;
      }
    } // black castling
    else if mv.from == Square::E8 && 
          (mv.to == Square::C8 || mv.to == Square::G8) && 
          board.piece_on(mv.from).unwrap() == Piece::King

    {
      let h8 = board.piece_on(Square::H8);
      let a8 = board.piece_on(Square::A8);

      match h8 {
        Some(piece) => {
          if piece != Piece::Rook {
            return mv;
          }
        },
        None => return mv
      }

      match a8 {
        Some(piece) => {
          if piece != Piece::Rook {
            return mv;
          }
        },
        None => return mv
      }

      if mv.to == Square::C8 {
        mv.to = Square::A8;
      } else {
        mv.to = Square::H8;
      }
    }

    return mv;
  }
}

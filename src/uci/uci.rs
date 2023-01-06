use crate::search::searcher::Engine;
use cozy_chess::*;
// http://wbec-ridderkerk.nl/html/UCIProtocol.html

pub struct Uci {
  engine: Engine,
  ready: bool,
  startpos: String
}

impl Uci {
  pub fn new() -> Uci {
    let mut engine = Engine::new();
    engine.depth = 5;
    Uci {
      engine,
      ready: false,
      startpos: "".to_string()
    }
  }

  pub fn handle_cmd(&mut self, msg: String) {
    let mut msg = msg.split(" ").collect::<Vec<&str>>();
    
    if msg[0] == "uci" {
      Uci::startuci();
    } else if msg[0] == "isready" {
      println!("readyok");
      self.ready = true;
    } else if msg[0] == "ucinewgame" {
      self.engine = Engine::new();
    }
    
    if msg[0] == "position" {

      if msg[1] == "startpos" {
        if msg.len() == 2 {
          self.engine.board = Board::default();
        } else {
          self.engine.board = Board::default();
          let moves = Uci::parsemoves(self, msg.split_off(3));

          for mv in moves {
            self.engine.board.play(mv);
          }
        }
      } else {
        self.engine.board = Board::from_fen(msg[2], false).unwrap();
      }
    }

    if msg[0] == "go" {
      self.engine.depth = 5;
      let (mv, eval, nodes, nps) = self.engine.go();

      println!("info score {} nodes {} nps {}", eval, nodes, nps);
      println!("bestmove {}", mv);
    }
  }
  
  fn startuci() {
    println!("id name Milnesiidae");
    println!("id author Zalander");

    println!("uciok");
  }

  pub fn parsemoves(&self, msg: Vec<&str>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for str in msg {
      let mut mv: Move = str.parse().unwrap();
      if mv.from == Square::E1 && (mv.to == Square::C1 || mv.to == Square::G1) && self.engine.board.piece_on(mv.from).unwrap() == Piece::King {
        if mv.to == Square::C1 {
          mv.to = Square::A1;
        } else {
          mv.to = Square::H1;
        }
      } else if mv.from == Square::E8 && (mv.to == Square::C8 || mv.to == Square::G8) && self.engine.board.piece_on(mv.from).unwrap() == Piece::King {
        if mv.to == Square::C8 {
          mv.to = Square::A8;
        } else {
          mv.to = Square::H8;
        }
      }
      
      moves.push(mv);
    }

    moves
  }
}

use cozy_chess::*;

use crate::eval::evaluator;
use crate::search::constants::*;

pub struct Searcher {
    pub board: Board,
    pub past_pos: Vec<u64>,
    pub depth: i32,
    pub nodes: u64,
    pub endgame: bool,

    pub killer_moves: [[Move; MAX_DEPTH as usize]; MAX_KILLER_MOVES],
}

impl Searcher {
pub fn new() -> Searcher {
    Searcher {
        board: Board::default(),
        past_pos: Vec::with_capacity(64),
        depth: 0,
        nodes: 0,

        killer_moves: [[Searcher::nmove(); MAX_DEPTH as usize]; MAX_KILLER_MOVES],
        endgame: false
    }
}

pub fn search(&mut self, board: &Board, depth: i32, mut ply: i32, mut alpha: i32, beta: i32) -> (Option<Move>, i32) {
    self.nodes += 1;
    ply += 1;
    
    match board.status() {
        GameStatus::Drawn => return (None, 0),
        GameStatus::Won => return (None, -30000 + self.depth - depth),
        GameStatus::Ongoing => {}
    }

    if self.past_pos.len() > 6 {
        let hash = board.hash();
        if hash == self.past_pos[self.past_pos.len() - 4] {
            return (None, 0)
        }
    }
    
    let moves: Vec<Move> = evaluator::sorted_move_gen(&board, self);

    if depth == 0 {
        return self.qsearch(board, alpha, beta, self.depth);
    }
    
    let mut best_move: Option<Move> = None;
    let mut eval = i32::MIN;
    
    for mv in moves {
        let mut nboard = board.clone();
        nboard.play_unchecked(mv);
        let (_, score) = self.search(&nboard, depth - 1, ply, -beta, -alpha);
        let score = -score;
        if score > eval {
            eval = score;
            best_move = Some(mv);
            if eval > alpha {
                alpha = eval;
                if alpha >= beta {
                    self.store_killer(mv, ply);
                    break;
                }
            }
        }

    }
    (best_move, eval)
}
pub fn qsearch(&mut self, board: &Board, mut alpha: i32, beta: i32, mut ply: i32) -> (Option<Move>, i32) {
    self.nodes += 1;
    ply += 1;

    match board.status() {
        GameStatus::Drawn => return (None, 0),
        GameStatus::Won => return (None, -30000 + ply),
        GameStatus::Ongoing => {}
    }

    if self.past_pos.len() > 6 {
        let hash = board.hash();
        if hash == self.past_pos[self.past_pos.len() - 4] {
            return (None, 0)
        }
    }
    
    let stand_pat = evaluator::evaluate(&board, self.endgame);
    
    if stand_pat >= beta {
        return (None, beta);
    }
    
    if alpha < stand_pat {
        alpha = stand_pat;
    }
    
    let moves: Vec<Move> = evaluator::loud_move_gen(&board);

    if moves.len() == 0 {
        return (None, stand_pat)
    }
    
    let mut best_move: Option<Move> = None;
    let mut eval = i32::MIN;
    
    for mv in moves {
        let mut nboard = board.clone();
        nboard.play_unchecked(mv);
        
        let (_, score) = self.qsearch(&nboard, -beta, -alpha, ply);
        let score = -score;
        if score > eval {
            eval = score;
            best_move = Some(mv);
            if eval > alpha {
                alpha = eval;
                if alpha >= beta {
                    break;
                }
            }
        }

    }
    (best_move, eval)
}

fn store_killer(&mut self, mv: Move, ply: i32) {
    let ply = ply as usize;
    
    if !(self.killer_moves[0][ply].to == mv.to && self.killer_moves[0][ply].from == mv.from) {
      for i in (1..MAX_KILLER_MOVES).rev() {
        let n = i as usize;
        let previous = self.killer_moves[n - 1][ply];
        self.killer_moves[n][ply] = previous;
      }
    }
  
    self.killer_moves[0][ply] = mv;
  }
  fn nmove() -> Move {
    Move {
      from: Square::A1,
      to: Square::A1,
      promotion: None
    }
  }
}
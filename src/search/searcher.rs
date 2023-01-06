use cozy_chess::*;

use crate::eval::evaluator;

pub struct Engine {
    pub board: Board,
    pub depth: i32,
    pub past_pos: Vec<u64>,
    nodes: u64,
    endgame: bool
}

impl Engine {
pub fn new() -> Engine {
    Engine {
        board: Board::default(),
        depth: 0,
        past_pos: Vec::with_capacity(64),
        nodes: 0,
        endgame: false
    }
}

pub fn go(&mut self) -> (Move, i32) {
    let board = &self.board.clone();

    let (mv, eval) = self.search(board, self.depth, -i32::MAX, i32::MAX);

    self.board.play_unchecked(mv.unwrap());
    self.past_pos.push(self.board.hash());

    let knight_phase = 1;
    let bishop_phase = 1;
    let rook_phase = 2;
    let queen_phase = 4;

    let mut phase = 24;

    let knights = board.pieces(Piece::Knight);
    let bishops = board.pieces(Piece::Bishop);
    let rooks = board.pieces(Piece::Rook);
    let queens = board.pieces(Piece::Queen);


    phase -= knights.len() * knight_phase;
    phase -= bishops.len() * bishop_phase;
    phase -= rooks.len() * rook_phase;
    phase -= queens.len() * queen_phase;

    phase = (phase * 256 + (24 / 2)) / 24;

    if phase > 145 {
        self.endgame = true;
    }

    (mv.unwrap(), eval)
}

fn search(&mut self, board: &Board, depth: i32, mut alpha: i32, beta: i32) -> (Option<Move>, i32) {
    self.nodes += 1;
    
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
    
    let moves: Vec<Move> = evaluator::sorted_move_gen(&board);

    if depth == 0 {
        return self.qsearch(board, alpha, beta, self.depth);
    }
    
    let mut best_move: Option<Move> = None;
    let mut eval = i32::MIN;
    
    for mv in moves {
        let mut nboard = board.clone();
        nboard.play_unchecked(mv);
        let (_, score) = self.search(&nboard, depth - 1, -beta, -alpha);
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
fn qsearch(&mut self, board: &Board, mut alpha: i32, beta: i32, mut ply: i32) -> (Option<Move>, i32) {
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
}
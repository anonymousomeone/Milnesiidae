
use cozy_chess::*;

use crate::search::searcher::Searcher;
use std::time::Instant;
use crate::search;
pub struct Engine {
    pub board: Board,
    pub depth: i32,
    searcher: Searcher,
    endgame: bool
}

impl Engine {
    pub fn new() -> Engine {
        let depth: i32 = search::constants::MAX_DEPTH.into();

        Engine {
            board: Board::default(),
            depth,
            searcher: Searcher::new(),
            endgame: false
        }
    }
    pub fn go(&mut self) -> (Move, i32, u64, u64) {
        self.searcher.nodes = 0;
        let instant = Instant::now();
    
        let board = &self.board.clone();
    
        let (mv, eval) = self.searcher.search(board, self.depth, 0, -i32::MAX, i32::MAX);
    
        let elapsed = instant.elapsed().as_secs();
    
        let nps;
        if elapsed == 0 {
            nps = self.searcher.nodes
        } else {
            nps = self.searcher.nodes / elapsed as u64;
        }

        match mv {
            None => { panic!("No valid move !!!") }
            _ => {}
        }
    
        self.board.play_unchecked(mv.unwrap());
        self.searcher.past_pos.push(self.board.hash());
    
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
    
        (mv.unwrap(), eval, self.searcher.nodes, nps)
    }
    
}

use cozy_chess::*;

use crate::search::searcher::Searcher;
use crate::search::{self, constants};
use crate::search::timeman::Timeman;

use std::process;
use std::thread::Thread;
use std::time::Duration;
use std::{time::Instant, thread};
use std::sync::atomic::{Ordering};

pub struct Engine {
    pub board: Board,
    pub depth: i32,
    pub wtime: u32,
    pub btime: u32,
    pub movestogo: u16,

    searcher: Searcher,
    endgame: bool,

    max_depth: u16,

}

impl Engine {
    pub fn new() -> Engine {
        let depth: i32 = search::constants::MAX_DEPTH.into();

        Engine {
            board: Board::default(),
            depth,
            searcher: Searcher::new(),
            endgame: false,

            wtime: 10000,
            btime: 10000,

            movestogo: 10,

            max_depth: constants::MAX_DEPTH,
        }
    }
    pub fn go(&mut self) -> Move {
        self.searcher.nodes = 0;
        self.searcher.out_of_time.store(false, Ordering::Relaxed);

        let instant = Instant::now();
    
        let board = &self.board.clone();


        // i hate multithreading
        let out_of_time = self.searcher.out_of_time.clone();
        let time_left;

        match self.board.side_to_move() {
            Color::White => {
                time_left = self.wtime;
            },
            Color::Black => {
                time_left = self.btime;
            }
        }
        let time_to_search = Timeman::get_time(time_left, self.movestogo);

        thread::spawn(move || {
            let duration = Duration::from_millis(time_to_search.into());
            let stop = Instant::now() + duration;

            loop {
                let instant = Instant::now();

                if instant.le(&stop) {
                    let dur = Duration::from_millis(10);
                    thread::sleep(dur);
                    continue;
                }

                // println!("out of time!!");
                out_of_time.swap(true, Ordering::Relaxed);

                break;
            }

        });

        // iterative deepener
        let mut best_mv = None;

        for depth in 0..self.max_depth {
            let result = self.searcher.search(board, depth.into(), 0, -i32::MAX, i32::MAX);

            let eval;
    
            match result {
                Some((m, e)) => {
                    best_mv = m;
                    eval = e;
                },
                None => {
                    break;
                }
            }

            let elapsed = instant.elapsed().as_secs();
        
            let nps;
            if elapsed == 0 {
                nps = self.searcher.nodes
            } else {
                nps = self.searcher.nodes / elapsed as u64;
            }

            let side = self.board.side_to_move();
            
            let cp;
            match side {
            Color::Black => { cp = -eval }
            Color::White => { cp = eval }
            }

            println!("info depth {} nodes {} nps {} score cp {}", depth, self.searcher.nodes, nps, cp);
        }

        self.board.play_unchecked(best_mv.unwrap());
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
    
        best_mv.unwrap()
    }
    
}
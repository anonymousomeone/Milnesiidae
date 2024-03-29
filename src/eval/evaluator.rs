use crate::{eval::constants::*, search::{searcher::Searcher, constants::*}};
use cozy_chess::*;

#[derive(Debug)]
#[derive(Clone)]
struct ScoredMove {
    mv: Move,
    sc: u32,
    capture: Option<Piece>,
    piece: Piece,
}

impl ScoredMove {
    fn new(board: &Board, mv: Move) -> ScoredMove {
        let capture = board.piece_on(mv.to);
        let piece = board.piece_on(mv.from).unwrap();
        ScoredMove {
            mv,
            sc: 0,
            capture,
            piece,
        }
    }

    fn get_mv(&self) -> Move {
        self.mv
    }

    fn get_sc(&self) -> u32 {
        self.sc
    }
    fn set_sc(&mut self, score: u32) {
        self.sc = score;
    }
}

// get capturing moves or smth? idk
pub fn loud_move_gen(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(64);
    let color = board.side_to_move();
    let enemy_pcs = board.colors(!color);

    let mut captures: Vec<ScoredMove> = Vec::with_capacity(32);
    board.generate_moves(|mut capture_moves| {
        capture_moves.to &= enemy_pcs;
        for mv in capture_moves {
            captures.push(ScoredMove::new(board, mv));
        }
        false
    });

    captures = score_mvv_lva(captures);
    captures.sort_by(|a, b| b.sc.cmp(&a.sc));

    for capture in captures {
        moves.push(capture.mv);
    }

    moves
}

pub fn sorted_move_gen(board: &Board, sinfo: &mut Searcher) -> Vec<Move> {
    let mut res: Vec<Move> = Vec::with_capacity(64);
    let color = board.side_to_move();
    let enemy_pcs = board.colors(!color);

    let mut moves: Vec<ScoredMove> = Vec::with_capacity(64);
    board.generate_moves(|mut capture_moves| {
        capture_moves.to &= enemy_pcs;
        for mv in capture_moves {
            moves.push(ScoredMove::new(board, mv));
        }
        false
    });

    moves = score_mvv_lva(moves);
    moves.sort_by(|a, b| b.sc.cmp(&a.sc));
    for mv in moves {
        res.push(mv.mv);
    }

    moves = Vec::with_capacity(64);
    board.generate_moves(|mut quiet_moves| {
        quiet_moves.to &= !enemy_pcs;
        for mv in quiet_moves {
            moves.push(ScoredMove::new(board, mv));
        }
        false
    });
    let killers = score_killer(&moves, sinfo);
    // killers.sort_by(|a, b| b.sc.cmp(&a.sc));
    for mv in killers {
        res.push(mv.mv);
    }
    for mv in moves {
        res.push(mv.mv);
    }

    // board.generate_moves(|mut check_moves| {
    //     check_moves.to &= board.colors(!color) & board.pieces(Piece::King);
    //     moves.extend(check_moves);
    //     false
    // });

    res
}

fn score_killer(ml: &Vec<ScoredMove>, sinfo: &mut Searcher) -> Vec<ScoredMove> {
    let mut res: Vec<ScoredMove> = Vec::with_capacity(64);

    for mv in ml {
        match mv.capture {
            Some(_) => {},
            None => {
                let ply = sinfo.depth as usize;
                let mut i = 0;
                while i < MAX_KILLER_MOVES && mv.get_sc() == 0 {
                    let mut mv = mv.clone();
                    let killer = sinfo.killer_moves[i][ply];
                    if mv.get_mv() == killer {
                        mv.set_sc(MVV_LVA_OFFSET - ((i as u32 + 1) * KILLER_VALUE));
                        res.push(mv)
                    }
                    i += 1;
                }
            }
        }
    }

    res
}

fn score_mvv_lva(ml: Vec<ScoredMove>) -> Vec<ScoredMove> {
    let mut res: Vec<ScoredMove> = Vec::with_capacity(64);

    for mut mv in ml {
        match mv.capture {
            Some(capture) => {
                let val = MVV_LVA_OFFSET + MVV_LVA[piece_index(capture)][piece_index(mv.piece)] as u32;
                mv.sc = val as u32;
                res.push(mv);
            },
            None => {}
        }
    }

    res
}

fn piece_index(piece: Piece) -> usize {
    return match piece {
        Piece::King => 0,
        Piece::Queen => 1,
        Piece::Rook => 2,
        Piece::Bishop => 3,
        Piece::Knight => 4,
        Piece::Pawn => 5,
    };
}

pub fn evaluate(board: &Board, endgame: bool) -> i32 {
    let color: Color = board.side_to_move();
    let our_pieces = board.colors(color);
    let their_pieces = board.colors(!color);

    let eval: i32;

    let mut our_eval: i32 = 0;
    let mut their_eval: i32 = 0;

    for &piece in &Piece::ALL {
        let my_pieces = our_pieces & board.pieces(piece);
        let enemy_pieces = their_pieces & board.pieces(piece);

        for square in my_pieces {
            let mut sum: i32 = 0;
            match piece {
                Piece::Pawn => {
                    if endgame {
                        sum += PAWN_E
                    } else {
                        sum += PAWN
                    }

                    if color == Color::White {
                        sum += P[square as usize];
                    } else {
                        sum += BP[square as usize];
                    }
                }
                Piece::Knight => {
                    if endgame {
                        sum += KNIGHT_E
                    } else {
                        sum += KNIGHT
                    }

                    if color == Color::White {
                        sum += K[square as usize];
                    } else {
                        sum += BK[square as usize];
                    }
                }
                Piece::Bishop => {
                    if endgame {
                        sum += BISHOP_E
                    } else {
                        sum += BISHOP
                    }

                    if color == Color::White {
                        sum += B[square as usize];
                    } else {
                        sum += BB[square as usize];
                    }
                }
                Piece::Rook => {
                    if endgame {
                        sum += ROOK_E
                    } else {
                        sum += ROOK
                    }

                    if color == Color::White {
                        sum += R[square as usize];
                    } else {
                        sum += BR[square as usize];
                    }
                }
                Piece::Queen => {
                    if endgame {
                        sum += QUEEN_E
                    } else {
                        sum += QUEEN
                    }

                    if color == Color::White {
                        sum += Q[square as usize];
                    } else {
                        sum += BQ[square as usize];
                    }
                }
                Piece::King => {
                    if endgame {
                        if color == Color::White {
                            sum += KE[square as usize];
                        } else {
                            sum += BKE[square as usize];
                        }
                    } else {
                        if color == Color::White {
                            sum += KG[square as usize];
                        } else {
                            sum += BKG[square as usize];
                        }
                    }
                }
            };
            our_eval += sum;
        }
        for square in enemy_pieces {
            let mut sum: i32 = 0;
            match piece {
                Piece::Pawn => {
                    if endgame {
                        sum += PAWN_E
                    } else {
                        sum += PAWN
                    }

                    if color == Color::White {
                        sum += BP[square as usize];
                    } else {
                        sum += P[square as usize];
                    }
                }
                Piece::Knight => {
                    if endgame {
                        sum += KNIGHT_E
                    } else {
                        sum += KNIGHT
                    }

                    if color == Color::White {
                        sum += BK[square as usize];
                    } else {
                        sum += K[square as usize];
                    }
                }
                Piece::Bishop => {
                    if endgame {
                        sum += BISHOP_E
                    } else {
                        sum += BISHOP
                    }

                    if color == Color::White {
                        sum += BB[square as usize];
                    } else {
                        sum += B[square as usize];
                    }
                }
                Piece::Rook => {
                    if endgame {
                        sum += ROOK_E
                    } else {
                        sum += ROOK
                    }

                    if color == Color::White {
                        sum += BR[square as usize];
                    } else {
                        sum += R[square as usize];
                    }
                }
                Piece::Queen => {
                    if endgame {
                        sum += QUEEN_E
                    } else {
                        sum += QUEEN
                    }

                    if color == Color::White {
                        sum += BQ[square as usize];
                    } else {
                        sum += Q[square as usize];
                    }
                }
                Piece::King => {
                    if endgame {
                        if color == Color::White {
                            sum += BKE[square as usize];
                        } else {
                            sum += KE[square as usize];
                        }
                    } else {
                        if color == Color::White {
                            sum += BKG[square as usize];
                        } else {
                            sum += KG[square as usize];
                        }
                    }
                }
            };
            their_eval += sum;
        }
    }
    eval = our_eval - their_eval;

    eval
}

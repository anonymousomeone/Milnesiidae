use cozy_chess::*;
use crate::eval::eval_info::*;

pub fn evaluate(board: &Board, endgame: bool) -> i32 {
    let color: Color = board.side_to_move();
    let our_pieces = board.colors(color);
    let enemy_pieces = board.colors(!color);

    let eval: i32;

    let mut our_eval: i32 = 0;
    let mut their_eval: i32 = 0;

    for &piece in &Piece::ALL {
        for square in our_pieces {
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
                    sum += KING;

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

// get capturing moves or smth? idk
pub fn loud_move_gen(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(64);
    let color = board.side_to_move();
    let enemy_pcs = board.colors(!color);

    board.generate_moves(|mut capture_moves| {
        capture_moves.to &= enemy_pcs;
        moves.extend(capture_moves);
        false
    });

    moves
}
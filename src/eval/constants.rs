// piece values
pub const PAWN: i32 = 126;
pub const KNIGHT: i32 = 781;
pub const BISHOP: i32 = 825;
pub const ROOK: i32 = 1276;
pub const QUEEN: i32 = 2538;
// endgame variants
pub const PAWN_E: i32 = 208;
pub const KNIGHT_E: i32 = 854;
pub const BISHOP_E: i32 = 915;
pub const ROOK_E: i32 = 1380;
pub const QUEEN_E: i32 = 2682;

// killer move stuff
pub const MVV_LVA_OFFSET: u32 = u32::MAX - 256;
pub const KILLER_VALUE: u32 = 10;

// MVV_VLA[victim][attacker]
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];


// psts

// white men
// pawn
pub const P: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    -31,   8,  -7, -37, -36, -14,   3, -31,
    -22,   9,   5, -11, -10,  -2,   3, -19,
    -26,   3,  10,   9,   6,   1,   0, -23,
    -17,  16,  -2,  15,  14,   0,  15, -13,
      7,  29,  21,  44,  40,  31,  44,   7,
     78,  83,  86,  73, 102,  82,  85,  90,
    100, 100, 100, 100, 105, 100, 100,  100
];
// knight
pub const K: [i32; 64] = [
    -74, -23, -26, -24, -19, -35, -22, -69,
    -23, -15,   2,   0,   2,   0, -23, -20,
    -18,  10,  13,  22,  18,  15,  11, -14,
     -1,   5,  31,  21,  22,  35,   2,   0,
     24,  24,  45,  37,  33,  41,  25,  17,
     10,  67,   1,  74,  73,  27,  62,  -2,
     -3,  -6, 100, -36,   4,  62,  -4, -14,
    -66, -53, -75, -75, -10, -55, -58, -70,  
];
// bishop
pub const B: [i32; 64] = [
    -7,   2, -15, -12, -14, -15, -10, -10,
    19,  20,  11,   6,   7,   6,  20,  16,
    14,  25,  24,  15,   8,  25,  20,  15,
    13,  10,  17,  23,  17,  16,   0,   7,
    25,  17,  20,  34,  26,  25,  15,  10,
    -9,  39, -32,  41,  52, -10,  28, -14,
   -11,  20,  35, -42, -39,  31,   2, -22,
   -59, -78, -82, -76, -23,-107, -37, -50,
];
// rook
pub const R: [i32; 64] = [
    -30, -24, -18,   5,  -2, -18, -31, -32,
    -53, -38, -31, -26, -29, -43, -44, -53,
    -42, -28, -42, -25, -25, -35, -26, -46,
    -28, -35, -16, -21, -13, -29, -46, -30,
      0,   5,  16,  13,  18,  -4,  -9,  -6,
     19,  35,  28,  33,  45,  27,  25,  15,
     55,  29,  56,  67,  55,  62,  34,  60,
     35,  29,  33,   4,  37,  33,  56,  50
];
// queen
pub const Q: [i32; 64] = [
    -39, -30, -31, -13, -31, -36, -34, -42,
    -36, -18,   0, -19, -15, -15, -21, -38,
    -30,  -6, -13, -11, -16, -11, -16, -27,
    -14, -15,  -2,  -5,  -1, -10, -20, -22,
      1, -16,  22,  17,  25,  20, -13,  -6,
     -2,  43,  32,  60,  72,  63,  43,   2,
     14,  32,  60, -10,  20,  76,  57,  24,
      6,   1,  -8,-104,  69,  24,  88,  26
];
// king
pub const KG: [i32; 64] = [
    17,  30,  -3, -14,   6,  -1,  40,  18,
    -4,   3, -14, -50, -57, -18,  13,   4,
   -47, -42, -43, -79, -64, -32, -29, -32,
   -55, -43, -52, -28, -51, -47,  -8, -50,
   -55,  50,  11,  -4, -19,  13,   0, -49,
   -62,  12, -57,  44, -67,  28,  37, -31,
   -32,  10,  55,  56,  56,  55,  10,   3,
     4,  54,  47, -99, -99,  60,  83, -62
];
pub const KE: [i32; 64] = [
    -50, -30, -30, -30, -30, -30, -30, -50,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -50, -40, -30, -20, -20, -30, -40, -50
];

// black men
// pawn
pub const BP: [i32; 64] = [
    100, 100, 100, 100, 105, 100, 100,  100,
    78,  83,  86,  73, 102,  82,  85,  90,
     7,  29,  21,  44,  40,  31,  44,   7,
   -17,  16,  -2,  15,  14,   0,  15, -13,
   -26,   3,  10,   9,   6,   1,   0, -23,
   -22,   9,   5, -11, -10,  -2,   3, -19,
   -31,   8,  -7, -37, -36, -14,   3, -31,
     0,   0,   0,   0,   0,   0,   0,   0
];
// knight
pub const BK: [i32; 64] = [
    -66, -53, -75, -75, -10, -55, -58, -70,
    -3,  -6, 100, -36,   4,  62,  -4, -14,
    10,  67,   1,  74,  73,  27,  62,  -2,
    24,  24,  45,  37,  33,  41,  25,  17,
    -1,   5,  31,  21,  22,  35,   2,   0,
   -18,  10,  13,  22,  18,  15,  11, -14,
   -23, -15,   2,   0,   2,   0, -23, -20,
   -74, -23, -26, -24, -19, -35, -22, -69
];
// bishop
pub const BB: [i32; 64] = [
    -59, -78, -82, -76, -23,-107, -37, -50,
    -11,  20,  35, -42, -39,  31,   2, -22,
     -9,  39, -32,  41,  52, -10,  28, -14,
     25,  17,  20,  34,  26,  25,  15,  10,
     13,  10,  17,  23,  17,  16,   0,   7,
     14,  25,  24,  15,   8,  25,  20,  15,
     19,  20,  11,   6,   7,   6,  20,  16,
     -7,   2, -15, -12, -14, -15, -10, -10
];
// rook
pub const BR: [i32; 64] = [
    35,  29,  33,   4,  37,  33,  56,  50,
    55,  29,  56,  67,  55,  62,  34,  60,
    19,  35,  28,  33,  45,  27,  25,  15,
     0,   5,  16,  13,  18,  -4,  -9,  -6,
   -28, -35, -16, -21, -13, -29, -46, -30,
   -42, -28, -42, -25, -25, -35, -26, -46,
   -53, -38, -31, -26, -29, -43, -44, -53,
   -30, -24, -18,   5,  -2, -18, -31, -32
];
// queen
pub const BQ: [i32; 64] = [
    6,   1,  -8,-104,  69,  24,  88,  26,
    14,  32,  60, -10,  20,  76,  57,  24,
    -2,  43,  32,  60,  72,  63,  43,   2,
     1, -16,  22,  17,  25,  20, -13,  -6,
   -14, -15,  -2,  -5,  -1, -10, -20, -22,
   -30,  -6, -13, -11, -16, -11, -16, -27,
   -36, -18,   0, -19, -15, -15, -21, -38,
   -39, -30, -31, -13, -31, -36, -34, -42
];
// king
pub const BKG: [i32; 64] = [
    4,  54,  47, -99, -99,  60,  83, -62,
    -32,  10,  55,  56,  56,  55,  10,   3,
    -62,  12, -57,  44, -67,  28,  37, -31,
    -55,  50,  11,  -4, -19,  13,   0, -49,
    -55, -43, -52, -28, -51, -47,  -8, -50,
    -47, -42, -43, -79, -64, -32, -29, -32,
     -4,   3, -14, -50, -57, -18,  13,   4,
     17,  30,  -3, -14,   6,  -1,  40,  18
];
// bkeree 😱
pub const BKE: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -50, -30, -30, -30, -30, -30, -30, -50
];
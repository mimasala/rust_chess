
struct Chessboard{
    fields: [[Field; 8]; 8]
}

struct Field{
    piece: Option<Piece>,
    color: Color
}

struct Piece{
    color: Color,
    piece_type: PieceType
}

enum Color{
    White,
    Black
}
enum PieceType{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

fn main() {

}


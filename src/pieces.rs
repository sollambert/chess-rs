use crate::{board::Color, game::MoveError};

// White Pieces
const W_KING: char = '♔';
const W_QUEEN: char = '♕';
const W_ROOK: char = '♖';
const W_BISHOP: char = '♗';
const W_KNIGHT: char = '♘';
const W_PAWN: char = '♙';

// Black Pieces
const B_KING: char = '♚';
const B_QUEEN: char = '♛';
const B_ROOK: char = '♜';
const B_BISHOP: char = '♝';
const B_KNIGHT: char = '♞';
const B_PAWN: char = '♟';

pub struct Piece {
	pub color: Color,
	pub piece_type: PieceType,
}

impl Piece {
    pub fn char(&self) -> char {
		match self.color {
			Color::White => match self.piece_type {
				PieceType::Pawn => W_PAWN,
				PieceType::Bishop => W_BISHOP,
				PieceType::Knight => W_KNIGHT,
				PieceType::Rook => W_ROOK,
				PieceType::Queen => W_QUEEN,
				PieceType::King => W_KING,
			},
			Color::Black => match self.piece_type {
				PieceType::Pawn => B_PAWN,
				PieceType::Bishop => B_BISHOP,
				PieceType::Knight => B_KNIGHT,
				PieceType::Rook => B_ROOK,
				PieceType::Queen => B_QUEEN,
				PieceType::King => B_KING,
			},
		}
	}

	pub fn new (color: Color, piece_type: PieceType) -> Self {
		Self {
			color, piece_type
		}
	}

    pub fn color(&self) -> Color {
		self.color
	}
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
	Pawn,
	Bishop,
	Knight,
	Rook,
	Queen,
	King,
}

impl TryFrom<char> for PieceType {
	type Error = MoveError;

	fn try_from(char: char) -> Result<Self, Self::Error> {
		match char {
			'a' | 'b' | 'c' | 'd' | 'e' | 'f' => Ok(PieceType::Pawn),
			'N' => Ok(PieceType::Knight),
			'B' => Ok(PieceType::Bishop),
			'R' => Ok(PieceType::Rook),
			'Q' => Ok(PieceType::Queen),
			'K' => Ok(PieceType::King),
			'O' => Ok(PieceType::King),
			_ => Err(MoveError::Notation)
		}
	}
}
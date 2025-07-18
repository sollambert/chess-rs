use std::fmt;

use crate::{board::{Board, Color, Coordinate, File, Rank}, pieces::PieceType};

pub struct Game {
	pub board: Board,
	pub turns: Vec<Turn>,
	pub players: (Player, Player)
}

impl Game {
	pub fn turns_to_string(&self) -> String {
		let mut turns_iter = self.turns.iter();
		let mut turn_option: Option<&Turn> = turns_iter.next();
		let mut turn_number = 0;
		let mut turn_strings: Vec<String> = Vec::<String>::new();
		while turn_option.is_some() {
			let turn = turn_option.unwrap();
			turn_number += 1;
			turn_strings.push(format!("{}. {}", turn_number, turn));
			turn_option = turns_iter.next();
		}
		turn_strings.join("; ")
	}
	pub fn new(time: u32) -> Self {
		Self {
			board: Board::default(),
			turns: Vec::<Turn>::new(),
			players: (Player::new(Color::White, time), Player::new(Color::Black, time))
		}
	}
}

pub enum CastlingRights {
	Both,
	Queen,
	King,
	None
}

pub struct Player {
	pub castling_rights: CastlingRights,
	pub color: Color,
	pub is_checked: bool,
	pub time: u32,
}

impl Player {
	pub fn increment_time(&mut self, time: u32) {
		self.time += time;
	}
	pub fn decrement_time(&mut self, time: u32) {
		self.time -= time;
	}
	pub fn is_checked(&self) -> bool {
		self.is_checked
	}
	pub fn set_checked(&mut self, checked: bool) {
		self.is_checked = checked
	}
	pub fn color(&self) -> Color {
		self.color
	}
	pub fn remove_castling_rights(&mut self, rights: CastlingRights) {
		match rights {
			CastlingRights::Both => self.castling_rights = CastlingRights::None,
			CastlingRights::Queen => match self.castling_rights {
				CastlingRights::Both => self.castling_rights = CastlingRights::King,
				_ => self.castling_rights = CastlingRights::None,
			},
			CastlingRights::King => match self.castling_rights {
				CastlingRights::Both => self.castling_rights = CastlingRights::Queen,
				_ => self.castling_rights = CastlingRights::None,
			},
			_ => {},
		}
	}
	pub fn new(color: Color, time: u32) -> Self {
		Self {
			castling_rights: CastlingRights::Both,
			color,
			is_checked: false,
			time,
		}
	}
}

const KINGSIDE_CASTLE: &str = "O-O";
const QUEENSIDE_CASTLE: &str = "O-O-O";

#[derive(Clone)]
pub struct Move {
	pub notation: String,
	pub from: Coordinate,
	pub to: Coordinate,
	pub piece_type: PieceType,
	pub promotion: Option<PieceType>,
	pub move_type: MoveType,
	pub is_check: bool,
	pub is_mate: bool,
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let chars = value.as_bytes();
		let piece_type = match PieceType::try_from(char::from_u32(chars[0] as u32).unwrap()) {
			Ok(pt) => pt,
			Err(_) => return Err(MoveError::Notation.as_str()),
		};
		let move_type = match MoveType::try_from(value) {
			Ok(mt) => mt,
			Err(_) => return Err(MoveError::Notation.as_str()),
		};
		let promotion = match move_type {
			MoveType::Promotion => {
				Some(match PieceType::try_from(value.chars().last().unwrap()) {
						Ok(pt) => pt,
						Err(err) => return Err(err.as_str()),
				})
			},
			_ => None
		};
		let from: Coordinate = if piece_type == PieceType::Pawn {
			Coordinate::from((chars[0] as char, chars[1] as char))
		} else {
			Coordinate::from((chars[1] as char, chars[2] as char))
		};
		let to: Coordinate = if piece_type == PieceType::Pawn {
			Coordinate::from((chars[2] as char, chars[3] as char))
		} else {
			Coordinate::from((chars[3] as char, chars[4] as char))
		};
		let chess_move = Move {
			notation: value.to_string(),
			from,
			to,
			piece_type,
			promotion,
			move_type,
			is_check: false,
			is_mate: false,
		};
		Ok(chess_move)
	}
}

impl Default for Move {
	fn default() -> Self {
		Self {
			notation: "None".to_string(),
			piece_type: PieceType::Pawn,
			from: Default::default(),
			to: Default::default(),
			promotion: Default::default(),
			move_type: Default::default(),
			is_check: Default::default(),
			is_mate: Default::default(),
		}
	}
}

pub enum MoveError {
	Blocked,
	InCheck,
	Invalid,
	Pinned,
	Notation,
	OutOfBounds,
	PawnNoCapture,
	PromotionRank,
	
}

impl MoveError {
	pub fn as_str(&self) -> &'static str {
		match self {
			MoveError::InCheck => "King is in check!",
			MoveError::Invalid => "Move is invalid!",
			MoveError::Pinned => "Piece is currently pinned!",
			MoveError::Notation => "Issue parsing notation!",
			MoveError::OutOfBounds => todo!(),
			MoveError::PawnNoCapture => "No opposing piece at destination!",
			MoveError::PromotionRank => "Cannot promote on this rank!",
			MoveError::Blocked => "Movement is blocked!",
		}
	}
}

#[derive(Clone, Default, PartialEq)]
pub enum MoveType {
	#[default]
	Normal,
	CastleKingSide,
	CastleQueenSide,
	Promotion
}

impl TryFrom<&str> for MoveType {
	type Error = MoveError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let promotion = PieceType::try_from(value.chars().last().unwrap());
		if let Ok(_) = promotion { return Ok(MoveType::Promotion) };
		if value.contains("O-O-O") {
			return Ok(MoveType::CastleQueenSide);
		}
		if value.contains("O-O") {
			return Ok(MoveType::CastleKingSide);
		}
		let piece_type = PieceType::try_from(value.chars().next().unwrap());
		if let Ok(_) = piece_type { return Ok(MoveType::Normal) };
		Err(MoveError::Notation)
	}
}

pub struct Turn(Option<Move>, Option<Move>);

impl fmt::Display for Turn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}",
			self.0.clone().unwrap_or(Move::default()).notation,
			self.1.clone().unwrap_or(Move::default()).notation
		)
	}
}
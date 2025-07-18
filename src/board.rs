use std::{array::from_fn, fmt::Display};

use crate::{game::{CastlingRights, Move, MoveError, MoveType, Player}, pieces::{Piece, PieceType}};

const B_SQUARE: char = '▓';
const W_SQUARE: char = '░';

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::White => "White",
            Color::Black => "Black",
        })
    }
}

impl From<Color> for char {
    fn from(value: Color) -> Self {
        match value {
            Color::White => W_SQUARE,
            Color::Black => B_SQUARE,
        }
    }
}

pub struct Board {
    pub squares: [[Square; 8]; 8],
	pub can_passant: Option<Coordinate>,
}

impl Board {
    pub fn to_string(&self, perspective: BoardPerspective) -> String {
        let mut display_string: String = "  ▁▁▁▁▁▁▁▁\n".to_string();
		let mut file_str = "abcdefgh".to_string();
		let square_iter: Box<dyn Iterator<Item = &[Square; 8]>> = match perspective {
			BoardPerspective::Black => {
				file_str = file_str.chars().rev().collect::<String>();
				Box::new(self.squares.iter())
			},
			BoardPerspective::White => Box::new(self.squares.iter().rev()),
		};
		let rank_str_builder = |rank: &[Square; 8]| {
			let rank_iter: Box<dyn Iterator<Item = &Square>> = match perspective {
				BoardPerspective::Black => Box::new(rank.iter().rev()),
				BoardPerspective::White => Box::new(rank.iter()),
			};
			let mut rank_string: String = rank_iter.map(|square| square.char()).collect();
			rank_string.push_str("▏\n");
			rank_string.insert(0, '▕');
			rank_string.insert(0, rank.first().unwrap().coordinate.rank.into());
			rank_string
		};
		display_string.push_str(&square_iter.map(rank_str_builder).collect::<String>());
		display_string.push_str("  ▔▔▔▔▔▔▔▔\n");
		display_string.push_str(format!("  {}\n", file_str).as_str());
        display_string
    }
	pub fn get_square(&self, coordinate: Coordinate) -> &Square {
		&self.squares[coordinate.rank as usize][coordinate.file as usize]
	}
	pub fn get_square_mut(&mut self, coordinate: Coordinate) -> &mut Square {
		&mut self.squares[coordinate.rank as usize][coordinate.file as usize]
	}
	pub fn execute_move(&mut self, chess_move: Move, player: &mut Player) -> Result<(), MoveError> {
        let valid = self.validate_move(&chess_move, player);
		if valid.is_err() {return valid;}
		let mut piece_type = chess_move.piece_type;
		match chess_move.move_type {
			MoveType::CastleKingSide | MoveType::CastleQueenSide => player.remove_castling_rights(CastlingRights::Both),
			MoveType::Promotion => piece_type = chess_move.promotion.unwrap(),
			_ => {}
		};
		match piece_type {
			PieceType::Pawn => {
				let starting_rank = match player.color {
					Color::White => Rank::Two,
					Color::Black => Rank::Seven,
				};
				if self.can_passant.is_some() {
					let passant_pawn_coord = match player.color {
						Color::White => Coordinate {
							rank: Rank::try_from(chess_move.to.rank as usize - 1).unwrap(),
							file: chess_move.to.file,
						},
						Color::Black => Coordinate {
							rank: Rank::try_from(chess_move.to.rank as usize + 1).unwrap(),
							file: chess_move.to.file,
						},
					};
					if chess_move.to == self.can_passant.unwrap() {
						self.get_square_mut(passant_pawn_coord).piece = None;
					}
				}
				if chess_move.from.rank == starting_rank && i32::abs(chess_move.from.rank as i32 - chess_move.to.rank as i32) == 2 {
					self.can_passant = match player.color {
						Color::White => Some(Coordinate {
							rank: Rank::try_from(chess_move.from.rank as usize + 1).unwrap(),
							file: chess_move.from.file,
						}),
						Color::Black => Some(Coordinate {
							rank: Rank::try_from(chess_move.from.rank as usize - 1).unwrap(),
							file: chess_move.from.file,
						}),
					};
				}
			},
			PieceType::Bishop => {
				self.can_passant = None;
			},
			PieceType::Knight => {
				self.can_passant = None;
			},
			PieceType::Rook => {
				self.can_passant = None;
				let starting_rank = match player.color {
					Color::White => Rank::One,
					Color::Black => Rank::Eight,
				};
				if chess_move.from.rank == starting_rank {
					match chess_move.from.file {
							File::A => player.remove_castling_rights(CastlingRights::Queen),
							File::H => player.remove_castling_rights(CastlingRights::King),
							_ => ()
					};
				}
			},
			PieceType::Queen => {
				self.can_passant = None;
			},
			PieceType::King => {
				self.can_passant = None;
			},
		}
		self.get_square_mut(chess_move.from).piece = None;
		self.get_square_mut(chess_move.to).piece = Some(Piece::new(player.color, piece_type));
		Ok(())
	}
	fn validate_move(&self, chess_move: &Move, player: &Player) -> Result<(), MoveError> {
		if chess_move.from == chess_move.to {
			return Err(MoveError::Invalid)
		}
		let current_piece = &self.get_square(chess_move.from).piece;
		if let Some(piece) = current_piece {
			if piece.color() != player.color() || chess_move.piece_type != piece.piece_type {
				return Err(MoveError::Invalid);
			}
		} else {
			return Err(MoveError::Invalid);
		}
		let occupying_piece = &self.get_square(chess_move.to).piece;
		if let Some(piece) = occupying_piece {
			if piece.color() == player.color() {
				return Err(MoveError::Blocked);
			}
		}
		match chess_move.piece_type {
			PieceType::Pawn => {
				let file_adjacent = (chess_move.to.file as usize).abs_diff(chess_move.from.file as usize) == 1;
				let same_file = chess_move.to.file == chess_move.from.file;
				let passantable = self.can_passant.is_some() && chess_move.to == self.can_passant.unwrap();
				let promote_rank = match player.color {
						Color::White => Rank::Eight,
						Color::Black => Rank::One,
					};
				let forward_one = match player.color {
					Color::White => chess_move.to.rank as usize == chess_move.from.rank as usize + 1,
					Color::Black => chess_move.to.rank as usize == chess_move.from.rank as usize - 1,
				};
				let forward_two = match player.color {
					Color::White => chess_move.to.rank as usize == chess_move.from.rank as usize + 2,
					Color::Black => chess_move.to.rank as usize == chess_move.from.rank as usize - 2,
				};
				let on_starting_rank = match player.color {
					Color::White => chess_move.from.rank == Rank::Two,
					Color::Black => chess_move.from.rank == Rank::Seven,
				};
				if !forward_one && !(forward_two && on_starting_rank) {
					return Err(MoveError::Invalid);
				}
				if file_adjacent {
					if occupying_piece.is_none() && !passantable {
						return Err(MoveError::PawnNoCapture);
					}
				} else {
					if occupying_piece.is_some() {
						return Err(MoveError::Blocked);
					}
				}
				if !file_adjacent && !same_file {
					return Err(MoveError::Invalid);
				}
				if chess_move.move_type == MoveType::Promotion && !(promote_rank == chess_move.to.rank) {
					return Err(MoveError::PromotionRank);
				}
			},
			PieceType::Bishop => {
				let file_diff: i32 = i32::wrapping_sub((chess_move.to.file as usize) as i32, (chess_move.from.file as usize) as i32);
				let rank_diff: i32 = i32::wrapping_sub((chess_move.to.rank as usize) as i32, (chess_move.from.rank as usize) as i32);
				let direction: (bool, bool) = (file_diff > 0, rank_diff > 0);
				if file_diff != rank_diff && file_diff != rank_diff * -1 {
					return Err(MoveError::Invalid);
				}
				for distance in 1..i32::abs(file_diff) {
					let file_distance = if direction.0 { distance } else { distance * -1 };
					let rank_distance = if direction.1 { distance } else { distance * -1 };
					let check_coord = Coordinate {
						file: File::try_from((chess_move.from.file as usize as i32 + file_distance) as usize).unwrap(),
						rank: Rank::try_from((chess_move.from.rank as usize as i32 + rank_distance) as usize).unwrap()
					};
					let check_occupied = self.get_square(check_coord).piece.is_some();
					if check_occupied {
						return Err(MoveError::Blocked);
					}
				}
			},
			PieceType::Knight => {
				let valid_diffs = [[2, 1], [1, 2], [-1, 2] ,[-2, 1], [-2, -1], [-1, -2], [1, -2], [2, -1]];
				let valid_coords: Vec<Coordinate> = valid_diffs.iter().map(|diff| {
					let file_result = File::try_from((chess_move.from.file as usize as i32 + diff[0]) as usize);
					let rank_result = Rank::try_from((chess_move.from.rank as usize as i32 + diff[1]) as usize);
					if file_result.is_err() || rank_result.is_err() {
						return None;
					}
					let file = file_result.unwrap();
					let rank = rank_result.unwrap();
					let coord = Coordinate {
						file,
						rank
					};
					Some(coord)
				}).filter(|coord| coord.is_some()).map(|coord| coord.unwrap()).collect();
				let target_coord = valid_coords.iter().find(|coord| **coord == chess_move.to);
				if target_coord.is_none() {return Err(MoveError::Invalid)}
			},
			PieceType::Rook => {
				let file_diff: i32 = i32::wrapping_sub((chess_move.to.file as usize) as i32, (chess_move.from.file as usize) as i32);
				let rank_diff: i32 = i32::wrapping_sub((chess_move.to.rank as usize) as i32, (chess_move.from.rank as usize) as i32);
				let direction: (bool, bool) = (file_diff != 0, file_diff > 0 || rank_diff > 0 );
				if file_diff * rank_diff != 0 {
					return Err(MoveError::Invalid);
				}
				let check_coord = |distance| {
					let file_distance = if direction.0 { if direction.1 { distance } else { distance * -1 }} else { 0 };
					let rank_distance = if !direction.0 { if direction.1 { distance } else { distance * -1 }} else { 0 };
					let coord = Coordinate {
						file: File::try_from((chess_move.from.file as usize as i32 + file_distance) as usize).unwrap(),
						rank: Rank::try_from((chess_move.from.rank as usize as i32 + rank_distance) as usize).unwrap()
					};
					self.get_square(coord).piece.is_some()
				};
				for distance in 1..i32::abs(file_diff) {
					if check_coord(distance) {
						return Err(MoveError::Blocked);
					}
				}
				for distance in 1..i32::abs(rank_diff) {
					if check_coord(distance) {
						return Err(MoveError::Blocked);
					}
				}
			},
			PieceType::Queen => {
			},
			PieceType::King => {
			},
		};
		return Ok(())
	}
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(BoardPerspective::White))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
			can_passant: None,
            squares: from_fn::<[Square;8], 8, _>(|rank| {
                from_fn::<Square, 8, _>(|file| {
                    let index = rank * 8 + file;
                    let color = if (rank + file) % 2 == 0 { Color::Black } else { Color::White };
                    let file = File::try_from(file).unwrap();
                    let rank =  Rank::try_from(rank).unwrap();
                    let coordinate = Coordinate {
                        file,
                        rank
                    };
                    let piece_color = if rank == Rank::One || rank == Rank::Two { Color::White} else { Color::Black };
                    let piece: Option<Piece> = match rank {
                        Rank::One | Rank::Eight => {
                            match file {
                                File::A | File::H => Some(Piece::new(piece_color, PieceType::Rook)),
                                File::B | File::G=> Some(Piece::new(piece_color, PieceType::Knight)),
                                File::C | File::F => Some(Piece::new(piece_color, PieceType::Bishop)),
                                File::D => Some(Piece::new(piece_color, PieceType::Queen)),
                                File::E => Some(Piece::new(piece_color, PieceType::King)),
                            }
                        },
                        Rank::Two | Rank::Seven => Some(Piece::new(piece_color, PieceType::Pawn)),
                        _ => None,
                    };
                    Square {
                        coordinate,
                        color,
                        index,
                        piece,
                    }
                })
            })
        }
    }
}

#[derive(PartialEq)]
pub enum BoardPerspective {
	White,
	Black
}

impl From<Color> for BoardPerspective {
	fn from(value: Color) -> Self {
		match value {
			Color::White => Self::White,
			Color::Black => Self::Black,
		}
	}
}

pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub index: usize,
    pub color: Color
}

impl Square {
    fn char(&self) -> char {
        match &self.piece {
            Some(piece) => piece.char(),
            None => self.color.into(),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index: {}\nChar: {}\nCoord: {}\nColor: {}", self.index, self.char(), self.coordinate, self.color)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub file: File,
    pub rank: Rank
}

impl Default for Coordinate {
	fn default() -> Self {
		Self { file: File::A, rank: Rank::One }
	}
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", <File as Into<char>>::into(self.file), <Rank as Into<char>>::into(self.rank))
    }
}

impl From<(char, char)> for Coordinate {
	fn from(value: (char, char)) -> Self {
		Self {
			file: File::try_from(value.0).unwrap(),
			rank: Rank::try_from(value.1).unwrap()
		}
	}
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl TryFrom<u8> for File {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err("Value out of range for File (0–7)"),
        }
    }
}

macro_rules! impl_try_from_file {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for File {
                type Error = &'static str;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    // Convert to u8 safely, and then reuse the existing implementation
                    if value >= 0 && value <= 7 {
                        // `as u8` is safe here due to range check
                        File::try_from(value as u8)
                    } else {
                        Err("Value out of range for File (0–7)")
                    }
                }
            }
        )*
    };
}

impl_try_from_file!(i8, i16, i32, i64, isize, u16, u32, u64, usize);

impl TryFrom<char> for File {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(()),
        }
    }
}

impl Into<char> for File {
    fn into(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight
}

impl Into<char> for Rank {
    fn into(self) -> char {
        match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::One),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            _ => Err("Value out of range for Rank (0–7)"),
        }
    }
}

macro_rules! impl_try_from_rank {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for Rank {
                type Error = &'static str;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    // Convert to u8 safely, and then reuse the existing implementation
                    if value >= 0 && value <= 7 {
                        // `as u8` is safe here due to range check
                        Rank::try_from(value as u8)
                    } else {
                        Err("Value out of range for Rank (0–7)")
                    }
                }
            }
        )*
    };
}

impl_try_from_rank!(i8, i16, i32, i64, isize, u16, u32, u64, usize);

impl TryFrom<char> for Rank {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Rank::One),
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            _ => Err(()),
        }
    }
}
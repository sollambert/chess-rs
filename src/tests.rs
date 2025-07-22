#[cfg(test)]
mod tests {
    use crate::{board::{Board, BoardPerspective, File, Rank}, game::{Game, Move}, pieces::{Piece, PieceType}};

    #[test]
    fn init_board() {
        let  board = Board::default();
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
    }

    #[test]
    fn print_squares() {
        let board = Board::default();
        board.squares.iter().for_each(|rank| rank.iter().for_each(|square| {
            println!("***********************************\n{}", square);
        }));
    }

    #[test]
    fn valid_pawn_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("e2e3", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_pawn_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("e7e6", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_pawn_double_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("e2e4", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        print!("Can Passant: {}", board.can_passant.unwrap());
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_pawn_double_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("e7e5", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        print!("Can Passant: {}", board.can_passant.unwrap());
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_passant() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let player2 = &mut game.players.1;
        let chess_move = match Move::try_from(("e2e4", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let chess_move2 = match Move::try_from(("d4e3", player2.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Four as usize][File::D as usize].piece = Some(Piece::new(player2.color, PieceType::Pawn));
        let _ = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move2, player2);
        print!("{}", board.to_string(player2.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }
    #[test]
    fn valid_passant_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let player2 = &mut game.players.1;
        let chess_move = match Move::try_from(("e7e5", player2.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let chess_move2 = match Move::try_from(("d5e6", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Five as usize][File::D as usize].piece = Some(Piece::new(player.color, PieceType::Pawn));
        let _ = board.execute_move(&chess_move, player2);
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move2, player);
        print!("{}", board.to_string(player2.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_bishop_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Bc1a3", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::B as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_bishop_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Bc8a6", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Seven as usize][File::B as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_knight_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Ng1f3", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_knight_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Ng8f6", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_rook_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Ra1a7", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::A as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_rook_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Ra8a2", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Seven as usize][File::A as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queen_file_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Qd1d4", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::D as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queen_file_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Qd1d4", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[1][3].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queen_diag_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Qd1b3", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::C as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queen_diag_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Qd1b3", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[1][2].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_king_file_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Ke1e2", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::D as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_king_file_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Ke8e7", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Seven as usize][File::E as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_king_diag_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("Ke1d2", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Two as usize][File::D as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_king_diag_move_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("Ke8d7", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Seven as usize][File::D as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_kingside_castle() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("O-O", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[0][5].piece = None;
        board.squares[0][6].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queenside_castle() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("O-O-O", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[0][1].piece = None;
        board.squares[0][2].piece = None;
        board.squares[0][3].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_kingside_castle_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("O-O", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Eight as usize][File::F as usize].piece = None;
        board.squares[Rank::Eight as usize][File::G as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_queenside_castle_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("O-O-O", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Eight as usize][File::B as usize].piece = None;
        board.squares[Rank::Eight as usize][File::C as usize].piece = None;
        board.squares[Rank::Eight as usize][File::D as usize].piece = None;
        print!("{}", board.to_string(player.color.into()));
        let move_result = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn promotion() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.0;
        let chess_move = match Move::try_from(("a7a8=Q", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::Eight as usize][File::A as usize].piece = None;
        board.squares[Rank::Seven as usize][File::A as usize].piece = Some(Piece {
            color: player.color(),
            piece_type: PieceType::Pawn,
        });
        print!("{}", board.to_string(player.color.into()));
        let executed_move = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(executed_move.is_ok(), "{}", executed_move.unwrap_err().as_str());
    }

    #[test]
    fn promotion_black() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let player = &mut game.players.1;
        let chess_move = match Move::try_from(("a2a1=Q", player.color)) {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        board.squares[Rank::One as usize][File::A as usize].piece = None;
        board.squares[Rank::Two as usize][File::A as usize].piece = Some(Piece {
            color: player.color(),
            piece_type: PieceType::Pawn,
        });
        print!("{}", board.to_string(player.color.into()));
        let executed_move = board.execute_move(&chess_move, player);
        print!("{}", board.to_string(player.color.into()));
        assert!(executed_move.is_ok(), "{}", executed_move.unwrap_err().as_str());
    }
}
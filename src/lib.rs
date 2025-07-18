mod board;
mod game;
mod pieces;

#[cfg(test)]
mod tests {
    use crate::{board::{Board, BoardPerspective, Color, Coordinate, File, Rank, Square}, game::{self, Game, Move, MoveType}, pieces::{Piece, PieceType}};

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
        let chess_move = match Move::try_from("e2e3") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_pawn_double_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("e2e4") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        print!("Can Passant: {}", board.can_passant.unwrap());
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_passant() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("e2e4") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let chess_move2 = match Move::try_from("d4e3") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        let player2 = &mut game.players.1;
        board.squares[3][3].piece = Some(Piece::new(player2.color, PieceType::Pawn));
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::White));
        let move_result = board.execute_move(chess_move2, player2);
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_bishop_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("Bc1a3") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        board.squares[1][1].piece = None;
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_knight_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("Ng1f3") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn valid_rook_move() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("Ra1a6") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        board.squares[1][0].piece = None;
        let move_result = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(move_result.is_ok(), "{}", move_result.unwrap_err().as_str());
    }

    #[test]
    fn promotion() {
        let mut game = Game::new(3600);
        let mut board = game.board;
        let chess_move = match Move::try_from("a7a8=Q") {
            Ok(cm) => cm,
            Err(err) => panic!("{}", err),
        };
        let player = &mut game.players.0;
        board.squares[7][0].piece = None;
        board.squares[6][0].piece = Some(Piece {
            color: player.color(),
            piece_type: PieceType::Pawn,
        });
        let executed_move = board.execute_move(chess_move, player);
        print!("{}", board.to_string(BoardPerspective::Black));
        print!("\n");
        print!("{}", board.to_string(BoardPerspective::White));
        assert!(executed_move.is_ok(), "{}", executed_move.unwrap_err().as_str());
    }
}
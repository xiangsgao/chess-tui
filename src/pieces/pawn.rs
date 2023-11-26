use super::{PieceColor, PieceType};
use crate::utils::{cleaned_positions, get_piece_color, is_cell_color_ally, is_valid};

pub struct Pawn {}

impl Pawn {
    pub fn to_string() -> &'static str {
        "\
        \n\
        \n\
      ▟█▙\n\
      ▜█▛\n\
     ▟███▙\n\
    "
    }

    pub fn pawn_moves(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        allow_move_on_ally_positions: bool,
        latest_move: (Option<PieceType>, i32),
    ) -> Vec<Vec<i32>> {
        // Pawns can only move in one direction depending of their color
        // -1 if they are white (go up) +1 if they are black (go down)
        let direction = if color == PieceColor::White { -1 } else { 1 };

        let mut positions: Vec<Vec<i32>> = vec![];

        let (y, x) = (coordinates[0], coordinates[1]);

        // move one in front
        let new_x_front_one = x;
        let new_y_front_one = y + direction;
        let new_coordinates_front_one = [new_y_front_one, new_x_front_one];

        if is_valid(new_coordinates_front_one)
            && !allow_move_on_ally_positions
            && get_piece_color(board, new_coordinates_front_one).is_none()
        {
            // Empty cell
            positions.push(new_coordinates_front_one.to_vec());

            // move front a second cell
            let new_x_front_two = x;
            let new_y_front_two = y + direction * 2;
            let new_coordinates_front_two = [new_y_front_two, new_x_front_two];

            if is_valid(new_coordinates_front_two)
                && get_piece_color(board, new_coordinates_front_two).is_none()
                && ((color == PieceColor::White && y == 6)
                    || (color == PieceColor::Black && y == 1))
            {
                positions.push(new_coordinates_front_two.to_vec());
            }
        }

        // check for enemy piece on the right
        let new_x_right = x + 1;
        let new_y_right = y + direction;
        let new_coordinates_right = [new_y_right, new_x_right];

        // check for enemy piece on the left
        let new_x_left = x - 1;
        let new_y_left = y + direction;
        let new_coordinates_left = [new_y_left, new_x_left];

        // If we allow on ally position we push it anyway

        if allow_move_on_ally_positions {
            if is_valid(new_coordinates_right) {
                positions.push(new_coordinates_right.to_vec())
            };
            if is_valid(new_coordinates_left) {
                positions.push(new_coordinates_left.to_vec())
            };
        } else {
            // else we check if it's an ally piece
            if is_valid(new_coordinates_right)
                && get_piece_color(board, new_coordinates_right).is_some()
                && !is_cell_color_ally(board, new_coordinates_right, color)
            {
                positions.push(new_coordinates_right.to_vec());
            }
            if is_valid(new_coordinates_left)
                && get_piece_color(board, new_coordinates_left).is_some()
                && !is_cell_color_ally(board, new_coordinates_left, color)
            {
                positions.push(new_coordinates_left.to_vec());
            }
        }

        // We check for en passant
        match latest_move.0 {
            Some(PieceType::Pawn) => {
                let from_y: i32 = latest_move.1 / 1000 % 10;
                let from_x: i32 = latest_move.1 / 100 % 10;
                let to_y: i32 = latest_move.1 / 10 % 10;
                let to_x: i32 = latest_move.1 % 10;
                let valid_y_start: i32;
                let number_of_cells_move: i32;

                if color == PieceColor::White {
                    valid_y_start = 1;
                    number_of_cells_move = to_y - from_y;
                } else {
                    valid_y_start = 6;
                    number_of_cells_move = from_y - to_y;
                };

                // We check if the latest move was on the right start cell
                // if it moved 2 cells
                // and if the current pawn is next to this pawn latest position
                if from_y == valid_y_start
                    && number_of_cells_move == 2
                    && y == to_y
                    && (x == to_x - 1 || x == to_x + 1)
                {
                    let new_y = from_y + direction * -1;
                    let new_x = from_x;
                    positions.push([new_y, new_x].to_vec());
                }
            }
            _ => {}
        }

        cleaned_positions(positions)
    }
    pub fn authorized_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
        latest_move: (Option<PieceType>, i32),
    ) -> Vec<Vec<i32>> {
        Self::pawn_moves(coordinates, color, board, false, latest_move)
    }

    pub fn protecting_positions(
        coordinates: [i32; 2],
        color: PieceColor,
        board: [[Option<(PieceType, PieceColor)>; 8]; 8],
    ) -> Vec<Vec<i32>> {
        Self::pawn_moves(coordinates, color, board, true, (None, 0000))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::Board,
        pieces::{pawn::Pawn, PieceColor, PieceType},
    };

    #[test]
    fn pawn_moves_one_cell_forward() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![3, 4]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([4, 4], PieceColor::White, board.board, (None, 0000));
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_forward_two() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![5, 4], vec![4, 4]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([6, 4], PieceColor::White, board.board, (None, 0000));
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_enemy_left_right() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 3], vec![3, 3], vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([1, 3], PieceColor::Black, board.board, (None, 0000));
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_moves_one_cell_3_enemies() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
            ],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 4], vec![2, 2]];
        right_positions.sort();

        let mut positions =
            Pawn::authorized_positions([1, 3], PieceColor::Black, board.board, (None, 0000));
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_en_passant_white() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 2], vec![2, 3]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [3, 3],
            PieceColor::White,
            board.board,
            (Some(PieceType::Pawn), 1232),
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_en_passant_black() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![5, 2], vec![5, 3]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [4, 2],
            PieceColor::Black,
            board.board,
            (Some(PieceType::Pawn), 6343),
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }

    #[test]
    fn pawn_not_en_passant() {
        let custom_board = [
            [None, None, None, None, None, None, None, None],
            [
                None,
                Some((PieceType::Pawn, PieceColor::Black)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                None,
                None,
                None,
                Some((PieceType::Pawn, PieceColor::White)),
                None,
                None,
                None,
                None,
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
        let mut board = Board::default();
        board.set_board(custom_board);

        let mut right_positions = vec![vec![2, 1], vec![3, 1]];
        right_positions.sort();

        let mut positions = Pawn::authorized_positions(
            [1, 1],
            PieceColor::Black,
            board.board,
            (Some(PieceType::Pawn), 6343),
        );
        positions.sort();
        assert_eq!(right_positions, positions);
    }
}

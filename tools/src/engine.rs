//black --> negatives
//red --> positives
//|1| --> man  |2| --> king  0 --> empty

use crate::errors::CheckersError;


/// Moves that can be taken on a tile
pub enum Action {
    MoveNorthwest,
    MoveNortheast,
    MoveSouthwest,
    MoveSoutheast,
    JumpNorthwest,
    JumpNortheast,
    JumpSouthwest,
    JumpSoutheast,
}


/// The result of making a move on the board
#[derive(Debug)]
pub enum CheckersResult {
    Ok(bool),       //turn is over, contains the current player's turn
    Win(bool),            //win
}


/// The engine for a Checkers game
pub struct Engine {
    pub board_red: [i8; 32],
    pub board_black: [i8; 32],
    pub current_player: bool, //true = red, false = black
    pub red_pieces: u8,
    pub black_pieces: u8,
}
impl Engine {
    pub fn new() -> Self {
        let board = [-1,-1,-1,-1,-1,-1,-1,-1,-1,0, -1,0, -1,0, -1,0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        //           0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31

        Engine {
            board_red: board.clone(),
            board_black: board,
            current_player: false,
            red_pieces: 12,
            black_pieces: 12,
        }
    }

    /// Returns the tile a piece would land on given a specific action, can return invalid tiles
    pub fn action_on_tile(tile: u8, action: &Action) -> u8 {
        match action {
            Action::MoveNorthwest => {
                if tile % 2 == 0 {
                    tile - 9
                } else {
                    tile - 1
                }
            }
            Action::MoveNortheast => {
                if tile % 2 == 0 {
                    tile - 7
                } else {
                    tile + 1
                }
            }
            Action::MoveSouthwest => {
                if tile % 2 == 0 {
                    tile - 1
                } else {
                    tile + 7
                }
            }
            Action::MoveSoutheast => {
                if tile % 2 == 0 {
                    tile + 1
                } else {
                    tile + 9
                }
            }
            Action::JumpNorthwest => tile - 10,
            Action::JumpNortheast => tile - 6,
            Action::JumpSouthwest => tile + 6,
            Action::JumpSoutheast => tile + 10,
        }
    }

    /// Checks if the move can be completed for this player.
    pub fn is_move_valid(&self, tile: u8, action: &Action) -> bool {
        if tile > 31 {
            return false;
        }

        let board = self.get_board();
        let piece = board[tile as usize];

        if piece <= 0 { //ensures there's a moveable piece on this tile for this player
            return false;
        }

        match action {
            Action::MoveNorthwest => {
                if (tile % 2 == 0) && (tile % 8 == 0 || tile < 8) { 
                    return false;
                } //left and top edge
            }
            Action::MoveNortheast => {
                if tile % 2 == 0 {
                    if tile < 8 {
                        return false;
                    }
                }
                //top edge
                else {
                    if ((tile + 1) % 8) == 0 {
                        return false;
                    }
                } //right edge
            }
            Action::MoveSouthwest => {
                if tile % 2 == 0 {
                    if tile % 8 == 0 || piece == 1 {
                        return false;
                    }
                }
                //left edge
                else {
                    if tile > 23 {
                        return false;
                    }
                } //bottom edge
            }
            Action::MoveSoutheast => {
                if (tile % 2 == 1) && (tile > 23 || ((tile + 1) % 8 == 0)) || piece == 1 {
                    return false;
                } //right and bottom edge
            }
            Action::JumpNorthwest => {
                if (tile < 8) || (tile % 8 == 0) || (tile - 1) % 8 == 0 {
                    //left 2 and top 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveNorthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpNortheast => {
                if (tile < 8) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) {
                    //right 2 and top 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveNortheast) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSouthwest => {
                if (tile > 23) || (tile % 8 == 0) || ((tile - 1) % 8 == 0) || piece == 1 {
                    //left 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveSouthwest) as usize] >= 0 {
                    return false;
                }
            }
            Action::JumpSoutheast => {
                if (tile > 23) || ((tile + 1) % 8 == 0) || ((tile + 2) % 8 == 0) || piece == 1 {
                    //right 2 and bottom 2 edges
                    return false;
                }
                if board[Engine::action_on_tile(tile, &Action::MoveSoutheast) as usize] >= 0 {
                    return false;
                }
            }
        }

        board[Engine::action_on_tile(tile, action) as usize] == 0 //make sure spot is open
    }

    // Performs the specified move or defines the error message if the move is invalid
    pub fn make_move(&mut self, tile: u8, action: Action) -> Result<CheckersResult, CheckersError> {
        if !Engine::is_move_valid(&self, tile, &action) { //ensures coordinates are respected and spaces are open
            return Err(CheckersError::IllegalMove);
        }

        // we need to first perform the move, then take any automatic actions if necessary
        // question for implementing: if the enemy makes a move, and the player is forced to move,
        // does this mean their whole turn has completed without them having a say in what they moved?
        // more research is required

        // perform the move, NOTE: in move cases the turn will switch to the opponent for sure, not necessarily true for jumps
        let landing_tile = Engine::action_on_tile(tile, &action);
        match &action {
            Action::MoveNorthwest | Action::MoveNortheast | Action::MoveSouthwest | Action::MoveSoutheast => {
                self.update_board(landing_tile, self.get_board()[tile as usize]); //copy piece to new tile
                self.update_board(tile, 0); //remove piece from current tile
                self.handle_king(landing_tile); //update to king after the move

                // CASE I
                // Give control to the other team, and check the four adjacent spaces for any automatic moves
                // If only one such case then execute automatically, otherwise ask the ai
                self.current_player = !self.current_player;
                return self.handle_inward_jump(31 - landing_tile); //flip the perspective
            },
            Action::JumpNorthwest => {
                let killed_tile = Engine::action_on_tile(tile, &Action::MoveNorthwest);
                self.update_board(killed_tile, 0); //delete killed piece
            },
            Action::JumpNortheast => {
                let killed_tile = Engine::action_on_tile(tile, &Action::MoveNortheast);
                self.update_board(killed_tile, 0); //delete killed piece
            },
            Action::JumpSouthwest => {
                let killed_tile = Engine::action_on_tile(tile, &Action::MoveSouthwest);
                self.update_board(killed_tile, 0); //delete killed piece
            },
            Action::JumpSoutheast => {
                let killed_tile = Engine::action_on_tile(tile, &Action::MoveSoutheast);
                self.update_board(killed_tile, 0); //delete killed piece
            },
        }
        // CASE II
        // Check adjacent spaces for additional jumps 
        if self.current_player {self.black_pieces -= 1;} else {self.red_pieces -= 1;} //update piece count
        if self.black_pieces == 0 || self.red_pieces == 0 {return Ok(CheckersResult::Win(self.current_player));} //check win
        self.update_board(landing_tile, self.get_board()[tile as usize]); //copy piece to new tile
        self.update_board(tile, 0); //remove piece from current tile
        self.handle_king(landing_tile); //upgrade to king before checking for more jumps
        return self.handle_outward_jump(landing_tile);
    }

    /// Checks and executes inward jumps towards the specified tile
    #[inline]
    fn handle_inward_jump(&mut self, landing_tile: u8) -> Result<CheckersResult, CheckersError> {
        let mut possible_moves = [false; 4];
        let mut directions: [u8; 4] = [0; 4];
        if landing_tile % 2 == 0 {
            possible_moves[0] = if landing_tile < 9  {false} else {directions[0] = landing_tile - 9; self.is_move_valid(directions[0], &Action::JumpSoutheast)};
            possible_moves[1] = if landing_tile < 7  {false} else {directions[1] = landing_tile - 7; self.is_move_valid(directions[1], &Action::JumpSouthwest)};
            possible_moves[2] = if landing_tile < 1  {false} else {directions[2] = landing_tile - 1; self.is_move_valid(directions[2], &Action::JumpNortheast)};
            possible_moves[3] = if landing_tile > 30 {false} else {directions[3] = landing_tile + 1; self.is_move_valid(directions[3], &Action::JumpNorthwest)};
        } else {
            possible_moves[0] = if landing_tile < 1  {false} else {directions[0] = landing_tile - 1; self.is_move_valid(directions[0], &Action::JumpSoutheast)};
            possible_moves[1] = if landing_tile > 30 {false} else {directions[1] = landing_tile + 1; self.is_move_valid(directions[1], &Action::JumpSouthwest)};
            possible_moves[2] = if landing_tile > 24 {false} else {directions[2] = landing_tile + 7; self.is_move_valid(directions[2], &Action::JumpNortheast)};
            possible_moves[3] = if landing_tile > 22 {false} else {directions[3] = landing_tile + 9; self.is_move_valid(directions[3], &Action::JumpNorthwest)};
        }

        // get indices of trues
        let valid_bools: Vec<usize> = possible_moves.iter().enumerate().filter_map(|(i,v)| v.then_some(i)).collect();
        
        if valid_bools.len() != 1 {
            return Ok(CheckersResult::Ok(self.current_player)); //ask the AI to make the next move
        }
        // Find cool way of skipping this code when playing against a player? Like [Debug] attr in C# won't run in release.
        // we know we need to execute an automatic move
        match valid_bools[0] {
            0 => return self.make_move(directions[0], Action::JumpSoutheast), //moving from northwest to southeast
            1 => return self.make_move(directions[1], Action::JumpSouthwest), //moving from northeast to southwest
            2 => return self.make_move(directions[2], Action::JumpNortheast), //moving from southwest to northeast
            3 => return self.make_move(directions[3], Action::JumpNorthwest), //moving from southeast to northwest
            _ => unreachable!()
        }
    }

    // Checks and executes outward jumps for the specified tile.
    #[inline]
    fn handle_outward_jump(&mut self, landing_tile: u8) -> Result<CheckersResult, CheckersError>{
        let mut possible_moves = [false; 4];
        possible_moves[0] = self.is_move_valid(landing_tile, &Action::JumpNorthwest);
        possible_moves[1] = self.is_move_valid(landing_tile, &Action::JumpNortheast);
        possible_moves[2] = self.is_move_valid(landing_tile, &Action::JumpSouthwest);
        possible_moves[3] = self.is_move_valid(landing_tile, &Action::JumpSoutheast);

        let valid_bools: Vec<usize> = possible_moves.iter().enumerate().filter_map(|(i,v)| v.then_some(i)).collect();

        if valid_bools.len() == 0 {
            // turn is over, check automatic moves for enemy
            self.current_player = !self.current_player; //
            return self.handle_inward_jump(31 - landing_tile); //handle inward jumping
        }
        
        if valid_bools.len() > 1 {
            return Ok(CheckersResult::Ok(self.current_player)); //ask the AI to make the next move
        }
        
        // we know we need to execute an automatic move
        match valid_bools[0] {
            0 => return self.make_move(landing_tile, Action::JumpNorthwest), //moving from southeast to northwest
            1 => return self.make_move(landing_tile, Action::JumpNortheast), //moving from southwest to northeast
            2 => return self.make_move(landing_tile, Action::JumpSouthwest), //moving from northeast to southwest
            3 => return self.make_move(landing_tile, Action::JumpSoutheast), //moving from northwest to southeast
            _ => unreachable!()
        }
    }

    /// Decides whether or not to king a piece
    pub fn handle_king(&mut self, tile: u8) {
        let board = if self.current_player {self.board_red} else {self.board_black};
        if tile < 7 && tile % 2 == 0 && board[tile as usize] == 1 {
            self.update_board(tile, 2);
        }
    }

    /// Get a reference to the board for red
    pub fn peek_red(&self) -> &[i8; 32] {
        &self.board_red
    }
    /// Get a reference to the board for black
    pub fn peek_black(&self) -> &[i8; 32] {
        &self.board_black
    }

    /// Get a copy of the board for red
    pub fn peek_red_python(&self) -> [i8; 32] {
        self.board_red.clone()
    }
    /// Get a copy of the board for black
    pub fn peek_black_python(&self) -> [i8; 32] {
        self.board_black.clone()
    }

    /// Get the current board as immutable
    #[inline]
    pub fn get_board(&self) -> &[i8; 32] {
        if self.current_player {&self.board_red} else {&self.board_black} //retrieve the board
    }

    /// Updates both boards at the same time
    pub fn update_board(&mut self, tile: u8, value: i8) {
        let board_main;
        let board_secondary;
        if self.current_player {
            board_main = &mut self.board_red;
            board_secondary = &mut self.board_black;
        } else {
            board_main = &mut self.board_black;
            board_secondary = &mut self.board_red;
        }
        board_main[tile as usize] = value;
        board_secondary[31 - tile as usize] = 0 - value;
    }
}


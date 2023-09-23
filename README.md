See `src/bin/console_game.rs` and `src/bin/random_game.rs` for example code.

All moves are associated with only one piece, one square of departure, and one destination square. Castling is considered a king move.

Draw offers, resignation, the 50 move rule, and the threefold repetition rule are not implemented.

## Definitions

### `struct Game`
Represents a game of chess.

#### `Game::new() -> Self`
Create a new game of chess with the default starting position, and white to move.

#### `Game::from_board(board: Board) -> Self`
Create a new game from the given starting board.

#### `Game::player(self: & Self) -> Player`
Returns the player who was the turn.

#### `Game::board(self: & Self) -> Board`
Returns the current board state.

#### `Game::board_at(self: & Self, halfmove: i32) -> Board`
Returns the state that the board was in after the given halfmove. At `0` the board is in the initial state.

#### `Game::get_moves(self: & Self, from: Option<Loc>, to: Option<Loc>) -> Vec<Move>`
Returns all legal moves on the board for the player who has the turn. Optionally, only the moves that have `from` as the square of departure, and/or `to` as the destination square are returned.

#### `Game::play_move(self: & Self, move: & Move)`
Play the given move, updating the board state and passing the turn.

#### `Game::halfmove(self: & Self) -> i32`
Returns the number of halfmoves (`struct Move`'s) played.

#### `Game::fullmove(self: & Self) -> i32`
Returns the number of the current fullmove. Starts at `1`, and increments every time it's white's turn to play.

#### `Game::state(self: & Self) -> State`
Returns the state of the game, see `State`.

#### `Game::score(self: & Self) -> [i32; 2]`
Returns an array containing the score of each player. White's score is at index `0` and black's score is at index `1`. The score is measured in number of half points. When the game is in progress the score is `[0, 0]`. A checkmate results in a score of `[2, 0]` or `[0, 2]`, and a stalemate results in `[1, 1]`.

#### `Game::movetext(self: & Self) -> String`
Returns a string containing the movetext (as found in PGN) of the game.

---

### `enum State`
Represents the state of a chess game.

#### `State::Playing`
The game is in progress.

#### `State::Checkmate`
The game has ended by checkmate.

#### `State::Stalemate`
The game has ended by stalemate.

---

### `struct Loc`
Represents a location on the chess board. Implements `Iterator`, can be used to iterate over all board locations.

#### `Loc::x: i32`
The file (horizontal coordinate) of the location. Zero-indexed, starting at file a.

#### `Loc::y: i32`
The rank (vertical coordinate) of the location. Zero indexed.

#### `Loc::valid(self: & Self) -> bool`
Returns true iff the location is valid, i.e. it represents a location that exists on a chess board.

#### `Loc::offset(self: & Self, offset: (i32, i32)) -> Loc`
Returns a new location that represents the current location offset by `offset.0` in the x-direction and `offset.1` in the y-direction.

#### `Loc::notation(self: & Self, bool file, bool rank) -> String`
Returns a string representation of the location, such as "a1" or "e4". `file` and `rank` specify which components to include.

#### `Loc::parse(s: & str) -> Option<Self>`
Parses the specified string slice containing a string representation of a location. Returns the parsed location if successful.

---

### `enum Player`
Represents a player.

#### `Player::White`
The player who plays the white pieces.

#### `Player::Black`
The player who plays the black pieces.

#### `Player::opponent(self: & Self) -> Self`
Returns the players opponent.

---

### `enum Square`
Represents a square on the board.

#### `Square::Empty`
An empty square.

#### `Square::Occupied(Piece)`
A square occupied by a piece.

#### `Square::empty(self: & Self)`
Returns true iff the square is `Square::Empty`.

#### `Square::occupied(self: & Self)`
Returns true iff the square is `Square::Occupied(Piece)`.

---

### `struct Piece`
Represents a piece.

#### `Piece::player: Player`
The color of the piece.

#### `Piece::kind: &'static PieceKind`
A reference to a `struct PieceKind` that specifies the piece's kind.

#### `Piece::is_player(self: & Self, player: Player) -> bool`
Returns true iff the piece belongs to the given player.

#### `Piece::is_kind(self: & Self, kind: & PieceKind) -> bool`
Return true iff the piece is of the given kind.

#### `Piece::is(self: & Self, player: Player, kind: & PieceKind) -> bool`
Returns true iff the piece belongs to the given player and is of the given kind.

#### `Piece::moves(self: & Self, board: & Board, loc: Loc) -> Vec<Move>`
Returns all possible moves for the piece on the given board and location, according to the movement rules for that kind of piece. Includes moves that are not actually legal on the given board (i.e. self-checks). Wrapper around `PieceKind::moves_fn`.

---

### `struct PieceKind`
Represents a kind of piece by its name and how it's allowed to move.

#### `PieceKind::name: &'static str`
A string slice containing the name (typically a single letter) of the piece kind, as used in various chess notations.

#### `PieceKind::moves_fn: fn(board: & Board, piece: & Piece, loc: Loc) -> Vec<Move>`
A function returning all possible moves for a piece on the given board and location, according to the movement rules for that particular `PieceKind`. Includes moves that are not actually legal on the given board (i.e. self-checks).

The following piece kinds are defined:
- `static KING: PieceKind`
- `static QUEEN: PieceKind`
- `static ROOK: PieceKind`
- `static BISHOP: PieceKind`
- `static KNIGHT: PieceKind`
- `static PAWN: PieceKind`

---

### `struct Castling`
Represents the castling rights for a player.

#### `Castling::k: Option<Loc>`
Contains the location of a king-side rook iff the player is allowed to castle king-side.

#### `Castling::q: Option<Loc>`
Contains the location of a queen-side rook iff the player is allowed to castle queen-side.

---

### `struct Board`
Represents a chess board.

#### `Board::default() -> Self`
Returns the default starting board of a chess game.

#### `Board::from_fen(fen: & str) -> Option<Self>`
Parses a board the given FEN string and returns it if successful.

#### `Board::fen(self: & Self) -> String`
Returns a string with the FEN representation of the board state.

#### `Board::player: Player`
The player who has the turn on the board.

#### `Board::passant: Option<Loc>`
Contains the target square for a passant move, if one exists.

#### `Board::at(self: & Self, loc: Loc) -> & Square`
Returns a reference to the square at the given location on the board.

#### `Board::at_mut(self: & mut Self, loc: Loc) -> & mut Square`
Returns a mutable reference to the square at the given location on the board.

#### `Board::locations(self: & Self) -> Loc`
Returns the location of the first square on the board, used to iterate over all squares.

#### `Board::castling(self: & Self, player: Player) -> & Castling`
Returns a reference to the castling rights for the given player on the board.

#### `Board::castling_mut(self: & mut Self, player: Player) -> & mut Castling`
Returns a mutable reference to the castling rights for the given player on the board.

#### `Board::is_check(self: & Self, player: Player) -> bool`
Returns true iff the given player is in check.

#### `Board::moves(self: & Self, player: Player) -> Vec<Move>`
Returns all legal moves for the given player (disregarding the turn).

---

### `struct Move`
Represents a chess move (halfmove or ply in chess parlance).

#### `Move::board: Board`
Contains the board that results from the move.

#### `Move::piece: Piece`
Contains the primary piece that is moved.

#### `Move::from: Loc`
The location of the square of departure.

#### `Move::to: Loc`
The location of the destination square.

#### `Move::is_check(self: & Self) -> bool`
Returns true iff the move is a checking move.

#### `Move::is_castle(self: & Self) -> bool`
Returns true iff the move is a castling move.

#### `Move::is_passant(self: & Self) -> bool`
Returns true iff the move is an en passant move.

#### `Move::is_capture(self: & Self) -> bool`
Returns true iff the move is a capturing move.

#### `Move::is_promotion(self: & Self) -> Option<& PieceKind>`
If the move is promoting move, returns a `Some(& PieceKind)` containing a reference to the `PieceKind` that the move promotes to, otherwise `None`.

#### `Move::notation_fn: fn(r#move: & Move, board: & Board, disambiguate: & Vec<Move>) -> String`
A function returning a string containing the algebraic notation for the given move, when played on the given board. `disambiguate` contains all moves (possibly including the given move) from which the move must be disambiguated.

#### `Move::notation(self: & Self, board: & Board, disambiguate: & Vec<Move>) -> String`
Returns a string containing the algebraic notation for the move. Wrapper call around `self.notation_fn`.

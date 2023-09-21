use board::*;
use r#move::*;
use player::*;

#[derive(Copy, Clone)]
pub struct PieceKind
{
	pub moves_fn	: fn(board: & Board, piece: & Piece, loc: Loc)
				-> Vec<Move>,
	pub name	: &'static str,
}

#[derive(Copy, Clone)]
pub struct Piece
{
	pub player	: Player,
	pub kind	: &'static PieceKind,
}

impl Piece
{
	pub fn is_player(self: & Self, player: Player) -> bool
	{
		self.player == player
	}

	pub fn is_kind(self: & Self, kind: & PieceKind) -> bool
	{
		(self.kind as *const _) == (kind as *const _)
	}

	pub fn is(self: & Self, player: Player, kind: & PieceKind) -> bool
	{
		self.is_player(player) && self.is_kind(kind)
	}

	pub fn moves(self: & Self, board: & Board, loc: Loc) -> Vec<Move>
	{
		(self.kind.moves_fn)(board, self, loc)
	}
}

pub static KING: PieceKind = PieceKind
{
	moves_fn	: king_moves,
	name		: "K",
};

pub static QUEEN: PieceKind = PieceKind
{
	moves_fn	: queen_moves,
	name		: "Q",
};

pub static ROOK: PieceKind = PieceKind
{
	moves_fn	: cardinal_moves,
	name		: "R",
};

pub static BISHOP: PieceKind = PieceKind
{
	moves_fn	: diagonal_moves,
	name		: "B",
};

pub static KNIGHT: PieceKind = PieceKind
{
	moves_fn	: knight_moves,
	name		: "N",
};

pub static PAWN: PieceKind = PieceKind
{
	moves_fn	: pawn_moves,
	name		: "P",
};


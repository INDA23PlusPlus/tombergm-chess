use board::*;
use piece::*;
use player::*;
use move_notation::*;

#[derive(Copy, Clone)]
pub struct MoveFlags
{
	pub capture	: bool,
	pub passant	: bool,
	pub castle	: bool,
	pub promotion	: bool,
}

#[derive(Copy, Clone)]
pub struct Move
{
	pub board	: Board,
	pub piece	: Piece,
	pub from	: Loc,
	pub to		: Loc,
	pub notation_fn	: fn(r#move: & Move, board: & Board,
				disambiguate: & Vec<Move>) -> String,
	pub flags	: MoveFlags,
}

impl Move
{
	pub fn is_check(self: & Self) -> bool
	{
		self.board.is_check(self.board.player)
	}

	pub fn is_castle(self: & Self) -> bool
	{
		self.flags.castle
	}

	pub fn is_passant(self: & Self) -> bool
	{
		self.flags.passant
	}

	pub fn is_capture(self: & Self) -> bool
	{
		self.flags.capture
	}

	pub fn is_promotion(self: & Self) -> Option<& PieceKind>
	{
		if !self.flags.promotion
		{
			None
		}
		else if let Square::Occupied(q) = self.board.at(self.to)
		{
			if q.is_kind(self.piece.kind)
			{
				None
			}
			else
			{
				Some(q.kind)
			}
		}
		else
		{
			None
		}
	}

	pub fn notation(self: & Self, board: & Board,
			disambiguate: & Vec<Move>) -> String
	{
		(self.notation_fn)(self, board, disambiguate)
	}
}

/* Default move (chess move that is) constructor. Create a single move where
 * the given piece on the given board moves from 'from' to 'to'. */
pub fn single_move(b: & Board, p: & Piece, from: Loc, to: Loc) -> Move
{
	let mut mb = *b;

	*mb.at_mut(from) = Square::Empty;
	*mb.at_mut(to) = Square::Occupied(*p);

	/* Pass the turn */
	mb.player = mb.player.opponent();

	/* Clear the passant square, moves that allow passant should set this
	 * in the returned move. */
	mb.passant = None;

	/* Check for moves to or from castling squares, castling rights for
	 * that square is removed. */
	for loc in [from, to]
	{
		if Some(loc) == mb.castling(Player::White).k
		{
			mb.castling_mut(Player::White).k = None;
		}
		if Some(loc) == mb.castling(Player::White).q
		{
			mb.castling_mut(Player::White).q = None;
		}
		if Some(loc) == mb.castling(Player::Black).k
		{
			mb.castling_mut(Player::Black).k = None;
		}
		if Some(loc) == mb.castling(Player::Black).q
		{
			mb.castling_mut(Player::Black).q = None;
		}
	}

	let capture = matches!(b.at(to), Square::Occupied(q)
				if q.player != p.player);

	Move
	{
		board		: mb,
		piece		: *p,
		from,
		to,
		notation_fn	: default_move_notation,
		flags		: MoveFlags
		{
			capture,
			passant		: false,
			castle		: false,
			promotion	: false,
		},
	}
}

pub fn single_to_promo(m: & Move) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	let kinds =
	[
		& QUEEN,
		& ROOK,
		& BISHOP,
		& KNIGHT,
	];

	for k in kinds
	{
		let mut n = *m;

		if let Square::Occupied(p) = n.board.at_mut(n.to)
		{
			p.kind = k;
			n.flags.promotion = true;
		}

		ms.push(n);
	}

	ms
}

pub fn multi_to_promo(ms: & Vec<Move>) -> Vec<Move>
{
	let mut ns = Vec::<Move>::new();

	for m in ms
	{
		ns.extend(single_to_promo(m));
	}

	ns
}

pub fn directional_moves(b: & Board, p: & Piece, from: Loc, dir: (i32, i32))
	-> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	let mut to = from.offset(dir);

	while to.valid()
	{
		let m = single_move(b, p, from, to);

		match b.at(to)
		{
			Square::Occupied(q) =>
			{
				if p.player == q.player.opponent()
				{
					ms.push(m);
				}

				break;
			}
			Square::Empty => ms.push(m),
		}

		to = to.offset(dir);
	}

	ms
}

pub fn diagonal_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in [(-1,  1), (-1, -1), ( 1,  1), ( 1, -1)]
	{
		ms.extend(directional_moves(b, p, loc, dir));
	}

	ms
}

pub fn cardinal_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in [(-1,  0), ( 0,  1), ( 0, -1), ( 1,  0)]
	{
		ms.extend(directional_moves(b, p, loc, dir));
	}

	ms
}

pub fn queen_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	ms.extend(diagonal_moves(b, p, loc));
	ms.extend(cardinal_moves(b, p, loc));

	ms
}

pub fn check_castle(b: & Board, p: & Piece, loc: Loc, rook_loc: Option<Loc>,
			dir: i32)
	-> Option<Loc>
{
	/* Stop if the player doesn't have castling rights */
	if let None = rook_loc
	{
		return None;
	}

	{
		let mut to = loc.offset((dir, 0));

		/* Look for a rook connected to the king to castle with */
		while to.valid()
		{
			if let Square::Occupied(q) = b.at(to)
			{
				if q.is(p.player, & ROOK)
				{
					break;
				}
				else
				{
					return None;
				}
			}

			to = to.offset((dir, 0));
		}

		/* The rook must be on the castling square, castling with a
		 * rook that just happens to be next to the king is not
		 * allowed. */
		if Some(to) != rook_loc
		{
			return None;
		}
	}

	/* Check whether the king is clear to cross. The king always moves two
	 * squares. */
	for x in 0..3
	{
		let to = loc.offset((x * dir, 0));

		/* Stop if castling would move out of the board */
		if !to.valid()
		{
			return None;
		}

		/* Stop if a crossing square is occupied by a non-castling
		 * piece */
		if to != loc && Some(to) != rook_loc && b.at(to).occupied()
		{
			return None;
		}

		/* Stop if the king would move across a check */
		if single_move(b, p, loc, to).board.is_check(p.player)
		{
			return None;
		}
	}

	rook_loc
}

pub fn king_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	/* Check normal moves */
	for dir in
		[
			(-1,  1), ( 0,  1), ( 1,  1),
			(-1,  0),           ( 1,  0),
			(-1, -1), ( 0, -1), ( 1, -1),
		]
	{
		let to = loc.offset(dir);

		if to.valid()
		{
			let m = single_move(b, p, loc, to);

			match b.at(to)
			{
				Square::Occupied(q) =>
				{
					if p.player == q.player.opponent()
					{
						ms.push(m);
					}
				},
				Square::Empty => ms.push(m),
			}
		}
	}

	/* Check castling moves */
	for c in
		[
			(b.castling(p.player).k,  1,
				castle_k_notation
				as fn(& Move, & Board, & Vec<Move>) -> String),
			(b.castling(p.player).q, -1,
				castle_q_notation
				as fn(& Move, & Board, & Vec<Move>) -> String),
		]
	{
		if let Some(rook_loc) = check_castle(b, p, loc, c.0, c.1)
		{
			let to = loc.offset((2 * c.1, 0));
			let rook_to = to.offset((-c.1, 0));

			let mut m = single_move(b, p, loc, to);

			*m.board.at_mut(rook_to) = *m.board.at(rook_loc);
			*m.board.at_mut(rook_loc) = Square::Empty;
	
			m.notation_fn = c.2;

			m.flags.castle = true;

			ms.push(m);
		}
	}

	/* Moving the king gives up all of the player's castling rights */
	for m in & mut ms
	{
		*m.board.castling_mut(p.player) = Castling
		{
			k: None,
			q: None,
		};
	}

	ms
}

pub fn knight_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in
		[
			(-2,  1), (-1,  2), ( 1,  2), ( 2,  1),
			(-2, -1), (-1, -2), ( 1, -2), ( 2, -1),
		]
	{
		let to = loc.offset(dir);

		if to.valid()
		{
			let m = single_move(b, p, loc, to);

			match b.at(to)
			{
				Square::Occupied(q) =>
				{
					if p.player == q.player.opponent()
					{
						ms.push(m);
					}
				},
				Square::Empty => ms.push(m),
			}
		}
	}

	ms
}

pub fn pawn_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	/* Decide starting rank, promotion rank, and move direction, depending
	 * on the player. */
	let (start, promo, dir) = match p.player
	{
		Player::White => ( 1,  6,  1),
		Player::Black => ( 6,  1, -1),
	};

	let step_loc = loc.offset((0, dir));
	if step_loc.valid() && b.at(step_loc).empty()
	{
		ms.push(single_move(b, p, loc, step_loc));

		let jump_loc = step_loc.offset((0, dir));
		if jump_loc.valid() && loc.y == start && b.at(jump_loc).empty()
		{
			let mut m = single_move(b, p, loc, jump_loc);
			m.board.passant = Some(step_loc);
			ms.push(m);
		}
	}

	/* Compute capturing move locations */
	let capt_locs = [loc.offset((-1, dir)), loc.offset((1, dir))];
	for to in capt_locs
	{
		if to.valid()
		{
			if let Square::Occupied(q) = b.at(to)
			{
				if q.player == p.player.opponent()
				{
					ms.push(single_move(b, p, loc, to));
				}
			}
			else if Some(to) == b.passant
			{
				let mut m = single_move(b, p, loc, to);

				let s = m.board.at_mut(to.offset((0, -dir)));
				*s = Square::Empty;

				m.flags.capture = true;
				m.flags.passant = true;

				ms.push(m);
			}
		}
	}

	/* If the pawn is on the next to last rank, convert all move to their
	 * promotion variants. */
	if loc.y == promo
	{
		ms = multi_to_promo(& ms);
	}

	/* Change the notation function to the special pawn notation function
	 */
	for m in & mut ms
	{
		m.notation_fn = pawn_move_notation;
	}

	ms
}


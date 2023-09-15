#[derive(Copy, Clone, PartialEq)]
pub enum Player
{
	White,
	Black,
}

impl Player
{
	pub fn opponent(self: & Self) -> Self
	{
		match self
		{
			Self::White => Self::Black,
			Self::Black => Self::White,
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Loc
{
	pub x	: i32,
	pub y	: i32,
}

#[derive(enumset::EnumSetType)]
pub enum MoveFlags
{
	Capture,
	Check,
	Passant,
	Promote,
	LongCastle,
	ShortCastle,
}

#[derive(Copy, Clone)]
pub struct Move
{
	pub board	: Board,
	pub piece	: Piece,
	pub from	: Loc,
	pub to		: Loc,
	pub flags	: enumset::EnumSet<MoveFlags>,
}

#[derive(Copy, Clone)]
pub struct PieceKind
{
	pub moves	: fn(& Board, & Piece, Loc) -> Vec<Move>,
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
}

#[derive(Copy, Clone)]
pub enum Square
{
	Empty,
	Occupied(Piece),
}

impl Square
{
	pub fn empty(self: & Self) -> bool
	{
		match self
		{
			Self::Empty => true,
			Self::Occupied(_) => false,
		}
	}

	pub fn occupied(self: & Self) -> bool
	{
		match self
		{
			Self::Empty => false,
			Self::Occupied(_) => true,
		}
	}
}

#[derive(enumset::EnumSetType)]
pub enum Castling
{
	Long,
	Short,
}

#[derive(Copy, Clone)]
pub struct Board
{
	pub squares	: [Square; 8 * 8],
	pub passant	: Option<Loc>,
	pub castling_w	: enumset::EnumSet<Castling>,
	pub castling_b	: enumset::EnumSet<Castling>,
}

impl Iterator for Loc
{
	type Item = Loc;

	fn next (& mut self) -> Option<Loc>
	{
		if self.valid()
		{
			let n = *self;

			*self = Loc
			{
				x: (self.x + 1) % 8,
				y: self.y + (self.x + 1) / 8,
			};

			Some(n)
		}
		else
		{
			None
		}
	}
}

impl Loc
{
	pub fn valid(self: & Self) -> bool
	{
		match (self.x, self.y)
		{
			(0..=7, 0..=7) => true,
			_ => false,
		}
	}

	pub fn offset(self: & Self, offset: (i32, i32)) -> Loc
	{
		Loc
		{
			x: self.x + offset.0,
			y: self.y + offset.1
		}
	}

	pub fn notation(self: & Self) -> String
	{
		let f = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
			[self.x as usize];
		let r = self.y + 1;
		format!("{}{}", f, r)
	}
}

/* Create a single move where the given piece on the given board moves from
 * 'from' to 'to'. */
fn single_move(b: & Board, p: & Piece, from: Loc, to: Loc) -> Move
{
	let mut mb = *b;

	*mb.at_mut(from) = Square::Empty;
	*mb.at_mut(to) = Square::Occupied(*p);

	let mut m = Move
	{
		board	: mb,
		piece	: *p,
		from,
		to,
		flags	: enumset::EnumSet::new(),
	};

	m.board.passant = None;

	m
}

fn single_to_promo(m: & Move) -> Vec<Move>
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
		}

		ms.push(n);
	}

	ms
}

fn multi_to_promo(ms: & Vec<Move>) -> Vec<Move>
{
	let mut ns = Vec::<Move>::new();

	for m in ms
	{
		ns.extend(single_to_promo(m));
	}

	ns
}

fn directional_moves(b: & Board, p: & Piece, from: Loc, dir: (i32, i32))
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

	return ms;
}

fn diagonal_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in [(1, 1), (1, -1), (-1, 1), (-1, -1)]
	{
		ms.extend(directional_moves(b, p, loc, dir));
	}

	return ms;
}

fn cardinal_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)]
	{
		ms.extend(directional_moves(b, p, loc, dir));
	}

	return ms;
}

fn queen_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	ms.extend(diagonal_moves(b, p, loc));
	ms.extend(cardinal_moves(b, p, loc));

	return ms;
}

pub fn king_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in
		[
			(-1, -1), ( 0, -1), ( 1, -1),
			(-1,  0),           ( 1,  0),
			(-1,  1), ( 0,  1), ( 1,  1),
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

	return ms;
}

pub fn knight_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	for dir in
		[
			( 2, 1), ( 2, -1),
			( 1, 2), ( 1, -2),
			(-1, 2), (-1, -2),
			(-2, 1), (-2, -1)
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

	return ms;
}

pub fn pawn_moves(b: & Board, p: & Piece, loc: Loc) -> Vec<Move>
{
	let mut ms = Vec::<Move>::new();

	/* Decide starting rank, promotion rank, and move direction, depending
	 * on the player. */
	let (start, promo, dir) = match p.player
	{
		Player::White => (1, 6,  1),
		Player::Black => (6, 1, -1),
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
	let capt_locs = vec![loc.offset((-1, dir)), loc.offset((1, dir))];
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
			}
		}
	}

	if loc.y == promo
	{
		ms = multi_to_promo(& ms);
	}

	ms
}

pub static KING: PieceKind = PieceKind
{
	moves	: king_moves,
	name	: "K",
};

pub static QUEEN: PieceKind = PieceKind
{
	moves	: queen_moves,
	name	: "Q",
};

pub static ROOK: PieceKind = PieceKind
{
	moves	: cardinal_moves,
	name	: "R",
};

pub static BISHOP: PieceKind = PieceKind
{
	moves	: diagonal_moves,
	name	: "B",
};

pub static KNIGHT: PieceKind = PieceKind
{
	moves	: knight_moves,
	name	: "N",
};

pub static PAWN: PieceKind = PieceKind
{
	moves	: pawn_moves,
	name	: "",
};

impl Board
{
	pub fn at(self: & Self, loc: Loc) -> & Square
	{
		& self.squares[(loc.y * 8 + loc.x) as usize]
	}

	pub fn at_mut(self: & mut Self, loc: Loc) -> & mut Square
	{
		& mut self.squares[(loc.y * 8 + loc.x) as usize]
	}

	pub fn locations(self: & Self) -> Loc
	{
		Loc { x: 0, y: 0 }
	}

	/* Enumerate all possible moves according to the rules of piece_moves
	 * movement, including illegal moves (self-checks). */
	pub fn piece_moves(self: & Self) -> Vec::<Move>
	{
		let mut ms = Vec::<Move>::new();

		for loc in self.locations()
		{
			match self.at(loc)
			{
				Square::Occupied(p)
				=> ms.extend((p.kind.moves)(self, p, loc)),
				_ => (),
			}
		}

		ms
	}

	/* Check whether the board state is check for the given player's king
	 */
	pub fn is_check(self: & Self, pl: Player) -> bool
	{
		for m in self.piece_moves()
		{
			if m.piece.player != pl.opponent()
			{
				continue;
			}

			if let Square::Occupied(p) = self.at(m.to)
			{
				if p.is(pl, & KING)
				{
					return true;
				}
			}
		}

		false
	}

	/* Enumerate all legal moves */
	pub fn moves(self: & Self, pl: Player) -> Vec<Move>
	{
		let mut ms = self.piece_moves();

		/* Keep only legal moves, i.e. moves that do not put the moving
		 * player's king in check. */
		ms.retain(|m| m.piece.player == pl && !m.board.is_check(pl));
		
		ms
	}

	pub fn default() -> Board
	{

		/* Start with an empty board */
		let mut b = Board
		{
			squares		: [Square::Empty; 8 * 8],
			passant		: None,
			castling_w	: enumset::enum_set!
					(
						Castling::Long |
						Castling::Short
					),
			castling_b	: enumset::enum_set!
					(
						Castling::Long |
						Castling::Short
					),
		};

		/* Piece layout of the 1st and 8th rank */
		let k =
		[
			& ROOK,
			& KNIGHT,
			& BISHOP,
			& QUEEN,
			& KING,
			& BISHOP,
			& KNIGHT,
			& ROOK,
		];

		for x in 0..8
		{
			/* White pieces */
			*b.at_mut(Loc {x, y: 0}) = Square::Occupied
			(
				Piece
				{
					player	: Player::White,
					kind	: k[x as usize],
				},
			);
			*b.at_mut(Loc {x, y: 1}) = Square::Occupied
			(
				Piece
				{
					player	: Player::White,
					kind	: & PAWN,
				},
			);

			/* Black pieces */
			*b.at_mut(Loc {x, y: 7}) = Square::Occupied
			(
				Piece
				{
					player	: Player::Black,
					kind	: k[x as usize],
				},
			);
			*b.at_mut(Loc {x, y: 6}) = Square::Occupied
			(
				Piece
				{
					player	: Player::Black,
					kind	: & PAWN,
				},
			);
		}

		b
	}
}


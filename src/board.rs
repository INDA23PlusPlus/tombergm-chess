use crate::fen::*;
use crate::r#move::*;
use crate::piece::*;
use crate::player::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Loc
{
	pub x	: i32,
	pub y	: i32,
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

	pub fn notation(self: & Self, file: bool, rank: bool) -> String
	{
		let mut s = String::new();

		if file
		{
			s.push(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
				[self.x as usize]);
		}

		if rank
		{
			s.push_str(format!("{}", self.y + 1).as_str());
		}

		s
	}

	pub fn parse(s: & str) -> Option<Self>
	{
		if s.len() != 2
		{
			return None;
		}

		let x = match s.chars().nth(0).unwrap()
		{
			'a' => 0,
			'b' => 1,
			'c' => 2,
			'd' => 3,
			'e' => 4,
			'f' => 5,
			'g' => 6,
			'h' => 7,
			_   => return None,
		};

		let y = match s.chars().nth(1).unwrap()
		{
			'1' => 0,
			'2' => 1,
			'3' => 2,
			'4' => 3,
			'5' => 4,
			'6' => 5,
			'7' => 6,
			'8' => 7,
			_   => return None,
		};

		Some(Loc { x, y })
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

#[derive(Copy, Clone)]
pub struct Castling
{
	pub k	: Option<Loc>,
	pub q	: Option<Loc>,
}

impl Castling
{
	pub const DEFAULT: [Self; 2] =
	[
		Self
		{
			k: Some(Loc { x: 7, y: 0 }),
			q: Some(Loc { x: 0, y: 0 }),
		},
		Self
		{
			k: Some(Loc { x: 7, y: 7 }),
			q: Some(Loc { x: 0, y: 7 }),
		},
	];
}

#[derive(Copy, Clone)]
pub struct Board
{
	pub player	: Player,
	pub squares	: [Square; 8 * 8],
	pub passant	: Option<Loc>,
	pub castling	: [Castling; 2],
}

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

	pub fn castling(self: & Self, player: Player) -> & Castling
	{
		& self.castling[player as usize]
	}

	pub fn castling_mut(self: & mut Self, player: Player) -> & mut Castling
	{
		& mut self.castling[player as usize]
	}

	/* Enumerate all possible moves according to the rules of piece
	 * movement, including illegal moves (self-checks). */
	fn piece_moves(self: & Self, player: Player) -> Vec::<Move>
	{
		let mut ms = Vec::<Move>::new();

		for loc in self.locations()
		{
			match self.at(loc)
			{
				Square::Occupied(p) if p.player == player
					=> ms.extend(p.moves(self, loc)),
				_ => (),
			}
		}

		ms
	}

	/* Check whether the board state is check for the given player's king
	 */
	pub fn is_check(self: & Self, player: Player) -> bool
	{
		let mut b = *self;

		/* Disregard castling moves as they cannot capture anything */
		/* This is necessary to prevent infinite recursion when
		 * check castling availability */
		b.castling =  [ Castling { k: None, q: None }; 2 ];

		for m in b.piece_moves(player.opponent())
		{
			if let Square::Occupied(p) = b.at(m.to)
			{
				if p.is(player, & KING)
				{
					return true;
				}
			}
		}

		false
	}

	/* Enumerate all legal moves */
	pub fn moves(self: & Self, player: Player) -> Vec<Move>
	{
		let mut ms = self.piece_moves(player);

		/* Keep only legal moves, i.e. moves that do not put the moving
		 * player's king in check. */
		ms.retain(|m| !m.board.is_check(player));

		ms
	}

	pub fn default() -> Self
	{
		/* Start with an empty board */
		let mut b = Self
		{
			player		: Player::White,
			squares		: [Square::Empty; 8 * 8],
			passant		: None,
			castling	: Castling::DEFAULT,
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

	pub fn from_fen(fen: & str) -> Option<Self>
	{
		parse_fen(fen)
	}

	pub fn fen(self: & Self) -> String
	{
		make_fen(self)
	}
}

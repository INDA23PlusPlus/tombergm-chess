use board::*;
use piece::*;
use player::*;

macro_rules! pl
{
	(w) => { Player::White };
	(b) => { Player::Black };
}

macro_rules! pk
{
	(k) => { & KING };
	(q) => { & QUEEN };
	(r) => { & ROOK };
	(b) => { & BISHOP };
	(n) => { & KNIGHT };
	(p) => { & PAWN };
}

macro_rules! pc
{
	($pl:ident $pk:ident) =>
	{
		Piece { player: pl!($pl), kind : pk!($pk) }
	};
}

macro_rules! sq
{
	($pl:ident $pk:ident) => { Square::Occupied(pc!($pl $pk)) };
	() => { Square::Empty };
}

fn parse_piece(c: char) -> Option<Vec<Square>>
{
	match c
	{
		'K' => Some(vec![sq!(w k)]),
		'Q' => Some(vec![sq!(w q)]),
		'R' => Some(vec![sq!(w r)]),
		'B' => Some(vec![sq!(w b)]),
		'N' => Some(vec![sq!(w n)]),
		'P' => Some(vec![sq!(w p)]),
		'k' => Some(vec![sq!(b k)]),
		'q' => Some(vec![sq!(b q)]),
		'r' => Some(vec![sq!(b r)]),
		'b' => Some(vec![sq!(b b)]),
		'n' => Some(vec![sq!(b n)]),
		'p' => Some(vec![sq!(b p)]),
		'1' => Some(vec![sq!(); 1]),
		'2' => Some(vec![sq!(); 2]),
		'3' => Some(vec![sq!(); 3]),
		'4' => Some(vec![sq!(); 4]),
		'5' => Some(vec![sq!(); 5]),
		'6' => Some(vec![sq!(); 6]),
		'7' => Some(vec![sq!(); 7]),
		'8' => Some(vec![sq!(); 8]),
		_   => None,
	}
}

fn parse_rank(ci: & mut std::iter::Peekable<std::str::Chars>)
	-> Option<Vec<Square>>
{
	let mut r = Vec::<Square>::new();

	while r.len() < 8
	{
		match ci.peek()
		{
			Some(c) =>
			{
				r.extend(parse_piece(*c)?);
				ci.next();
			},
			_ => { break; },
		}
	}

	if r.len() == 8
	{
		Some(r)
	}
	else
	{
		None
	}
}

fn parse_squares(ci: & mut std::iter::Peekable<std::str::Chars>)
	-> Option<Vec<Square>>
{
	let mut s = Vec::<Square>::new();

	while s.len() < 8 * 8
	{
		let mut r = parse_rank(ci)?;
		r.append(& mut s);
		s = r;

		match ci.next()
		{
			Some('/') => continue,
			Some(' ') => break,
			_ => return None,
		}
	}

	if s.len() == 8 * 8
	{
		Some(s)
	}
	else
	{
		None
	}
}

fn parse_player(ci: & mut std::iter::Peekable<std::str::Chars>)
	-> Option<Player>
{
	let mut p = Player::White;

	if matches!(ci.peek(), Some(' ')) { return None; }

	loop
	{
		match ci.next()
		{
			Some('w') => p = Player::White,
			Some('b') => p = Player::Black,
			Some(' ') => return Some(p),
			_ => return None,
		}
	}
}

fn parse_castling(ci: & mut std::iter::Peekable<std::str::Chars>)
	-> Option<[Castling; 2]>
{
	let mut c = [Castling { k: None, q: None }; 2];

	if matches!(ci.peek(), Some(' ')) { return None; }

	if matches!(ci.peek(), Some('-')) { return Some(c); }

	loop
	{
		match ci.next()
		{
			Some('K') =>
				if let Some(_) = c[0].k { return None }
				else { c[0].k = Castling::DEFAULT[0].k },
			Some('Q') =>
				if let Some(_) = c[0].q { return None }
				else { c[0].q = Castling::DEFAULT[0].q },
			Some('k') =>
				if let Some(_) = c[1].k { return None }
				else { c[1].k = Castling::DEFAULT[1].k },
			Some('q') =>
				if let Some(_) = c[1].q { return None }
				else { c[1].q = Castling::DEFAULT[1].q },
			Some(' ') => return Some(c),
			_ => return None,
		}
	}
}

fn parse_passant(ci: & mut std::iter::Peekable<std::str::Chars>)
	-> Option<Option<Loc>>
{
	if matches!(ci.peek(), Some('-'))
	{
		ci.next();

		if matches!(ci.peek(), Some(' '))
		{
			ci.next();

			Some(None)
		}
		else
		{
			None
		}
	}
	else
	{
		let mut s = String::new();
		s.push(ci.next()?);
		s.push(ci.next()?);

		let p = Loc::parse(s.as_str())?;

		if matches!(ci.peek(), Some(' '))
		{
			ci.next();

			Some(Some(p))
		}
		else
		{
			None
		}
	}
}

pub fn parse_fen(fen: & str) -> Option<Board>
{
	let mut ci = fen.chars().peekable();

	let squares_v = parse_squares(& mut ci)?;
	let mut squares = [Square::Empty; 8 * 8];

	for i in 0..squares_v.len()
	{
		squares[i] = squares_v[i];
	}

	let player = parse_player(& mut ci)?;

	let castling = parse_castling(& mut ci)?;

	let passant = parse_passant(& mut ci)?;

	let board = Board
	{
		player,
		squares,
		passant,
		castling,
	};

	Some(board)
}


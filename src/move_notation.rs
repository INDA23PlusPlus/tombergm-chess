use board::*;
use r#move::*;

/* Check if the given move needs to be ambiguated from the given set of moves
 * by rank, file, or both. */
fn disambiguate(m: & Move, ms: & Vec<Move>) -> (bool, bool)
{
	/* Diagonal pawn moves always specify the departure file, and never
	 * need to be disambiguated further. */
	if m.piece.kind.name == "" && m.from.x != m.to.x
	{
		return (true, false);
	}

	let mut d = (false, false);

	for n in ms
	{
		/* Ignore moves that are the same as the provided move */
		if n.from.x == m.from.x
			&& n.from.y == m.from.y
			&& n.to.x == m.to.x
			&& n.to.y == n.to.y
		{
			continue;
		}

		/* Ignore moves that target a different square, or move a
		 * different kind of piece. */
		if n.to.x != m.to.x
			|| n.to.y != m.to.y
			|| !n.piece.is_kind(m.piece.kind)
		{
			continue;
		}

		/* If the two pieces are on the same file then we must
		 * disambiguate by rank. We must also do this if we find
		 * additional ambiguities after having already disambiguated
		 * by file. */
		if n.from.x == m.from.x || d.0
		{
			d.1 = true;
		}
		/* Disambiguate by file by default */
		else
		{
			d.0 = true;
		}
	}

	d
}

/* Add a + for checking moves, # for checkmating moves. */
fn add_check_suffix(mut s: String, m: & Move) -> String
{
	if m.board.is_check(m.board.player)
	{
		if m.board.moves(m.board.player).len() == 0
		{
			s.push('#');
		}
		else
		{
			s.push('+');
		}
	}

	s
}

/* Default notation function for most pieces and moves */
pub fn default_move_notation(m: & Move, b: & Board, ms: & Vec<Move>) -> String
{
	let x = match b.at(m.to)
	{
		Square::Empty => "",
		Square::Occupied(_) => "x",
	};

	let d =
	{
		let d = disambiguate(m, & ms);

		m.from.notation(d.0, d.1)
	};

	let p = match m.board.at(m.to)
	{
		Square::Occupied(p) if !p.is(m.piece.player, m.piece.kind)
			=> format!("={}", p.kind.name),
		_ => String::from(""),
	};

	let s = format!("{}{}{}{}{}",
			m.piece.kind.name,
			d,
			x,
			m.to.notation(true, true),
			p);

	add_check_suffix(s, m)
}

pub fn pawn_move_notation(m: & Move, _: & Board, _: & Vec<Move>) -> String
{
	let x = match m.is_capture()
	{
		true	=> "x",
		false	=> "",
	};

	let d = m.from.notation(m.is_capture(), false);

	let p = match m.board.at(m.to)
	{
		Square::Occupied(p) if !p.is(m.piece.player, m.piece.kind)
			=> format!("={}", p.kind.name),
		_ => String::from(""),
	};

	let s = format!("{}{}{}{}",
			d,
			x,
			m.to.notation(true, true),
			p);

	add_check_suffix(s, m)
}

pub fn castle_k_notation(m: & Move, _: & Board, _: & Vec<Move>) -> String
{
	add_check_suffix(String::from("O-O"), m)
}

pub fn castle_q_notation(m: & Move, _: & Board, _: & Vec<Move>) -> String
{
	add_check_suffix(String::from("O-O-O"), m)
}


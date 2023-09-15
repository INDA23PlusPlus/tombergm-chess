extern crate chess;

fn disambiguate(m: & chess::Move, ms: & Vec<chess::Move>) -> (bool, bool)
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
		 * disambiguate by rank. We must also do this if find
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

fn main()
{
	let mut b = chess::Board::default();
	let mut pl = chess::Player::White;
	let mut halfmove = 0;

	for i in 0..400
	{
		if i != 0
		{
			print!(" ");
		}

		let ms = b.moves(pl);
		if ms.len() == 0
		{
			if b.is_check(pl)
			{
				if pl == chess::Player::White
				{
					print!("0-1");
				}
				else
				{
					print!("1-0");
				}
			}
			else
			{
				print!("1/2-1/2");
			}

			break;
		}

		let n = rand::random::<usize>() % ms.len();
		let m = & ms[n];

		let x = match b.at(m.to)
		{
			chess::Square::Empty => "",
			chess::Square::Occupied(_) => "x",
		};

		let d =
		{
			let d = disambiguate(m, & ms);

			format!("{}{}",
				if d.0
				{
					["a", "b", "c", "d", "e", "f", "g", "h"]
					[m.from.x as usize]
				}
				else
				{
					""
				},
				if d.1
				{
					format!("{}", m.from.y + 1)
				}
				else
				{
					String::from("")
				}
			)
		};

		let c = if m.board.is_check(pl.opponent())
		{
			if m.board.moves(pl.opponent()).len() == 0
			{
				"#"
			}
			else
			{
				"+"
			}
		}
		else
		{
			""
		};

		let p = match m.board.at(m.to)
		{
			chess::Square::Occupied(p)
				if !p.is(m.piece.player, m.piece.kind)
				=> format!("={}", p.kind.name),
			_ => String::from(""),
		};

		if halfmove % 2 == 0
		{
			print!("{}. ", halfmove / 2 + 1);
		}

		print!("{}{}{}{}{}{}",
			m.piece.kind.name,
			d,
			x,
			m.to.notation(),
			p,
			c);

		b = m.board;
		pl = pl.opponent();
		halfmove = halfmove + 1;
	}

	println!("");
}


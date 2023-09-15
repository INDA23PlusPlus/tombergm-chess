extern crate chess;

fn print_board(b: & chess::Board)
{
	println!("  ---------------------------------");

	for y in 0..8
	{
		print!("{} |", 8 - y);

		for x in 0..8
		{
			let n = match b.at(chess::Loc {x: x, y: 7 - y})
			{
				chess::Square::Empty => String::from(" "),
				chess::Square::Occupied(p) =>
				match p.player
				{
					chess::Player::White =>
						String::from(
							if p.kind.name == ""
							{ "P" }
							else
							{ p.kind.name }
							)
						.to_uppercase(),
					chess::Player::Black =>
						String::from(
							if p.kind.name == ""
							{ "P" }
							else
							{ p.kind.name }
						)
						.to_lowercase(),
				},
			};

			print!(" {} |", n);
		}

		println!("");
		println!("  ---------------------------------");
	}

	println!("    a   b   c   d   e   f   g   h");
}

fn main()
{
	let mut b = chess::Board::default();
	let mut pl = chess::Player::White;
	let mut halfmove = 0;

	loop
	{
		print_board(& b);

		let ms = b.moves(pl);
		if ms.len() == 0
		{
			if b.is_check(pl)
			{
				if pl == chess::Player::White
				{
					println!("0-1");
				}
				else
				{
					println!("1-0");
				}
			}
			else
			{
				println!("1/2-1/2");
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

		let d = match b.at(m.to)
		{
			chess::Square::Empty => "",
			chess::Square::Occupied(_) =>
				if m.piece.kind.name == ""
				{
					["a", "b", "c", "d", "e", "f", "g", "h"]
					[m.from.x as usize]
				}
				else
				{
					""
				}
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

		println!("{}{} {}{}{}{}{}{}",
			halfmove / 2 + 1,
			[".", "..."][halfmove % 2],
			m.piece.kind.name,
			d,
			x,
			m.to.notation(),
			p,
			c);

		let mut l = String::new();
		let _ = std::io::stdin().read_line(& mut l);

		b = m.board;
		pl = pl.opponent();
		halfmove = halfmove + 1;
	}
}


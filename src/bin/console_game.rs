extern crate chess;

fn print_board(board: & chess::Board)
{
	println!("  ---------------------------------");

	for y in 0..8
	{
		print!("{} |", 8 - y);

		for x in 0..8
		{
			let loc = chess::Loc {x: x, y: 7 - y};

			let mut name = String::from(match board.at(loc)
			{
				chess::Square::Empty => " ",
				chess::Square::Occupied(piece) =>
					piece.kind.name
			});
		
			if let chess::Square::Occupied(piece) = board.at(loc)
			{
				if piece.player == chess::Player::Black
				{
					name = name.to_lowercase();
				}
			}

			print!(" {} |", name);
		}

		println!();
		println!("  ---------------------------------");
	}

	println!("    a   b   c   d   e   f   g   h");
}

fn main()
{
	println!("Enter the name of the destination square (e.g. `e4`),");
	print!("or both the departure the destination square of your move");
	println!("(e.g `e2 e4`).");

	/* Create a new chess game */
	let mut game = chess::Game::new();

	print_board(& game.board());

	loop
	{
		match game.player()
		{
			chess::Player::White => println!("White to play:"),
			chess::Player::Black => println!("Black to play:"),
		}

		let mut input = String::new();
		let _ = std::io::stdin().read_line(& mut input);

		let mut input_tokens = input.split_whitespace();
		let mut from = (|t| { chess::Loc::parse(t?) })
			(input_tokens.next());
		let mut to = (|t| { chess::Loc::parse(t?) })
			(input_tokens.next());

		if matches!(from, Some(_)) && matches!(to, None)
		{
			to = from;
			from = None;
		}

		let moves = game.get_moves(from, to);
		let m: & chess::Move;

		if moves.len() == 0
		{
			println!("No matching move was found. Try again.");

			continue;
		}
		else if moves.len() == 1
		{
			m = & moves[0];
		}
		else
		{
			print!("Several matching moves were found.");
			println!(" Pick one:");

			for i in 0..moves.len()
			{	
				println!("{}. {}",
					i + 1,
					(moves[i].notation_fn)(& moves[i],
							& game.board(),
					& moves));
			}

			input = String::new();
			let _ = std::io::stdin().read_line(& mut input);

			let n_result = input.trim().parse::<usize>();

			match n_result
			{
				Ok(n) if 0 < n && n <= moves.len()
					=> m = & moves[n - 1],
				_ =>
				{
					print!("Invalid choice.");
					println!(" Try another move.");

					continue;
				},
			}
		}

		if game.halfmove() % 2 == 0
		{
			print!("{}. ", game.fullmove());
		}
		else
		{
			print!("{}... ", game.fullmove());
		}

		println!("{}", m.notation(& game.board(), & moves));

		/* Play the move */
		game.play_move(m);

		/* Print the new board */
		print_board(& game.board());

		/* Stop if the game has ended */
		if game.state() != chess::State::Playing
		{
			break;
		}
	}

	/* Print the score if the game has ended */
	if game.state() != chess::State::Playing
	{
		let s = game.score().map(|s| match s
			{
				1 => "1/2",
				2 => "1",
				_ => "0",
			});

		print!("{}-{}", s[0], s[1]);
	}


	println!("");
}


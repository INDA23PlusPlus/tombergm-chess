extern crate chess;

fn main()
{
	/* Create a new chess game */
	let mut game = chess::Game::new();

	/* Play a game of at most 400 halfmoves */
	for i in 0..400
	{
		if i != 0
		{
			print!(" ");
		}

		/* Get a list of possible moves. (None, None) means to put no
		 * constraint on the departure or destination square. They
		 * would otherwise be Some(chess::Loc {file, rank}). Files and
		 * ranks are zero-indexed. */
		let moves = game.get_moves(None, None);

		/* Pick a move at random */
		let m = & moves[rand::random::<usize>() % moves.len()];

		/* Print the move number */
		if game.halfmove() % 2 == 0
		{
			print!("{}. ", game.fullmove());
		}

		/* Print the move notation */
		print!("{}", m.notation(& game.board(), & moves));

		/* Play the move */
		game.play_move(m);

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

		print!(" {}-{}", s[0], s[1]);
	}

	println!();
}


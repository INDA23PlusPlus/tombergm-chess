extern crate chess;

fn main()
{
	/* Create a new chess game */
	let mut game = chess::Game::new();

	/* Play a game of at most 400 halfmoves */
	for _ in 0..400
	{
		/* Get a list of possible moves. (None, None) means to put no
		 * constraint on the departure or destination square. They
		 * would otherwise be Some(chess::Loc {file, rank}). Files and
		 * ranks are zero-indexed. */
		let moves = game.get_moves(None, None);

		/* Pick a move at random */
		let m = & moves[rand::random::<usize>() % moves.len()];

		/* Play the move */
		game.play_move(m);

		/* Stop if the game has ended */
		if game.state() != chess::State::Playing
		{
			break;
		}
	}

	/* Print the movetext for the game */
	println!("{}", game.movetext());
}

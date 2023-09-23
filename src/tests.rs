#[cfg(test)]
mod tests
{
	use board::Board;

	fn perft(board: & Board, depth: u32) -> u64
	{
		match depth
		{
			0 => 1,
			_ =>
			{
				let mut n: u64 = 0;

				for m in & board.moves(board.player)
				{
					n = n + perft(& m.board, depth - 1);
				}

				n
			}
		}
	}

	/* perft values lifted from chessprogramming.com/Perft_Results */

	#[test]
	fn perft_p1_d0()
	{
		assert_eq!(perft(& Board::default(), 0), 1);
	}

	#[test]
	fn perft_p1_d1()
	{
		assert_eq!(perft(& Board::default(), 1), 20);
	}

	#[test]
	fn perft_p1_d2()
	{
		assert_eq!(perft(& Board::default(), 2), 400);
	}

	#[test]
	fn perft_p1_d3()
	{
		assert_eq!(perft(& Board::default(), 3), 8902);
	}

	#[test]
	fn perft_p1_d4()
	{
		assert_eq!(perft(& Board::default(), 4), 197281);
	}

	#[test]
	fn perft_p1_d5()
	{
		assert_eq!(perft(& Board::default(), 5), 4865609);
	}

	const P2: & str = concat!(
		"r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
		" w KQkq - 0 1");

	#[test]
	fn perft_p2_d3()
	{
		assert_eq!(perft(& Board::from_fen(P2).unwrap(), 3), 97862);
	}

	#[test]
	fn perft_p2_d4()
	{
		assert_eq!(perft(& Board::from_fen(P2).unwrap(), 4), 4085603);
	}

	const P3: & str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

	#[test]
	fn perft_p3_d5()
	{
		assert_eq!(perft(& Board::from_fen(P3).unwrap(), 5), 674624);
	}

	const P4: & str = concat!(
		"r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
		" w kq - 0 1");

	#[test]
	fn perft_p4_d4()
	{
		assert_eq!(perft(& Board::from_fen(P4).unwrap(), 4), 422333);
	}

	const P5: & str = concat!(
		"rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
		" w KQ - 1 8");

	#[test]
	fn perft_p5_d3()
	{
		assert_eq!(perft(& Board::from_fen(P5).unwrap(), 3), 62379);
	}
}


#[cfg(test)]
mod tests
{
	use board::Board;

	fn perft(depth: u32) -> u64
	{
		perft_r(depth, & Board::default())
	}

	fn perft_r(depth: u32, board: & Board) -> u64
	{
		match depth
		{
			0 => 1,
			_ =>
			{
				let mut n: u64 = 0;

				for m in & board.moves(board.player)
				{
					n = n + perft_r(depth - 1, & m.board);
				}

				n
			}
		}
	}

	/* perft values lifted from chessprogramming.com/Perft_Results */

	#[test]
	fn perft_depth_0()
	{
		assert_eq!(perft(0), 1);
	}

	#[test]
	fn perft_depth_1()
	{
		assert_eq!(perft(1), 20);
	}

	#[test]
	fn perft_depth_2()
	{
		assert_eq!(perft(2), 400);
	}

	#[test]
	fn perft_depth_3()
	{
		assert_eq!(perft(3), 8902);
	}

	#[test]
	fn perft_depth_4()
	{
		assert_eq!(perft(4), 197281);
	}

	#[test]
	fn perft_depth_5()
	{
		assert_eq!(perft(5), 4865609);
	}
}


use board::*;
use r#move::*;
use player::*;

#[derive(Copy, Clone, PartialEq)]
pub enum State
{
	Playing,
	Checkmate,
	Stalemate,
}

#[derive(Clone)]
pub struct Game
{
	start_board	: Board,
	moves		: Vec<Move>,
}

impl Game
{
	pub fn new() -> Self
	{
		Game
		{
			start_board	: Board::default(),
			moves		: Vec::<Move>::new(),
		}
	}

	pub fn from_board(board: Board) -> Self
	{
		Game
		{
			start_board	: board,
			moves		: Vec::<Move>::new(),
		}
	}

	pub fn player(self: & Self) -> Player
	{
		self.board().player
	}

	pub fn board(self: & Self) -> Board
	{
		self.board_at(self.halfmove())
	}

	pub fn board_at(self: & Self, halfmove: i32) -> Board
	{
		if halfmove == 0
		{
			self.start_board
		}
		else
		{
			self.moves[(halfmove - 1) as usize].board
		}
	}

	pub fn get_moves(self: & Self, from: Option<Loc>, to: Option<Loc>)
		-> Vec<Move>
	{
		let mut ms = self.board().moves(self.player());

		if let Some(from) = from
		{
			ms.retain(|m| m.from == from);
		}
		if let Some(to) = to
		{
			ms.retain(|m| m.to == to);
		}

		ms
	}

	pub fn play_move(self: & mut Self, r#move: & Move)
	{
		self.moves.push(*r#move);
	}

	pub fn halfmove(self: & Self) -> i32
	{
		self.moves.len() as i32
	}

	pub fn fullmove(self: & Self) -> i32
	{
		1 + (self.moves.len() / 2) as i32
	}

	pub fn state(self: & Self) -> State
	{
		if self.get_moves(None, None).len() == 0
		{
			if self.board().is_check(self.player())
			{
				State::Checkmate
			}
			else
			{
				State::Stalemate
			}
		}
		else
		{
			State::Playing
		}
	}

	pub fn score(self: & Self) -> [i32; 2]
	{
		match self.state()
		{
			State::Playing => [0, 0],
			State::Stalemate => [1, 1],
			State::Checkmate => match self.player()
			{
				Player::White => [0, 2],
				Player::Black => [2, 0],
			}
		}
	}

	pub fn movetext(self: & Self) -> String
	{
		let mut s = String::new();

		for i in 0..self.halfmove()
		{
			let b = self.board_at(i);
			let m = & self.moves[i as usize];
			let ms = b.moves(b.player);

			/* Print a space in between moves */
			if i != 0
			{
				s.push(' ');
			}

			/* Print the move number */
			if i % 2 == 0
			{
				s.push_str(format!("{}. ", 1 + i / 2)
					.as_str());	
			}

			/* Print the move notation */
			s.push_str(format!("{}", m.notation(& b, & ms))
					.as_str());
		}

		/* Print the score if the game has ended */
		if self.state() != State::Playing
		{
			let r = self.score().map(|r| match r
				{
					1 => "1/2",
					2 => "1",
					_ => "0",
				});

			s.push_str(format!(" {}-{}", r[0], r[1]).as_str());
		}

		s
	}
}

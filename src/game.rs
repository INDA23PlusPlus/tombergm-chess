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
}


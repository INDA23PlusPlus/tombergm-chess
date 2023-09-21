#[derive(Copy, Clone, PartialEq)]
pub enum Player
{
	White,
	Black,
}

impl Player
{
	pub fn opponent(self: & Self) -> Self
	{
		match self
		{
			Self::White => Self::Black,
			Self::Black => Self::White,
		}
	}
}


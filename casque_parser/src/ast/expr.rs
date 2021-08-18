#[derive(Debug, PartialEq)]
pub enum Expr {
	If(),
	Binary(),
	Unary(),
	Atomic(Atomic)
}

impl Expr {
	pub fn boolean(b: bool) -> Self {
		Self::Atomic(Atomic::Bool(b))
	}
	
	pub fn character(c: char) -> Self {
		Self::Atomic(Atomic::Char(c))
	}

	pub fn integer(i: i32) -> Self {
		Self::Atomic(Atomic::Integer(i))
	}
	
	pub fn ident(i: &str) -> Self {
		Self::Atomic(Atomic::Ident(i.into()))
	}
}

#[derive(Debug, PartialEq)]
pub enum Atomic {
	Bool(bool),
	Char(char),
	Integer(i32),
	Ident(String),
}
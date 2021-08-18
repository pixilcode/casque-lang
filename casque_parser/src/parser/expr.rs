use crate::parser::{Input, ParserResult};
use crate::parser::scanner::*;
use crate::ast::Expr;
use nom::combinator::map;

fn atomic(input: Input) -> ParserResult<Expr> {
	map(
		boolean,
		|s: &str| match s {
			"true" => Expr::boolean(true),
			"false" => Expr::boolean(false),
			_ => unreachable!()
		}
	)(input).or_else(
		|_| map(
			character,
			|s: &str| if s.starts_with('\\') {
				match s {
					"\\n" => Expr::character('\n'),
					"\\t" => Expr::character('\t'),
					"\\\\" => Expr::character('\\'),
					"\\'" => Expr::character('\''),
					_ => unreachable!()
				}
			} else {
				Expr::character(s.chars().next().unwrap())
			}
		)(input)
	).or_else(
		|_| map(
			number,
			|s: &str| if s.contains('_') {
				Expr::integer(s.replace('_', "").parse().unwrap())
			} else {
				Expr::integer(s.parse().unwrap())
			}
		)(input)
	).or_else(
		|_| map(
			ident,
			|s: &str| Expr::ident(s)
		)(input)
	)
}

mod tests {
	#![allow(unused_imports)]
	use super::*;
	use crate::ast::Expr;

	parser_tests! {
		atomic_test: atomic {
			// booleans
			"true" => Ok(("", Expr::boolean(true)));
			"false" => Ok(("", Expr::boolean(false)));

			// characters
			"'c'" => Ok(("", Expr::character('c')));
			"'\\n'" => Ok(("", Expr::character('\n')));
			"'\\t'" => Ok(("", Expr::character('\t')));
			"'\\''" => Ok(("", Expr::character('\'')));
			"'\\\\'" => Ok(("", Expr::character('\\')));

			// numbers
			"123" => Ok(("", Expr::integer(123)));
			"123_456" => Ok(("", Expr::integer(123456)));

			// variable
			"abc" => Ok(("", Expr::ident("abc")));
			"a_b_c" => Ok(("", Expr::ident("a_b_c")));
			"abc123" => Ok(("", Expr::ident("abc123")));
		}
	}
}
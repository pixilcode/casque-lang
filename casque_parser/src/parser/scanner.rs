use crate::parser::{Input, ScannerResult};
use nom::bytes::complete::tag;

pub fn ident(input: Input) -> ScannerResult<Input> {
    Ok(("", ""))
}

pub fn number(input: Input) -> ScannerResult<Input> {
    Ok(("", ""))
}

pub fn character(input: Input) -> ScannerResult<Input> {
    Ok(("", ""))
}

pub fn boolean(input: Input) -> ScannerResult<Input> {
    Ok(("", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    parser_tests! {
        variable_test: ident {
            "abc" => Ok(("", "abc"));
            "a_b_c" => Ok(("", "a_b_c"));
            "abc_123" => Ok(("", "abc_123"));
            "_abc" => Ok(("", "_abc"));
            "123abc" => Err(nom::Err::Error(nom::error::Error {
                input: "123abc",
                code: nom::error::ErrorKind::OneOf
            }));
        }

		number_test: number {
			"123" => Ok(("", "123"));
			"12_345" => Ok(("", "12_345"));
			"12a" => Ok(("a", "12"));
			"a12" => Err(nom::Err::Error(nom::error::Error {
				input: "a12",
				code: nom::error::ErrorKind::Alt
			}));
		}

		character_test: character {
			"'a'" => Ok(("", "a"));
			"'\\n'" => Ok(("", "\n"));
			"\"a\"" => Err(nom::Err::Error(nom::error::Error {
				input: "\"a\"",
				code: nom::error::ErrorKind::Tag
			}));
		}

		bool_test: boolean {
			"true" => Ok(("", "true"));
			"false" => Ok(("", "false"));
		}

		// Ensure that keywords aren't parsed as identifiers
		keyword_test: ident {}
    }
}

macro_rules! simple_token {
    ($name:ident $lexeme:expr) => {
        pub fn $name(input: Input) -> ScannerResult<Input> {
            tag($lexeme)(input)
        }
    };
}

// Simple tokens generated by `simple_token_gen.tgo`
simple_token!(colon ":");
simple_token!(double_colon "::");
simple_token!(open_paren "(");
simple_token!(close_paren ")");
simple_token!(comma ",");
simple_token!(arrow "->");
simple_token!(eq "=");
simple_token!(q_mark "?");
simple_token!(double_bar "||");
simple_token!(double_amp "&&");
simple_token!(bar "|");
simple_token!(carot "^");
simple_token!(amp "&");
simple_token!(double_eq "==");
simple_token!(bang_eq "!=");
simple_token!(gt ">");
simple_token!(lt "<");
simple_token!(gt_eq ">=");
simple_token!(lt_eq "<=");
simple_token!(double_gt ">>");
simple_token!(double_lt "<<");
simple_token!(plus "+");
simple_token!(minus "-");
simple_token!(star "*");
simple_token!(slash "/");
simple_token!(percent "%");
simple_token!(bang "!");
simple_token!(tilde "~");

#[cfg(test)]
mod simple_tests {
    use super::*;

    parser_tests! {
            colon_test: colon { ":" => Ok(("", ":")); }
            double_colon_test: double_colon { "::" => Ok(("", "::")); }
            open_paren_test: open_paren { "(" => Ok(("", "(")); }
            close_paren_test: close_paren { ")" => Ok(("", ")")); }
            comma_test: comma { "," => Ok(("", ",")); }
            arrow_test: arrow { "->" => Ok(("", "->")); }
            eq_test: eq { "=" => Ok(("", "=")); }
            q_mark_test: q_mark { "?" => Ok(("", "?")); }
            double_bar_test: double_bar { "||" => Ok(("", "||")); }
            double_amp_test: double_amp { "&&" => Ok(("", "&&")); }
            bar_test: bar { "|" => Ok(("", "|")); }
            carot_test: carot { "^" => Ok(("", "^")); }
            amp_test: amp { "&" => Ok(("", "&")); }
            double_eq_test: double_eq { "==" => Ok(("", "==")); }
            bang_eq_test: bang_eq { "!=" => Ok(("", "!=")); }
            gt_test: gt { ">" => Ok(("", ">")); }
            lt_test: lt { "<" => Ok(("", "<")); }
            gt_eq_test: gt_eq { ">=" => Ok(("", ">=")); }
            lt_eq_test: lt_eq { "<=" => Ok(("", "<=")); }
            double_gt_test: double_gt { ">>" => Ok(("", ">>")); }
            double_lt_test: double_lt { "<<" => Ok(("", "<<")); }
            plus_test: plus { "+" => Ok(("", "+")); }
            minus_test: minus { "-" => Ok(("", "-")); }
            star_test: star { "*" => Ok(("", "*")); }
            slash_test: slash { "/" => Ok(("", "/")); }
            percent_test: percent { "%" => Ok(("", "%")); }
            bang_test: bang { "!" => Ok(("", "!")); }
            tilde_test: tilde { "~" => Ok(("", "~")); }
    }
}

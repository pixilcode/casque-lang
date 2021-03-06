macro_rules! parser_tests {
	( $( $test_name:ident : $parser:ident { $( $input:expr => $output:expr ; )* } )* ) => {
		$(
			#[test]
			fn $test_name() {
				$(assert_eq!($parser($input), $output));*
			}
		)*
	};
}

type Input<'a> = &'a str;
type ParserResult<'a, O> = nom::IResult<Input<'a>, O>;

fn nom_error<O>(input: &str, code: nom::error::ErrorKind) -> ParserResult<O> {
	Err(nom::Err::Error(nom::error::Error {
		input,
		code
	}))
}

mod scanner;
mod expr;
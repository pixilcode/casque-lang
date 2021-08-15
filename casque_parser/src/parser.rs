#[macro_use]
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
type ScannerResult<'a, O> = nom::IResult<Input<'a>, O>;

mod scanner;

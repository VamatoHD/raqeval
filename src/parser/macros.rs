macro_rules! count_captures {
    ($count:expr ;) => { $count };
    ($count:expr ; [$pat:pat] $(, $($rest:tt)*)? ) => { count_captures!($count + 1 ; $($($rest)*)? ) };
    ($count:expr ; $pat:pat $(, $($rest:tt)*)? ) => { count_captures!($count ; $($($rest)*)? ) };
}

macro_rules! capture_inner {
    ($tokens: expr, $groups:expr, $matched: expr, $index: expr; [$pat:pat] $(, $($rest:tt)*)?) => {
        if $matched {
            use $crate::lexer::Token;
            let mut temp: Vec<Token> = Vec::new();

            while matches!($tokens.peek(), Some($pat)) {
                let v = $tokens.next().unwrap();
                temp.push(v)
            }

            if temp.len() == 0 {
                $matched = false
            }

            $groups[$index] = temp;
        };
        capture_inner!($tokens, $groups, $matched, $index + 1; $($($rest)*)?);
    };
    ($tokens: expr, $groups:expr, $matched: expr, $index: expr; $pat:pat $(, $($rest:tt)*)?) => {
        if !matches!($tokens.next(), Some($pat)) {
            $matched = false;
        };
        capture_inner!($tokens, $groups, $matched, $index; $($($rest)*)?);
    };
    ($tokens: expr, $groups:expr, $matched: expr, $index: expr;) => {};
}

macro_rules! capture {
    ($tokens:expr, $($rest:tt)*) => {{
        const N: usize = count_captures!(0; $($rest)*);

        use $crate::lexer::Token;
        let mut iter: ::std::iter::Peekable<_> = $tokens.into_iter().peekable();

        let mut matched = true;
        let mut captures: [Vec<Token>; N] = std::array::from_fn(|_| Vec::new());

        capture_inner!(iter, captures, matched, 0; $($rest)*);

        if matched {
            Some(captures)
        } else {
            None
        }
    }};
}

pub(super) use capture;

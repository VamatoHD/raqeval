use crate::{Error, Expr, Func, lexer::*, rat};

fn compute_atom(lexer: &mut Lexer) -> Result<Expr, Error> {
    match lexer.next() {
        Token::LParen => {
            let expr = compute_expr(lexer, 1)?;
            if matches!(lexer.next(), Token::RParen) {
                Ok(expr)
            } else {
                Err(Error::InvalidParens)
            }
        }

        Token::String(s) => {
            if let Some(c) = crate::Consts::from_str(s.as_str()) {
                Ok(Expr::Const(c))
            } else if lexer.peek() == Token::LParen {
                //A function call
                lexer.next(); //Consume "("

                let mut args = Vec::new();

                if lexer.peek() != Token::RParen {
                    loop {
                        args.push(compute_expr(lexer, 1)?);

                        match lexer.next() {
                            Token::Comma => continue,
                            Token::RParen => break,
                            _ => return Err(Error::InvalidParens),
                        }
                    }
                } else {
                    lexer.next();
                }

                match s.as_str() {
                    "log" if args.len() == 1 => Ok(Expr::Log {
                        base: Box::new(Expr::Number(rat!(10))),
                        arg: Box::new(args.into_iter().next().unwrap()),
                    }),
                    "log" if args.len() == 2 => {
                        let mut args_iter = args.into_iter();
                        Ok(Expr::Log {
                            base: Box::new(args_iter.next().unwrap()),
                            arg: Box::new(args_iter.next().unwrap()),
                        })
                    }
                    "log" => Err(Error::InvalidFunc(
                        "log expects exactly 2 arguments".to_string(),
                    )),
                    _ => Ok(Expr::Call { func: s, args }),
                }
            } else {
                //A variable
                Ok(Expr::Var(s))
            }
        }

        Token::Number(n) => Ok(Expr::Number(n)),
        t => Err(Error::AtomExpected(t)),
    }
}

fn compute_expr(lexer: &mut Lexer, min_prec: usize) -> Result<Expr, Error> {
    let mut atom_lhs = compute_atom(lexer)?;

    loop {
        // Break out of the loop if:
        // - Precedence is lower
        // - No more tokens left
        // - Isn't a binary operation
        let op = match lexer.peek() {
            Token::Op(op) => {
                let (prec, _) = op.get_info();
                if prec < min_prec {
                    break;
                };
                op.clone()
            }
            Token::Eof => break,
            _ => break,
        };

        //Consume the next token
        lexer.next();

        let (prec, assoc) = op.get_info();
        let next_min_prec = if assoc == Assoc::Left { prec + 1 } else { prec };

        let atom_rhs = compute_expr(lexer, next_min_prec)?;

        atom_lhs = Expr::Infix {
            lhs: Box::new(atom_lhs),
            op,
            rhs: Box::new(atom_rhs),
        };
    }

    Ok(atom_lhs)
}

pub fn parse(input: &str) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(input)?;
    compute_expr(&mut lexer, 1)
}

pub fn parse_func(input: &str) -> Result<Func, Error> {
    let (lhs, rhs) = input
        .split_once("=")
        .ok_or_else(|| Error::InvalidFunc("no \"=\" found".to_string()))?;

    let mut lhs_iter = parse_string(lhs)?.into_iter().peekable();

    let mut func_name = String::new();
    while let Some(token) = lhs_iter.peek() {
        match token {
            Token::String(s) => func_name.push_str(s),
            Token::Number(n) if n.is_integer() && !n.is_integer() => {
                func_name.push_str(&n.to_string())
            }
            Token::LParen => break,
            _ => return Err(Error::InvalidFunc("invalid function name".to_string())),
        }
        lhs_iter.next();
    }

    if func_name.is_empty() {
        return Err(Error::InvalidFunc("empty function name".to_string()));
    }

    if !matches!(lhs_iter.next(), Some(Token::LParen)) {
        return Err(Error::InvalidFunc(
            "expected '(' after function name".to_string(),
        ));
    }

    let mut func_args = Vec::new();
    if !matches!(lhs_iter.peek(), Some(Token::RParen)) {
        loop {
            match lhs_iter.next() {
                Some(Token::String(s)) => func_args.push(s),
                _ => return Err(Error::InvalidFunc("expected argument name".to_string())),
            }

            match lhs_iter.next() {
                Some(Token::Comma) => continue,
                Some(Token::RParen) => break,
                _ => return Err(Error::InvalidFunc("expected ',' or ')'".to_string())),
            }
        }
    } else {
        lhs_iter.next();
    };

    let expr = parse(rhs)?;

    Ok(Func::new(func_name, func_args, expr))
}

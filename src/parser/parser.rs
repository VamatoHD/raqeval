use crate::{Ctx, Error, Expr, Func, Rational, lexer::*};

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
        Token::Ident(Ident::Func(func)) => {
            if !matches!(lexer.next(), Token::LParen) {
                return Err(Error::InvalidParens);
            }

            let expr = compute_expr(lexer, 0)?;

            if matches!(lexer.next(), Token::RParen) {
                Ok(Expr::Call {
                    func,
                    arg: Box::new(expr),
                })
            } else {
                Err(Error::InvalidParens)
            }
        }
        Token::Ident(Ident::Var(var)) => Ok(Expr::Var(var)),
        Token::Number(n) => Ok(Expr::Const(n)),
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

pub fn parse(input: &str, ctx: Option<&Ctx>) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(input, ctx)?;
    compute_expr(&mut lexer, 1)
}

pub fn parse_func(input: &str) -> Result<Func, Error> {
    let (lhs, rhs) = input
        .split_once("=")
        .ok_or_else(|| Error::InvalidFunc(todo!()))?;

    let mut lhs_tokens = parse_string(lhs, None, true)?.into_iter();

    let func_name = {
        let mut parts = Vec::new();
        let mut had_lparen = false;
        while let Some(token) = lhs_tokens.next() {
            match token {
                Token::LParen => {
                    had_lparen = true;
                    break;
                }
                Token::Ident(Ident::Unknown(v)) => parts.push(v.to_string()),
                Token::Number(n) => {
                    if n.is_integer() && !n.is_neg() {
                        parts.push(n.to_string())
                    } else {
                        return Err(Error::InvalidFunc(todo!()));
                    }
                }

                _ => return Err(Error::InvalidFunc(todo!())),
            }
        }

        if !had_lparen {
            return Err(Error::InvalidFunc(todo!()));
        }

        let name = parts.join("");
        if name.is_empty() {
            return Err(Error::InvalidFunc(todo!()));
        }
        name
    };

    let Some(Token::Ident(Ident::Unknown(func_arg))) = lhs_tokens.next() else {
        return Err(Error::InvalidFunc(todo!()));
    };

    let ctx = {
        let mut ctx = Ctx::new();
        ctx.add_var(func_arg.to_string().as_str());
        ctx
    };

    let expr = parse(rhs, Some(&ctx))?;

    Ok(Func::new(
        func_name.as_str(),
        func_arg.to_string().as_str(),
        expr,
    ))
}

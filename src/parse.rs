use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    #[token(">")]
    PointerIncrement,

    #[token("<")]
    PointerDecrement,

    #[token("+")]
    CellIncrement,

    #[token("-")]
    CellDecrement,

    #[token(".")]
    CellOutput,

    #[token(",")]
    CellInput,

    #[token("[")]
    Open,

    #[token("]")]
    Close,

    #[token("#")]
    Breakpoint,

    #[regex(r"[^><\+\-\.,\[\]]+", logos::skip)]
    #[error]
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    PointerIncrement,
    PointerDecrement,
    CellIncrement,
    CellDecrement,
    CellOutput,
    CellInput,
    Open(usize),
    Close(usize),
    Breakpoint,
}

pub fn parse_program(program: &str) -> Vec<Instruction> {
    let lex = Token::lexer(program);
    let indexed_tokens = lex.enumerate().collect::<Vec<_>>();

    indexed_tokens
        .iter()
        .map(|(index, token)| match token {
            Token::PointerIncrement => Instruction::PointerIncrement,
            Token::PointerDecrement => Instruction::PointerDecrement,
            Token::CellIncrement => Instruction::CellIncrement,
            Token::CellDecrement => Instruction::CellDecrement,
            Token::CellOutput => Instruction::CellOutput,
            Token::CellInput => Instruction::CellInput,
            Token::Breakpoint => Instruction::Breakpoint,
            Token::Error => unreachable!(),
            Token::Open => {
                let mut close = *index;
                let mut opened: usize = 1;

                while opened != 0 {
                    close += 1;
                    if let Some((_, ft)) = indexed_tokens.get(close) {
                        match ft {
                            Token::Open => opened += 1,
                            Token::Close => opened -= 1,
                            _ => (),
                        }
                    } else {
                        panic!("unclosed [");
                    }
                }
                Instruction::Open(close)
            }
            Token::Close => {
                let mut open = *index;
                let mut closed: usize = 1;

                while closed != 0 {
                    open -= 1;
                    if let Some((_, ft)) = indexed_tokens.get(open) {
                        match ft {
                            Token::Close => closed += 1,
                            Token::Open => closed -= 1,
                            _ => (),
                        }
                    } else {
                        panic!("unclosed ]");
                    }
                }
                Instruction::Close(open)
            }
        })
        .collect()
}

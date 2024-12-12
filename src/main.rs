// https://soundcloud.com/geekedhub/meth-pipe

enum UndNodeKind {
    Plain(String),
    Parenthesized(String),
}
struct UndNode {
    idx: usize,
    kind: UndNodeKind,
}
struct UndParsingResult {
    unexpected_closers: Vec<usize>,
    unclosed_openers: Vec<usize>,
    text: Vec<UndNode>,
}
fn parse_und(input: &str) -> UndParsingResult {
    let mut text = Vec::new();
    let mut unexpected_closers = Vec::new();
    let mut unclosed_openers = Vec::new();

    let mut nextidx = 0;
    let mut curidx = nextidx;

    let mut sbuf = String::new();
    let mut sbufidx = nextidx;

    let mut escaped = false;

    loop {
        let c = unsafe { input.get_unchecked(nextidx..) }.chars().next();
        if let Some(c) = c {
            nextidx += c.len_utf8();
            if escaped {
                escaped = false;
                sbuf.push(c);
                continue;
            } else {
                curidx = nextidx;
            }
        }
        match c {
            None | Some('(') | Some(')') => {
                if !sbuf.is_empty() {
                    text.push(UndNode {
                        idx: sbufidx,
                        kind: if unclosed_openers.is_empty() {
                            UndNodeKind::Plain(sbuf)
                        } else {
                            UndNodeKind::Parenthesized(sbuf)
                        },
                    });
                    sbuf = String::new();
                    sbufidx = nextidx;
                }
                if c == None {
                    break;
                }
                if c == Some('(') {
                    if !unclosed_openers.is_empty() {
                        sbuf.push('(');
                    }
                    unclosed_openers.push(curidx);
                } else {
                    match unclosed_openers.pop() {
                        None => unexpected_closers.push(curidx),
                        Some(_) => {
                            if !unclosed_openers.is_empty() {
                                sbuf.push(')')
                            }
                        }
                    }
                }
            }
            Some(c) => {
                if c == '\\' {
                    escaped = true;
                }
                sbuf.push(c);
            }
        }
    }

    UndParsingResult {
        text,
        unexpected_closers,
        unclosed_openers,
    }
}

enum PonCommandKind {
    Name(Vec<PonWord>),
    Invocation(String),
}
struct PonCommand {
    idx: usize,
    kind: PonCommandKind,
}
type PonWord = String;
fn und_to_pon(und: Vec<UndNode>) -> Vec<PonCommand> {
    let mut program = Vec::new();
    for und_node in und {
        match und_node.kind {
            UndNodeKind::Parenthesized(input) => {
                program.push(PonCommand {
                    idx: und_node.idx,
                    kind: PonCommandKind::Invocation(input),
                });
            }
            UndNodeKind::Plain(plain) => {
                if let Some((idx, _)) = plain.char_indices().find(|(_, c)| !c.is_whitespace()) {
                    program.push(PonCommand {
                        idx,
                        kind: PonCommandKind::Name(
                            plain.split_whitespace().map(str::to_owned).collect(),
                        ),
                    })
                }
            }
        }
    }
    program
}

fn main() {
    println!("Hello, world!");
}

// https://soundcloud.com/geekedhub/meth-pipe

#[derive(Debug)]
enum UndNodeKind {
    Plain(String),
    Parenthesized(String),
}
#[derive(Debug)]
struct UndNode {
    idx: usize,
    kind: UndNodeKind,
}
#[derive(Debug)]
struct UndParsingResult {
    unexpected_closers: Vec<usize>,
    unclosed_openers: Vec<usize>,
    nodes: Vec<UndNode>,
}
fn parse_und(input: &str) -> UndParsingResult {
    let mut nodes = Vec::new();
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
                    nodes.push(UndNode {
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
        nodes,
        unexpected_closers,
        unclosed_openers,
    }
}

#[derive(Debug)]
enum PonCommandKind {
    Name(Vec<PonWord>),
    Invocation(String),
}
#[derive(Debug)]
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
    let contents = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let und_results = parse_und(&contents);
    println!("Und results: {:#?}", und_results);
    let pon = und_to_pon(und_results.nodes);
    println!("Pon: {:#?}", pon);
}

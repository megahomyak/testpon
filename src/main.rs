enum UndNodeKind {
    Text(String),
    Group(Vec<UndNode>),
}
struct UndNode {
    idx: usize,
    kind: UndNodeKind,
}
struct UndParsingResult {
    unexpected_closers: Vec<usize>,
    unclosed_openers: Vec<usize>,
    root: Vec<UndNode>,
    escape_at_end_of_input: bool,
}
macro_rules! und_get_top {
    ($overlays: ident, $root: ident) => {
        match $overlays.last_mut() {
            None => &mut $root,
            Some(top) => &mut top.group,
        }
    };
}
fn parse_und(input: &str) -> UndParsingResult {
    let mut root = Vec::new();
    struct Overlay {
        idx: usize,
        group: Vec<UndNode>,
    }
    let mut overlays: Vec<Overlay> = vec![];

    let mut unclosed_openers = vec![];
    let mut unexpected_closers = vec![];

    let mut idx = 0;
    let mut curidx = idx;

    let mut sbuf = String::new();
    let mut sbufidx = idx;

    let mut escaped = false;

    loop {
        let c = unsafe { input.get_unchecked(idx..) }.chars().next();
        if let Some(c) = c {
            curidx = idx;
            idx += c.len_utf8();
            if escaped {
                escaped = false;
                if !(c == '(' || c == ')' || c == '\\') {
                    sbuf.push('\\');
                }
                sbuf.push(c);
                continue;
            }
        }

        match c {
            Some(')') | None | Some('(') => {
                if sbuf != "" {
                    let top = und_get_top!(overlays, root);
                    top.push(UndNode {
                        idx: sbufidx,
                        kind: UndNodeKind::Text(sbuf),
                    });
                    sbufidx = idx;
                    sbuf = String::new();
                }
                if c == Some(')') || c == None {
                    match overlays.pop() {
                        None => {
                            if c == None {
                                break;
                            } else {
                                unexpected_closers.push(curidx);
                            }
                        }
                        Some(old_top) => {
                            let new_top = und_get_top!(overlays, root);
                            new_top.push(UndNode {
                                idx: old_top.idx,
                                kind: UndNodeKind::Group(old_top.group),
                            });
                            if c == None {
                                unclosed_openers.push(old_top.idx);
                            }
                        }
                    }
                } else {
                    overlays.push(Overlay {
                        idx: curidx,
                        group: Vec::new(),
                    });
                }
            }
            Some('\\') => escaped = true,
            Some(c) => sbuf.push(c),
        }
    }

    UndParsingResult {
        unclosed_openers,
        unexpected_closers,
        root,
        escape_at_end_of_input: escaped,
    }
}

enum PonCommandKind {
    Name(Vec<PonWord>),
    Invocation(Vec<UndNode>),
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
            UndNodeKind::Group(group) => program.push(PonCommand {
                idx: und_node.idx,
                kind: PonCommandKind::Invocation(group),
            }),
            UndNodeKind::Text(text) => {
                if let Some((name_idx, _)) = text.char_indices().find(|(_, c)| !c.is_whitespace()) {
                    program.push(PonCommand {
                        idx: und_node.idx + name_idx,
                        kind: PonCommandKind::Name(
                            text.split_whitespace().map(|s| s.to_owned()).collect(),
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

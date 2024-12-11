from types import SimpleNamespace as SN

class UndGroup(list): pass # List[Node]
class UndText(str): pass
# UndNode = {value: UndGroup | UndText, idx: int}

def parse_und(input_: str) -> UndGroup:
    root = UndGroup([])
    overlays = []

    unclosed_openers = []
    unexpected_closers = []

    idx = 0
    curidx = idx

    sbuf = ""
    sbufidx = idx

    escaped = False

    while True:
        try: c = input_[idx]
        except IndexError: c = '\0'
        if c != '\0':
            curidx = idx
            idx += 1
            if escaped:
                escaped = False
                if not (c == '(' or c == ')' or c == '\\'):
                    sbuf += '\\'
                sbuf += c
                continue

        if c == ')' or c == '\0' or c == '(':
            if sbuf != "":
                try: top = overlays[-1]
                except IndexError: top = root
                top.append(SN(idx=sbufidx, value=UndText(sbuf)))
                sbufidx = idx
                sbuf = ""
            if c == ')' or c == '\0':
                try:
                    old_top = overlays.pop()
                except IndexError:
                    if c == '\0':
                        break
                    else:
                        unexpected_closers.append(curidx)
                else:
                    try: new_top = overlays[-1]
                    except IndexError: new_top = root
                    new_top.append(SN(idx=old_top.idx, value=old_top.group))
                    if c == '\0':
                        unclosed_openers.append(old_top.idx)
            else:
                overlays.append(SN(idx=curidx, group=UndGroup([])))
        else if c == '\\':
            escaped = True
        else:
            sbuf += c

    return SN(
        unexpected_closers=unexpected_closers,
        unclosed_openers=unclosed_openers,
        root=root,
    )

# PonProgram = List[PonCommand]
# PonCommand = {value: PonName | PonInvocation, idx: int}
class PonName(list): pass # List[PonWord]
class PonInvocation(UndGroup): pass
# PonWord = str
def und_to_pon(und: UndGroup):
    pon_program = []
    for und_node in und:
        if isinstance(und_node, UndText):
            name = []
            # TODO
        elif isinstance(und_node, UndGroup):
            pon_program.append(SN(idx=und_node.idx, value=PonInvocation(und_node.value)))

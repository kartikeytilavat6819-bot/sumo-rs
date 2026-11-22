#!/usr/bin/env python3
"""Import official netconvert TextTest error-exit leaves (exitcode.netconvert=1)."""
import sys, os, shutil
from pathlib import Path
sys.path.insert(0, 'tools/oracle')
import import_texttest_tools as t

START = int(sys.argv[1]); END = int(sys.argv[2])
sub = Path('C:/Users/karti/sumo-src/tests/netconvert/import')
rowsfile = Path('tools/oracle/import_texttest_net.rows')
CASES = t.CASES

leaves = []
for leaf in t.leaf_tests(sub, 'netconvert'):
    root = t.tests_root(leaf, 'netconvert')
    rel = leaf.relative_to(root).as_posix()
    tokens = t.effective_options(leaf, 'netconvert', root)
    # keep only genuine error-exit leaves: exitcode nonzero, no knownbugs, no unsupported options
    ec = None
    cur = leaf
    while cur == root or root in cur.parents:
        f = cur / 'exitcode.netconvert'
        if f.is_file() and f.read_text(encoding='utf-8', errors='replace').strip() not in ('', '0'):
            ec = f.read_text(encoding='utf-8', errors='replace').strip()
            break
        if cur == root:
            break
        cur = cur.parent
    if ec is None:
        continue
    # skip the same reasons as the importer, except the exitcode (that is the point here)
    bad = False
    cur = leaf
    while cur == root or root in cur.parents:
        kb = cur / 'knownbugs.netconvert'
        if kb.is_file() and kb.read_text(encoding='utf-8', errors='replace').strip():
            bad = True
            break
        if cur.name == 'errors':
            bad = True
            break
        if cur == root:
            break
        cur = cur.parent
    joined = ' '.join(tokens)
    if bad or any(s in joined for s in t.SKIP_TOKENS):
        continue
    leaves.append((rel, leaf, root, tokens, ec))

leaves.sort(key=lambda x: x[0])
new_rows = []
for rel, leaf, root, tokens, ec in leaves[START:END]:
    name = 'itt_nc_' + rel.replace('/', '_').replace('\\', '_').replace('-', '_').replace('.', '_')
    case = CASES / name
    if case.exists():
        shutil.rmtree(case)
    case.mkdir(parents=True, exist_ok=True)
    copied = t.copy_inputs(leaf, 'netconvert', root, case)
    opts, net = t.rewrite(tokens, 'netconvert', copied, case, '@D')
    inputs = [x for x in opts if '@D' not in x]
    outputs = [x for x in opts if '@D' in x]
    (case / 'net.args').write_text(' '.join(inputs) + '\n', encoding='utf-8', newline='\n')
    # official expectation, verbatim from the texttest leaf (nearest file wins)
    lines = []
    for fname in ('errors.netconvert', 'output.netconvert'):
        cur = leaf
        while cur == root or root in cur.parents:
            f = cur / fname
            if f.is_file():
                lines.extend(ln.rstrip('\r') for ln in f.read_text(encoding='utf-8', errors='replace').splitlines())
                break
            if cur == root:
                break
            cur = cur.parent
    (case / 'ref.errors').write_text(''.join(x + '\n' for x in lines), encoding='utf-8', newline='\n')
    (case / 'EXPECT_EXIT').write_text(
        'official SUMO 1.28.0 exits non-zero (expected-error TextTest row)\n'
        'Official expectation: ' + rel + '/{errors,output}.netconvert\n',
        encoding='utf-8', newline='\n')
    (case / 'CASE.md').write_text(
        '# ' + name + '\n\nSource: `' + rel + '`\nOfficial netconvert exit code: ' + ec + '\n',
        encoding='utf-8', newline='\n')
    new_rows.append('row %s "args:%s" "%s"' % (name, ' '.join(inputs), ' '.join(outputs)))

with rowsfile.open('a', encoding='utf-8', newline='\n') as fh:
    for r in new_rows:
        fh.write(r + '\n')

print('imported', len(new_rows), 'leaves [%d:%d] of %d error-exit candidates' % (START, END, len(leaves)))
for r in new_rows:
    print(r.split()[1])

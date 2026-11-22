import os, shutil, re
from pathlib import Path
sys_path = 'tools/oracle'
import sys
sys.path.insert(0, sys_path)
import import_texttest as m

TESTS = Path('C:/Users/karti/sumo-src/release-1.28.0/tests/sumo')
LEAVES = [
    'basic/person/errors/duplicate_id',
    'basic/person/errors/invalid_busstop',
    'basic/person/errors/person_triggered_invalidFrom',
    'basic/person/errors/trainStop_car_disconnected',
]

def find_up(leaf, name):
    cur = leaf
    while True:
        p = cur / name
        if p.exists():
            return p
        if cur == TESTS or cur.parent == cur:
            return None
        cur = cur.parent

for rel in LEAVES:
    leaf = TESTS / rel
    name = 'tt_' + rel.replace('/', '_')
    case = Path(m.CASES) / name
    if case.exists():
        shutil.rmtree(case)
    case.mkdir(parents=True)
    # copy all files in the leaf
    for f in leaf.iterdir():
        if f.is_file():
            shutil.copy2(f, case / f.name)
    # resolve the net named in options and copy it as ref_simple.net.xml
    opts = (leaf / 'options.sumo').read_text(encoding='utf-8', errors='replace')
    netname = None
    mm = re.search(r'--net-file[= ]+([^\s]+)', opts)
    if mm:
        netname = mm.group(1)
    if netname:
        src = case / os.path.basename(netname)
        if not src.exists():
            found = find_up(leaf, os.path.basename(netname))
            if found:
                shutil.copy2(found, src)
                src = case / os.path.basename(netname)
        if src.exists():
            shutil.copy2(src, case / 'ref_simple.net.xml')
    # routes: copy the leaf route file to routes.rou if present
    for rn in ('input_routes.rou.xml',):
        if (case / rn).exists():
            shutil.copy2(case / rn, case / 'routes.rou')
            break
    # sim.args: option tokens minus harness flags and output files
    tokens = m.collate_options(m.collect_files_and_config(str(leaf), str(TESTS))[0], 'sumo')
    drop = set(['--no-step-log', '--no-duration-log'])
    sim, _, _, additions, _ = m.parse_file_options(tokens)
    sim = [t for t in sim if t not in drop]
    kept = []
    for a in additions:
        p = find_up(leaf, os.path.basename(a))
        if p:
            if not (case / os.path.basename(p)).exists():
                shutil.copy2(p, case / os.path.basename(p))
            kept.append(os.path.basename(p))
    if kept:
        sim += ['-a', ','.join(kept)]
    (case / 'sim.args').write_text(' '.join(sim) + '\n', encoding='utf-8', newline='\n')
    # expectations
    (case / 'ref.errors').write_text((leaf / 'errors.sumo').read_text(encoding='utf-8', errors='replace'), encoding='utf-8', newline='\n')
    (case / 'ref.output').write_text((leaf / 'output.sumo').read_text(encoding='utf-8', errors='replace') if (leaf / 'output.sumo').exists() else '', encoding='utf-8', newline='\n')
    (case / 'EXPECT_EXIT').write_text('official SUMO 1.28.0 exits non-zero (expected-error TextTest row)\nOfficial expectation: ' + rel + '/{errors,output}.sumo\n', encoding='utf-8', newline='\n')
    (case / 'CASE.md').write_text('# ' + name + '\n\nSource: `tests/sumo/' + rel + '`\n', encoding='utf-8', newline='\n')
    print('IMPORTED', name, 'net', (case/'ref_simple.net.xml').exists(), 'sim', ' '.join(sim)[:100])

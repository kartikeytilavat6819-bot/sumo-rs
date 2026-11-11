import sys


def fill(path, bodies):
    with open(path, encoding='utf-8', newline='') as f:
        src = f.read()
    nl = '\r\n' if '\r\n' in src else '\n'
    for marker, body in bodies.items():
        old = '        todo!("%s")' % marker
        n = src.count(old)
        if n != 1:
            raise SystemExit('marker %r found %d times in %s' % (marker, n, path))
        lines = []
        for l in body.strip('\n').split('\n'):
            lines.append(('        ' + l) if l.strip() else '')
        src = src.replace(old, nl.join(lines), 1)
    with open(path, 'w', encoding='utf-8', newline='') as f:
        f.write(src)
    print('ok', path, len(bodies))

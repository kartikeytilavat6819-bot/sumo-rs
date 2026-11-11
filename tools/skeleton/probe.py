import sys, clang.cindex as ci
SRC = r"C:\Users\karti\sumo-src\src"
idx = ci.Index.create()
tu = idx.parse(sys.argv[1], args=["-x", "c++", "-std=c++17", "-I", SRC, "-I", "fake_include", "-DHAVE_FOX=0"],
               options=ci.TranslationUnit.PARSE_SKIP_FUNCTION_BODIES | ci.TranslationUnit.PARSE_INCOMPLETE)
errs = [d for d in tu.diagnostics if d.severity >= 3]
print("errors:", len(errs)); [print(" ", d) for d in errs[:8]]
for c in tu.cursor.get_children():
    if c.location.file and c.location.file.name.endswith(sys.argv[1].split("\\")[-1]) and c.kind == ci.CursorKind.CLASS_DECL and c.is_definition():
        print("class", c.spelling, "bases:", [b.type.spelling for b in c.get_children() if b.kind == ci.CursorKind.CXX_BASE_SPECIFIER])
        for m in c.get_children():
            if m.kind in (ci.CursorKind.CXX_METHOD, ci.CursorKind.FIELD_DECL):
                print("  ", m.kind.name[:5], m.spelling, "|", m.type.spelling, "| virtual" if m.kind == ci.CursorKind.CXX_METHOD and m.is_virtual_method() else "")

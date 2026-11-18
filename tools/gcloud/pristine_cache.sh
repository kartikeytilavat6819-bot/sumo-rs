#!/bin/bash
# Gate VM helper: run an OFFICIAL SUMO binary once per distinct input and replay its outputs afterwards.
#   pristine_cache.sh <real-binary> <args...>
# The reference outputs of the official tools never change for the same inputs, but every gate used to
# regenerate them (netopts rows, duarouter cases). The output root is the directory of the -o/--output-file
# value; its path is stored as @ROOT@ so a replay into another row/slot directory is byte-identical.
# Key = binary identity + args (root-relative) + contents of every input file's directory.
real=$1; shift
C=${PCACHE_DIR:-/home/gate/pcache/entries}; mkdir -p "$C"
root=""; prev=""
for a in "$@"; do
  case "$prev" in -o|--output-file|--output) root=$(dirname "$a") ;; esac
  case "$a" in --output-file=*) root=$(dirname "${a#*=}") ;; esac
  prev=$a
done
[ -n "$root" ] && root=$(cd "$root" 2>/dev/null && pwd)
[ -z "$root" ] && exec "$real" "$@"          # nothing to capture: just run it

key=$( {
  echo "bin $real $(stat -c '%s %Y' "$real")"
  for a in "$@"; do
    echo "arg ${a//$root/@ROOT@}"
    IFS=',' read -ra parts <<< "${a#*=}"
    for p in "${parts[@]}"; do
      [ -f "$p" ] || continue
      case "$(readlink -f "$p")" in "$root"/*) continue ;; esac
      find "$(dirname "$p")" -maxdepth 1 -type f -size -50M -exec sha1sum {} + 2>/dev/null | awk '{print $1}' | sort
    done
  done
} | sha1sum | cut -c1-40)
E=$C/$key

if [ -f "$E/rc" ]; then                       # replay
  (cd "$E/files" && find . -type f) | while IFS= read -r f; do
    mkdir -p "$root/$(dirname "$f")"
    if grep -Iq . "$E/files/$f" 2>/dev/null; then sed "s#@ROOT@#$root#g" "$E/files/$f" > "$root/$f"; else cp "$E/files/$f" "$root/$f"; fi
  done
  sed "s#@ROOT@#$root#g" "$E/stdout"; sed "s#@ROOT@#$root#g" "$E/stderr" >&2
  touch "$E/rc"
  exit "$(cat "$E/rc")"
fi

mark=$(mktemp); out=$(mktemp); err=$(mktemp)
"$real" "$@" > "$out" 2> "$err"; rc=$?
T=$(mktemp -d "$C/.tmp.XXXXXX"); mkdir -p "$T/files"
(cd "$root" && find . -type f -newer "$mark") | while IFS= read -r f; do
  mkdir -p "$T/files/$(dirname "$f")"
  if grep -Iq . "$root/$f" 2>/dev/null; then sed "s#$root#@ROOT@#g" "$root/$f" > "$T/files/$f"; else cp "$root/$f" "$T/files/$f"; fi
done
sed "s#$root#@ROOT@#g" "$out" > "$T/stdout"; sed "s#$root#@ROOT@#g" "$err" > "$T/stderr"; echo $rc > "$T/rc"
mv -T "$T" "$E" 2>/dev/null || rm -rf "$T"    # another slot may have stored it first
cat "$out"; cat "$err" >&2
rm -f "$mark" "$out" "$err"
exit $rc

#!/bin/bash
# Keep C: from filling up (user's order): every 20 min, if free space < MIN_GB, remove agent worktrees that are
# (1) merged into main, (2) clean (git refuses otherwise), (3) untouched for 3 h (protects freshly created ones,
# which are trivially "merged" before their first commit). Branches stay, so any worktree can be re-created.
#   bash tools/lead/disk_keeper.sh      (run in background)
REPO=/c/Users/karti/sumo-rs MIN_GB=${MIN_GB:-60}
free_gb() { df -BG /c | awk 'NR==2{gsub("G","",$4); print $4}'; }
while true; do
  if [ "$(free_gb)" -lt "$MIN_GB" ]; then
    n=0
    git -C "$REPO" worktree list --porcelain | awk '/^worktree /{print $2}' | grep '/.claude/worktrees/agent-' > /tmp/dk_wts
    while read -r wt; do
      [ "$(free_gb)" -ge "$MIN_GB" ] && break
      git -C "$REPO" merge-base --is-ancestor "$(git -C "$wt" rev-parse HEAD 2>/dev/null)" main 2>/dev/null || continue
      idx=$(git -C "$wt" rev-parse --git-dir 2>/dev/null)/index
      [ -n "$(find "$idx" -mmin -180 2>/dev/null)" ] && continue
      [ -n "$(find "$wt" -path "$wt/target" -prune -o -type f -mmin -180 -print -quit 2>/dev/null)" ] && continue
      git -C "$REPO" worktree remove "$wt" 2>/dev/null && n=$((n+1))
    done < /tmp/dk_wts
    git -C "$REPO" worktree prune
    echo "$(date +%H:%M) removed $n worktrees, free $(free_gb) GB"
  fi
  # stage 2 (user: "if there is still less space you can use drive d"): move idle worktrees to D: and leave a
  # junction at the old path, so git and agents keep the same path
  if [ "$(free_gb)" -lt "${D_GB:-30}" ]; then
    m=0
    git -C "$REPO" worktree list --porcelain | awk '/^worktree /{print $2}' | grep '/.claude/worktrees/agent-' > /tmp/dk_wts
    while read -r wt; do
      [ "$(free_gb)" -ge "${D_GB:-30}" ] && break
      id=${wt##*/}
      [ -n "$(find "$wt" -maxdepth 0 -type l 2>/dev/null)" ] && continue          # already a junction
      powershell.exe -NoProfile -c "if ((Get-Item -LiteralPath '$(cygpath -w "$wt")').LinkType) { exit 3 }" || continue
      idx=$(git -C "$wt" rev-parse --git-dir 2>/dev/null)/index
      [ -n "$(find "$idx" -mmin -180 2>/dev/null)" ] && continue
      [ -n "$(find "$wt" -path "$wt/target" -prune -o -type f -mmin -180 -print -quit 2>/dev/null)" ] && continue
      W=$(cygpath -w "$wt"); D="D:\\sumo-worktrees\\$id"
      powershell.exe -NoProfile -c "robocopy '$W' '$D' /E /MOVE /NFL /NDL /NJH /NJS /R:1 /W:1 | Out-Null; if (\$LASTEXITCODE -ge 8) { exit 1 }; if (Test-Path -LiteralPath '$W') { Remove-Item -LiteralPath '$W' -Recurse -Force }; New-Item -ItemType Junction -Path '$W' -Target '$D' | Out-Null" && m=$((m+1))
    done < /tmp/dk_wts
    echo "$(date +%H:%M) moved $m idle worktrees to D:, C: free $(free_gb) GB"
  fi
  sleep 1200
done

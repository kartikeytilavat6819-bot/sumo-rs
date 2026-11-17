# Unmerged branches (inventory, 2026-11-16)

Measured with:

```bash
git for-each-ref --format='%(refname:short)' refs/heads | grep -E 'worktree-agent-|fix-|local-fill'
# then, per branch:  git rev-list --count main..BR  and  git diff --shortstat main...BR
```

**51 branches carry commits that are not on `main`.** They are all from porting waves that ran before the
verification loop in `LOOP.md` existed, so they were never gate-checked. Nothing here is known to be
superseded, but nothing is known to be needed either — treat this as an inventory, not a to-do list.

Largest by insertion count:

| Branch | Commits | Insertions | Last commit subject |
|---|---|---|---|
| `fix-a498d714ba555444f` | 1 | 13,523 | Merge branch 'worktree-agent-a498d714…' |
| `fix-a2404ac0bfd4c7fc0` | 1 | 13,523 | Merge branch 'worktree-agent-a2404ac0…' |
| `fix-a60ca9556cae1357c` | 1 | 8,100 | Merge branch 'worktree-agent-a60ca955…' |
| `worktree-agent-a71db91b240aab912` | 19 | 3,007 | **WIP: paused at usage limit (uncommitted agent edits, not compiled)** |
| `worktree-agent-a2e3b61f8513670ad` | 15 | 2,394 | Fill rodf_net (part 6) |
| `worktree-agent-aefe4385a3be9513c` | 15 | 2,393 | WIP: paused at usage limit |
| `worktree-agent-a037eefaa380ba9a6` | 6 | 2,291 | WIP: paused at usage limit |
| `fix-ab5de0c78731e6d1c` | 1 | 1,671 | Merge branch … |
| `fix-a378aaf1aee25a2df` | 1 | 1,416 | Merge branch … |
| `worktree-agent-ae38ef9020b511f9b` | 7 | 1,373 | WIP: paused at usage limit |

## How to triage one of these safely

1. `git diff --shortstat main...BRANCH` — if it is empty, the content is already in `main`; delete the branch.
2. `git diff main...BRANCH -- <file>` for a file the branch touches. If `main` has moved on (most of these
   predate the 2026-11-16 work), the interesting question is whether the *specific bodies* are missing, not
   whether the diff applies.
3. The cheapest check that actually means something: take the branch's file list, `grep` for `todo!(` /
   `panic!("shortcut` in those files on `main`, and see whether the branch fills any of them. A branch that
   no longer contains anything `main` lacks is finished, whatever its diff says.
4. Never merge one of these wholesale. They predate the release/UB work and the gate; a merge would be
   unreviewed bulk code. Port forward by hand, file by file, with the gate as the referee.

## Why this file exists

Four agents tonight reported a headline number that was not in the artefact it referenced, and two branches
were nearly lost because a session ended with work uncommitted. Unmerged branches are the same failure at a
larger scale: work that exists but that nobody is counting. This inventory makes it countable.

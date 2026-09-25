---
name: align-upstream
description: >-
  Aligns this agent-desktop fork with lahfir/agent-desktop main while preserving
  local additions, then rebuilds and verifies the installed CLI. Use for
  "sync upstream", "align with remote", "rebase local additions", "update
  fork main", or "rebuild agent-desktop after upstream changes".
---

# Align agent-desktop with upstream

For a plain-language explanation of branch dependencies and saved pointers,
see [README.md](README.md). The commands and verification gates live in
[references/sync-and-smoke.md](references/sync-and-smoke.md).

**When to use:** A local feature stack needs the latest `upstream/main`, or a
fresh local binary must be qualified after syncing. For one-off UI automation,
use the bundled `skills/agent-desktop/SKILL.md` instead.

**Inspect:** Read the sync-and-smoke reference before changing refs. Check
cleanliness, remotes, upstream tips, branch graph,
and install destination; preserve unrelated changes.

**Align:** Record actual local branch dependencies as symbolic Git refs and
capture pre-sync tips before moving anything. Fast-forward local `main` from
`upstream/main`, update fork `origin/main`, then move independent branches and
stacks in parent-before-child order. Use `--force-with-lease` only for
published branches after verifying their remote tips. Never force-push `main`
or `upstream`.

**Verify:** Run owning tests, `just install-local` using the environment-selected
destination, permissions and core AX smoke per the reference. Consult the
bundled `skills/agent-desktop/SKILL.md` for current snapshot/refs semantics.
If permissions need interactive approval, run `permissions --request` and ask
the user to confirm the OS grant via the question tool before rechecking.
Record observable pass/fail, not a claimed UI pass from a mere successful exit.

**Success:** Fork `main` equals upstream, every affected local branch preserves
its changes on the new base, dependency refs still point to the intended local
parents, the installed executable is from the rebased HEAD, and
permissions plus a real snapshot have been checked. Report hashes, pushes,
install path, smoke evidence, and any unresolved permission gate.

**Test / improve:** Exercise this workflow against a real upstream advance and
the representative smoke. If a branch, permission, or capture assumption fails,
update the reference with the observed correction before the next sync.

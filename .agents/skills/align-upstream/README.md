# How upstream alignment moves branches

This skill keeps the personal fork's `main` in step with `upstream/main`, then
moves local feature branches onto that updated base. It records two different
kinds of **local Git refs** so it knows what to move and can recover if a rebase
goes wrong. These refs are not pushed to GitHub.

## A dependency names a parent branch

Suppose `feature/B` was created from `feature/A`:

```text
main ── feature/A ── feature/B
          parent        child
```

The skill records this relationship as a *symbolic ref*:

```text
refs/align-upstream/deps/feature/B
    → refs/heads/feature/A
```

It points to the **branch name**, not a particular commit. When `feature/A`
moves, the dependency still identifies A as B's parent. The skill checks that
B really descends from A before recording the relationship; a shared file tree
or similar branch name is not proof of dependency.

## Saved tips remember the old commits

Before moving branches, the skill saves each old tip, including `main`:

```text
refs/align-upstream/pre-sync/main       → old main commit
refs/align-upstream/pre-sync/feature/A  → old A commit
refs/align-upstream/pre-sync/feature/B  → old B commit
```

Unlike a dependency ref, each saved tip points to a **specific old commit**.
It preserves a recovery point and lets the skill inspect where a branch
actually forked. A branch may have forked *before* its parent's old tip;
assuming the tip was its starting point could silently skip commits.

## The order of a sync

```text
Before                                   After

upstream/main ── U0                      upstream/main ── U1
                  │                                        │
main ──────────────┘                      main ──────────────┘
  └─ feature/A ── A0                        └─ feature/A ── A1
       └─ feature/B ── B0                        └─ feature/B ── B1
```

The skill first fast-forwards `main` to the new upstream tip and pushes fork
`main`. It then finds A's old fork point and rebases A onto the updated main;
only after A moves does it rebase B onto the updated A. It checks that the
patches survived, builds and smoke-tests the installed CLI, and uses
lease-checked pushes for published feature branches. Saved tips remain local
recovery pointers until the sync has been verified.

**An example from this fork:** after the workflow commits were cherry-picked
onto `feature/local-additions`, it and `feature/local-build-workflow` had
equivalent changes but divergent commit histories. They were independent at
that point: no dependency ref should be invented merely because trees match.
Always inspect the live graph at the next sync.

For the operational commands and verification checklist, see
[references/sync-and-smoke.md](references/sync-and-smoke.md).

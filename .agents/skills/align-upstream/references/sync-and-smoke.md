# Sync and desktop smoke

## Preflight and branch alignment

- Require clean worktrees; inspect `git worktree list`, `git status --short`,
  `git remote -v`, `git branch -vv`, and the graph before rebasing.
- Confirm `upstream` is `lahfir/agent-desktop`, `origin` is the personal fork,
  and the remote default branch from refs (do not infer it from a name).
  Set `CI=true` for every git operation.
- Fetch both remotes. Record current hashes for `main`, the local branches,
  `origin/main`, and any published feature refs. If fork main diverged from
  upstream, stop and reconcile it rather than replacing commits.
- Fast-forward local `main` from `upstream/main`; push it to `origin/main`
  normally. Rebase the bottom feature branch onto `main`, then each child
  onto its updated parent using `git rebase --onto <new-parent> <old-parent>`.
  Check `git range-diff`/diff and tests after conflict resolution.
- Push rebased published feature branches in dependency order with explicit
  `--force-with-lease=refs/heads/<branch>:<recorded-remote-hash>` to `origin`.
  A changed remote tip means stop, fetch, and investigate; never disable hooks.
- At most three failed fix attempts per issue. Keep old hashes as recovery
  pointers and report unresolved conflicts rather than dropping commits.

## Build and verify

- Use the repo-pinned Rust toolchain; run `cargo fmt --all -- --check`,
  `cargo test -p agent-desktop-macos`, and relevant checks for changed files.
- Load `.envrc`/ignored `private.env` with direnv, or export
  `AGENT_DESKTOP_INSTALL_PATH`; check that it is the intended CLI path.
  Run `just install-local` and verify `command -v agent-desktop`, the installed
  file, and `agent-desktop --version`.
- Run `agent-desktop permissions` and inspect Accessibility, Screen Recording,
  and Automation states. Run `agent-desktop permissions --request` when a
  required grant is missing; ask the user via the question tool to approve in
  macOS Settings if needed, then recheck. Do not claim a grant based solely on
  the request command's return code.
- Follow `skills/agent-desktop/SKILL.md` for observe/ref semantics. Use a
  running, non-sensitive app such as Finder: `agent-desktop list-apps`,
  `agent-desktop list-windows --app Finder`, and
  `agent-desktop snapshot --app Finder -i`. Require `ok: true`, a complete
  snapshot, and identifiable elements. If Finder has no window, open a
  disposable Finder window or choose another known app and record the target;
  do not treat `WINDOW_NOT_FOUND` as a pass. Re-snapshot after any UI change.
- For Screen Recording, capture a visible screen/window to an ignored local
  file and inspect the PNG for actual content. For changes affecting window
  inventory/correlation, additionally snapshot the affected app/window; a
  Finder-only smoke cannot qualify a floating-window regression.

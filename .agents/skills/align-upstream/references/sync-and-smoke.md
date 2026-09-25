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
- Enumerate **all affected local branches**, including branches not checked
  out, and inspect ancestry plus `refs/align-upstream/deps/`. For each genuine
  child branch, record its parent by ref name, not an old SHA:
  `git symbolic-ref refs/align-upstream/deps/<child> refs/heads/<parent>`.
  These are local symbolic refs; do not push them. Validate each target exists,
  the child actually descends from the parent, and the graph has no cycle.
  Equivalent trees on divergent sibling branches are **not** a stack; keep
  them independent unless a real dependency is established.
- Before moving any branch, save its old tip with
  `git update-ref refs/align-upstream/pre-sync/<branch> refs/heads/<branch>`
  (including `main`). Confirm each saved SHA and keep it until the sync is
  verified; check before overwriting a previous recovery ref. Save published
  remote tip hashes separately for the push leases.
- Fast-forward local `main` from `upstream/main`; push it to `origin/main`
  normally. For each independent branch, compute its actual old fork point
  against `refs/align-upstream/pre-sync/main` and run
  `git rebase --onto main <old-fork-point> <branch>`. Move every stacked child
  after its parent: compute its old fork point against
  `refs/align-upstream/pre-sync/<parent>` and run
  `git rebase --onto <parent> <old-fork-point> <child>`. Inspect merge bases
  before choosing them, especially for branches cut before the last sync;
  blindly using the saved parent tip can drop commits. Symbolic dependency
  refs continue following moved parents. Check `git range-diff`/diff and tests
  after conflict resolution.
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
  macOS Settings if needed, then recheck. `unknown` Automation can mean the
  requesting helper has no established TCC record; macOS may show the parent
  terminal as the controller instead. Record it as unverified unless an actual
  Automation-dependent operation succeeds; do not claim a grant based solely
  on the request command's return code or a parent terminal's toggle.
- Follow `skills/agent-desktop/SKILL.md` for observe/ref semantics. Use a
  running, non-sensitive app such as Finder: `agent-desktop list-apps`,
  `agent-desktop list-windows --app Finder`, and
  `agent-desktop snapshot --app Finder -i`. Require `ok: true`, a complete
  snapshot, and identifiable elements. If Finder has no window, open a
  disposable Finder window or choose another known app and record the target;
  for a dense app use `--skeleton -i --compact` and require `complete: true`.
  Do not treat `WINDOW_NOT_FOUND` or a truncated full tree as a pass.
  Re-snapshot after any UI change.
- For Screen Recording, capture a visible screen/window to an ignored local
  file and inspect the PNG for actual content. For changes affecting window
  inventory/correlation, additionally snapshot the affected app/window; a
  Finder-only smoke cannot qualify a floating-window regression.

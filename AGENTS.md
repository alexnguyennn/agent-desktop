# Local development

- For requests to update the fork from upstream, rebase local additions, or
  rebuild after upstream changes, load `.agents/skills/align-upstream/SKILL.md`
  and follow its linked sync-and-smoke workflow.
- To exercise changes through a stable local CLI, set
  `AGENT_DESKTOP_INSTALL_PATH` in the ignored `private.env` at the repository
  root. `.envrc` loads that file through direnv; allow it locally with
  `direnv allow`.
- Run `just install-local` to build the release CLI and install it at
  that environment-provided path. The install destination must not be hardcoded
  in task recipes or committed configuration.
- Set `AGENT_DESKTOP_LOCAL_BUILD=1` in ignored `private.env` to embed
  `<package-version>+local.<short-commit>` (plus `.dirty` if applicable) in
  the installed CLI's version reports; ordinary Cargo/release builds keep the
  package version.
- Do not commit `private.env`; it can contain machine-specific paths or local
  credentials. The task accepts the same variable directly from the shell when
  direnv is not in use.

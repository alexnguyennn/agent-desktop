set shell := ["bash", "-euo", "pipefail", "-c"]

# Build and install this checkout's release CLI at the environment-selected path.
install-local:
    #!/usr/bin/env bash
    set -euo pipefail
    : "${AGENT_DESKTOP_INSTALL_PATH:?Set AGENT_DESKTOP_INSTALL_PATH in private.env or the environment}"
    case "${AGENT_DESKTOP_LOCAL_BUILD:-0}" in
      1)
        package_id=$(cargo pkgid -p agent-desktop)
        version="${package_id##*@}"
        sha=$(git rev-parse --short=12 HEAD)
        dirty=""
        [[ -z "$(git status --porcelain)" ]] || dirty=".dirty"
        export AGENT_DESKTOP_BUILD_VERSION="${version}+local.${sha}${dirty}"
        ;;
      0) unset AGENT_DESKTOP_BUILD_VERSION ;;
      *) echo "AGENT_DESKTOP_LOCAL_BUILD must be 0 or 1" >&2; exit 2 ;;
    esac
    cargo build --release --bin agent-desktop
    mkdir -p "$(dirname "$AGENT_DESKTOP_INSTALL_PATH")"
    install -m 0755 target/release/agent-desktop "$AGENT_DESKTOP_INSTALL_PATH"
    "$AGENT_DESKTOP_INSTALL_PATH" --version

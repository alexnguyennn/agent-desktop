set shell := ["bash", "-euo", "pipefail", "-c"]

# Build and install this checkout's release CLI at the environment-selected path.
install-local:
    test -n "${AGENT_DESKTOP_INSTALL_PATH:-}" || { echo "Set AGENT_DESKTOP_INSTALL_PATH in private.env or the environment" >&2; exit 2; }
    cargo build --release --bin agent-desktop
    mkdir -p "$(dirname "$AGENT_DESKTOP_INSTALL_PATH")"
    install -m 0755 target/release/agent-desktop "$AGENT_DESKTOP_INSTALL_PATH"
    "$AGENT_DESKTOP_INSTALL_PATH" --version

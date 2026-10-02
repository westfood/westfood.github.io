#!/usr/bin/env bash
set -euo pipefail

usage() {
    printf 'Usage: %s [port]\nDefault: PORT environment variable, or 8000.\n' "$0"
}

if [[ $# -gt 1 ]]; then
    usage >&2
    exit 2
fi

if [[ ${1-} == --help || ${1-} == -h ]]; then
    usage
    exit 0
fi

preview_port_input=${1-${PORT:-8000}}
if [[ ! $preview_port_input =~ ^[0-9]{1,5}$ ]]; then
    printf 'Error: port must be a number from 1 to 65535.\n' >&2
    exit 2
fi
preview_port=$((10#$preview_port_input))
if (( preview_port < 1 || preview_port > 65535 )); then
    printf 'Error: port must be a number from 1 to 65535.\n' >&2
    exit 2
fi

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
if [[ ! -f "$repo_root/docs/index.html" ]]; then
    printf 'Error: the website is missing: %s/docs/index.html\n' "$repo_root" >&2
    exit 1
fi

if ! command -v uv >/dev/null 2>&1; then
    printf 'Error: uv is required. Install it from https://docs.astral.sh/uv/getting-started/installation/\n' >&2
    exit 1
fi

# Honor Rudolf's external build and artifact storage policy on his Mac.
if [[ $(uname -s) == Darwin && -d /Users/rudolf ]]; then
    storage_env=/Users/rudolf/.config/building-storage/env.sh
    storage_check=/Users/rudolf/.local/bin/building-check
    if [[ ! -r "$storage_env" || ! -x "$storage_check" ]]; then
        printf 'Error: Building storage setup is missing. Restore env.sh and building-check before previewing.\n' >&2
        exit 1
    fi
    source "$storage_env"
    "$storage_check"
fi

export PYTHONDONTWRITEBYTECODE=1
printf 'Local preview: http://127.0.0.1:%s\nPress Ctrl+C to stop.\n' "$preview_port"
exec uv run --no-project --no-config --no-python-downloads python -m http.server \
    "$preview_port" --bind 127.0.0.1 --directory "$repo_root/docs"

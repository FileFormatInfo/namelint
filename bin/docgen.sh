#!/usr/bin/env bash
#
# generate the documentation for namelint binary

set -o errexit
set -o pipefail
set -o nounset

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
REPO_DIR="$( cd "${SCRIPT_DIR}/.." && pwd )"

echo "INFO: docgen started at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

DATA_DIR="${REPO_DIR}/docs/src/data"
if [ ! -d "${DATA_DIR}" ]; then
	echo "INFO: creating '${DATA_DIR}' directory!"
	mkdir -p "${DATA_DIR}"
else
	echo "INFO: using existing '${DATA_DIR}' directory!"
fi

# build the app
cargo build --bin namelint --release

# if it worked, the app will be in ./target/release
${REPO_DIR}/target/release/namelint --docs | jq . > "${DATA_DIR}/usage.json"

echo "INFO: docgen complete at $(date -u +%Y-%m-%dT%H:%M:%SZ)"

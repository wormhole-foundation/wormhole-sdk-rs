#!/bin/bash

set -euo pipefail

cargo publish -p wormhole-io --manifest-path universal/Cargo.toml $@
cargo publish -p wormhole-raw-vaas --manifest-path universal/Cargo.toml $@
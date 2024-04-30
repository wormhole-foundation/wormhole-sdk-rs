#!/bin/bash

set -euo pipefail

cargo publish -p wormhole-io $@
cargo publish -p wormhole-raw-vaas --features ruint $@
cargo publish -p wormhole-solana-consts --features mainnet $@
cargo publish -p wormhole-solana-utils --features anchor $@
cargo publish -p wormhole-solana-vaas --features anchor,mainnet $@
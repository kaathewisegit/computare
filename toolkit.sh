set -e

check() {
	cargo fmt --check
	cargo check --tests
	cargo clippy --tests
	cargo test
}

if [ "$1" = "check" ]; then
	check
else
	echo "Usage: $0 check" >&2
	exit 1
fi

set -e

check() {
	local flags="$1"
	echo "# ${flags}"

	cargo check --tests ${flags}
	cargo clippy --tests ${flags}
	cargo test ${flags}
}

if [ "$1" = "check" ]; then
	cargo fmt --check

	check "--no-default-features"
	check "--no-default-features --features libm"
	check "--no-default-features --features std"
	check "--no-default-features --features libm,std"
else
	echo "Usage: $0 check" >&2
	exit 1
fi

set -e

check() {
	local flags="$1"
	echo "# ${flags}"

	cargo check --tests ${flags}
	cargo clippy --tests ${flags}
	cargo test --no-default-features ${flags}
}

case "$1" in
	"check")
		cargo fmt --check

		check ""
		check " --features libm"
		check " --features std"
		check " --features libm,std"
		check " --features rand"
		check " --features rand,std"
		;;
	"fuzz")
		if [ -n "$2" ]; then
			export ARBTEST_BUDGET_MS="$2"
		fi

		cargo test --release
		;;
	*)
		echo "Usage: $0 {check|fuzz}" >&2
		exit 1
		;;
esac

.PHONY: audit-deps
audit-deps:
	cargo audit

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets --all-features --examples

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --all-features --examples 
	cargo rustdoc --all-features -- -D warnings

.PHONY: doc
doc:
	cargo doc --all-features --open &

.PHONY: format
fmt:
	cargo lint
	cargo fmt

.PHONY: run-embeded
run-embeded:
	@cargo build --examples && ./scripts/xephyr.sh

.PHONY: test
test:
	cargo test --lib

.PHONY: test-and-publish
test-and-publish:
	cargo test --all-features && cargo publish

.PHONY: upgrade-check
upgrade-check:
	cargo upgrade --workspace --dry-run

.PHONY: todo
todo:
	rg 'TODO|FIXME|todo!' crates src


# GitHub helpers using the official gh GitHub CLI
.PHONY: list-issues
list-issues:
	gh issue list

.PHONY: list-prs
list-prs:
	gh pr list

.PHONY: new-issue
new-issue:
	gh issue create

.PHONY: pr
pr:
	gh pr create

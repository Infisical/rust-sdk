test:
	cargo test --all-targets --all-features -- --ignored --nocapture

reviewable:
	cargo fmt && \
	cargo clippy --all-targets --all-features --workspace -- -D warnings && \
	$(MAKE) test

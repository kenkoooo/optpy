.PHONY: test
test:
	cargo test --workspace -- --test-threads 1

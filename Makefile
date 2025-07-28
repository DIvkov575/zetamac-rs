.PHONY: install release-crates-patch release-crates-minor release-crates-major generate-brew-formulae push-brew-formulae release-patch release-minor release-major release

install:
	echo "Running install ..."
	cargo install cargo-release

release-crates-patch:
	echo "Running release-crates-patch..."
	- cargo release patch --execute

release-crates-minor:
	echo "Running release-crates-minor..."
	- cargo release minor --execute

release-crates-major:
	echo "Running release-crates-major ..."
	- cargo release minor --execute

generate-brew-formulae:
	echo "Running generate-brew-formulae ..."
	@VERSION=$$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'); \
	echo "cargo project version: $$VERSION"; \
	curl -L -o "zetamac-rs-$$VERSION.tar.gz" "https://github.com/divkov75/zetamac-rs/archive/refs/tags/$$VERSION.tar.gz"; \
	SHA256=$$(shasum -a 256 "zetamac-rs-$$VERSION.tar.gz" | cut -d ' ' -f1); \
	echo "shaaa shaaa $$SHA256"; \
	sed -e "s/{{version}}/$$VERSION/g" -e "s/{{sha256}}/$$SHA256/g" zetamac_template.rb > homebrew/zetamac.rb; \
	rm "zetamac-rs-$$VERSION.tar.gz"

push-brew-formulae:
	echo "Running push-brew-formulae ..."
	@VERSION=$$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'); \
	echo "cargo project version: $$VERSION"; \
	cd homebrew; \
	git commit -am "release version $$VERSION"; \
	git push

release-patch: release-crates-patch generate-brew-formulae push-brew-formulae
release-minor: release-crates-minor generate-brew-formulae push-brew-formulae
release-major: release-crates-major generate-brew-formulae push-brew-formulae

release: release-minor

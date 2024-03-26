.PHONY:
commit: generate
	@echo "Committing changes..."
	# check if there are any changes, if not, do nothing, otherwise, commit and push
	@if [ -z "$$(git status --porcelain)" ]; then \
		echo "No changes to commit."; \
	else \
		git add .; \
		git commit -m "."; \
		git push; \
	fi

.PHONY: check_mdi
check_mdi:
	@echo "Checking if mdi is installed..."
	@if ! command -v mdi &> /dev/null; then \
		echo "mdi is already installed"; \
	fi

.PHONY: generate
generate: check_mdi
	@git pull
	@echo "Generating README.md..."
	@mdi gen -f README.md -t "Learning Rust" -r --override --nav -v

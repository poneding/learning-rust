.PHONY:
# commit: generate cargo-sort
commit: cargo-sort
	@echo "Pulling latest changes..."
	@git pull
	@echo "Committing changes..."
	# check if there are any changes, if not, do nothing, otherwise, commit and push
	@if [ -z "$$(git status --porcelain)" ]; then \
		echo "No changes to commit."; \
	else \
		git add .; \
		git commit -m "."; \
		git push; \
	fi

# .PHONY: check_mdi
# check_mdi:
# 	@echo "Checking if mdi is installed..."
# 	@which mdi > /dev/null && \
# 	 echo "mdi is already installed" || \
# 	  (echo "mdi is not installed, installing now..."; \
# 	  go install github.com/poneding/mdi@latest && \
# 	  echo "mdi installed successfully" || \
# 	  echo "mdi installation failed, please try again later.")

# .PHONY: generate
# generate: check_mdi
# 	@git pull
# 	@echo "Generating README.md..."
# 	@mdi gen -f README.md -t "Learning Rust" -r --override --nav -v

.PHONY: cargo-sort
cargo-sort:
	@which cargo-sort > /dev/null || \
		(echo "installing cargo-sort..."; \
		cargo install cargo-sort)
	@cargo sort
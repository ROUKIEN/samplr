test: ## create a simple sample from a 60K lines csv file
	cargo run -- tests/input/60K_lines.csv 100

test-huge:
	cargo run -- tests/input/archive/Answers.csv 1000 > /tmp/result.csv

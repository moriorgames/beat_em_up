# Beat em UP Game with Rust 

This project is a Beat em UP Game with combination of Rust.

## Quick Start

Here is how you can get the project up and running:

```
# Clone the repository
$ git clone <repo_url>

# Change into project directory
$ cd beat_em_up

# Run the project
$ cargo run

# Test the project
$ cargo test
```

## Testing and Code Coverage

You can run the test suite and generate a code coverage report with these commands:

```
# Install the code coverage tool
$ cargo install cargo-tarpaulin

# In order to process code coverage without struggling with Arm64 arch I have created a workaround with docker
# Run the code coverage tool
$ docker-compose build
$ docker-compose up -d
$ docker logs -f coverage
$ docker cp coverage:/app/target/debug/tarpaulin/tarpaulin-report.html .

# Open the code coverage report on Google Chrome
$ open -a "Google Chrome" ./tarpaulin-report.html

# Open the code coverage report on Google Chrome
$ open -a "Google Chrome" ./target/debug/tarpaulin/tarpaulin-report.html

# Test a single file
$ cargo test --package beat_em_up --bin beat_em_up -- some::folder::first::tests --nocapture 

# Test a single integration test
$ cargo test --package beat_em_up --test some_file_on_tests_folder -- tests --nocapture
```

## Tooling

This utility command can be used to print file contents for exploration:

```
$ find . -type f -exec printf '### START OF FILE ###\n%s\n' {} \; -exec cat {} \; -exec printf '### END OF FILE ###\n' \;
```

## Modules

```  
$ cargo install cargo-modules
$ cargo modules generate tree
$ cargo modules generate tree -types
$ cargo modules generate tree --types --lib
$ cargo modules generate tree --types --bin beat_em_up
```

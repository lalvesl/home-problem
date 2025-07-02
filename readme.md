# CDP - Core Data Platform

![coverage](https://gitlab.weg.net/davm/core-data-processing-platform/cdpp/badges/main/coverage.svg?job=coverage&min_medium=55)
![pipeline](https://gitlab.weg.net/davm/core-data-processing-platform/cdpp/badges/main/pipeline.svg)
\
This is a platform for provide a core data for another applications,
like: SALES-HUB, SRM, SCA, WPM, WegFinance and another dashborads (Tableau and PowerBI).
Input of data are EASY-OracleDB, SAP-SOAP, SAP-JOBS, SMART-API and iFlow-PostgresDB.
Output of data are:
- PostgreSQL, avm-cdp, only for analysis (Tableau and PowerBI);
- PostgreSQL, Sales-HUB;
- PostgreSQL, SRM - Sales Routine Management;
- PostgreSQL, SCA - Sales Custom Automation;
- PostgreSQL, WPM - Weg Price Management;
- PostgreSQL, WegFinance;
- Restful API.

## Documentation of Restful APIs

- [swagger](https://cdp.weg.net/swagger-ui/).
- [redoc](https://cdp.weg.net/redoc).

## For Development

For development you need to set your environment variables
.env file with .env.example and:

### Unix Like Systems

It's not all FLAKES (for now) <small><small>If you have time, convert to the flake format, more performance and flexible.</small></small> 

ONLY install [`nix btw tools`](https://nixos.org/download/) and run ONLY:

```properties
nix-shell
```

All dependencies will be installed, ORM configurations and code generation.

For run the project, build, tests, test-coverage and benchmarks, run respectively:

```properties
cargo run;  # For running application
cargo build --release;  # For build with optimizations
test --workspace -- --include-ignored;    # Run unitary tests
cargo tarpaulin --verbose --workspace --ignored --out xml;  # Get the percentage of test coverage
cargo bench --workspace --color always --nocapture -- --include-ignored;    # Run performance benchmarks tests
```

For build the container image, run:

```properties
nix-shell --run "cargo build --release"
nix-build build.nix

# Load your image on container runner:
podman load -i release
```

### Dos Like Systems

Install the following tools:

- [rust](https://www.rust-lang.org/tools/install)
- [node](https://nodejs.org/en/download/)

Before start the project run:

```properties
# try to use bash from git-bash or git-cmd, i don't know...
./ci-cd/generate_schema.sh
```

For generate the schema of the databases.

Try to run the project with this command (below) and find the rest of dependencies. Good luck!!!

```properties
cargo run
```

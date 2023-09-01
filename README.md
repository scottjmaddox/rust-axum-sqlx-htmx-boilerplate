# rust-axum-sqlx-htmx-boilerplate

Work in progress boilerplate for a website with Rust/Axum/SQLx backend and HTMX frontend.

## Table of Contents

- [Running](#running)
- [Developing](#developing)
    - [Adding Migrations](#adding-migrations)
- [Contributing](#contributing)
- [License](#license)

## Running

TODO

```
cargo run
```

## Developing

### Adding Migrations

1. Install the SQLx command line tool:

    ```
    cargo install sqlx-cli
    ```

1. Use it to create a new migration script:

    ```
    cd backend
    sqlx migrate add -r <DESCRIPTION>
    ```

TODO: preparation for SQLx offline mode

```
sqlx database create
sqlx migrate run
cargo run
```

```
sqlx database drop
```

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as below, without any
additional terms or conditions.

## License

Copyright &copy; 2023 Scott J Maddox

Licensed under the BSD Zero Clause License. See [LICENSE file](LICENSE.md) in
the project root, or https://opensource.org/licenses/0BSD for full license
information.

The [SPDX](https://spdx.dev) license identifier for this project is `0BSD`.

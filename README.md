# rust-axum-sqlx-htmx-boilerplate

Work in progress boilerplate for a website with Rust/Axum/SQLx backend and HTMX frontend.

The example application is based on the one in https://hypermedia.systems

## Table of Contents

- [Running](#running)
- [Developing](#developing)
- [Contributing](#contributing)
- [License](#license)

## Running

1. [Install `rustup`](https://www.rust-lang.org/tools/install).

2. Use `rustup` to install the latest stable Rust toolchain:

    ```
    rustup update stable
    ```

1. Use `cargo` to build and run the server:

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
    sqlx migrate add -r <DESCRIPTION>
    ```

1. Migrate the database:

    ```
    sqlx migrate run
    ```

### Saving SQLx `query!` metadata to enable building without access to the database

1. Recreate and migrate the database:

    ```
    sqlx database drop -y && sqlx database create && sqlx migrate run
    ```

1. Save query metadata for offline usage:

    ```
    cargo sqlx prepare --workspace
    ```

If `DATABASE_URL` is defined, SQLx will continue building against a database.
To force building in offline mode, set the `SQLX_OFFLINE` environment variable to `true`.
If you want to make this the default, add it to your `.env` file.

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

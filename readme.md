# `edgedb_codegen`

> Generates fully typed rust code from an edgedb schema and inline / file-based queries.

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link]

## Installation

To install the `edgedb_codegen` crate you can use the following command.

```bash
cargo add edgedb_codegen
```

Or directly add the following to your `Cargo.toml` file.

```toml
edgedb_codegen = "0.1.0" # replace with the latest version
```

## Description

When working with `edgedb` you often need to write queries and also provide the typed for both the input and output. Your code is only checked at runtime which increases the risk of bugs and errors.

Fortunately, `edgedb` has a query language that is typed and can be converted into types and queried for correctness at compile time.

### Inline Queries

```rust
use edgedb_codegen::edgedb_query;
use edgedb_errors::Result;
use edgedb_tokio::create_client;

// Creates a module called `simple` with a function called `query` and structs
// for the `Input` and `Output`.
edgedb_query!(simple, "select {hello := \"world\", custom := <str>$custom }");

let client = create_client().await?;
let input = simple::Input::builder().custom("custom").build();

// For queries the following code can be used.
let output = simple::query(&client).await?;
```

This macro will generate the following code:

### Query Files

Define a query file in the `queries` directory of your crate called `select_user.edgeql`.

```edgeql
# queries/select_user.edgeql

select User {
  name,
  bio,
  slug,
} filter .slug = <str>$slug;
```

Then use the `edgedb_query` macro to import the query.

```rust
use edgedb_codegen::edgedb_query;
use edgedb_tokio::create_client;


// Creates a module called `select_user` with public functions `transaction` and
// `query` as well as structs for the `Input` and `Output`.
edgedb_query!(select_user);

let client = create_client().await?;
let input = select_user::Input::builder().slug("test").build();

// Generated code can be run inside a transaction.
let result = client.transaction(|mut txn| {
	async move {
		let output = select_user::transaction(&mut txn).await?;
		Ok(output)
	}
}).await?;
```

## Contributing

[`devenv`](https://devenv.sh/) is used to provide a reproducible development environment for this project. Follow the [getting started instructions](https://devenv.sh/getting-started/).

To automatically load the environment you should [install direnv](https://devenv.sh/automatic-shell-activation/) and then load the `direnv`.

```bash
# The security mechanism didn't allow to load the `.envrc`.
# Since we trust it, let's allow it execution.
direnv allow .
```

At this point you should see the `nix` commands available in your terminal.

Run the following commands to install all the required dependencies.

```bash
install:all
```

This installs all the node dependencies, cargo binaries and solana tooling locally so you don't need to worry about polluting your global namespace.

### Upgrading `devenv`

If you have an outdated version of `devenv` you can update it by running the following commands. If you have an easier way, please create a PR and I'll update these docs.

```bash
nix profile list # find the index of the nxi package
nix profile remove <index>
nix profile install --accept-flake-config github:cachix/devenv/<version>
```

### Editor Setup

To setup recommended configuration for your favorite editor run the following commands.

```bash
setup:vscode # Setup vscode
```

## License

Unlicense, see the [license](./license) file.

[crate-image]: https://img.shields.io/crates/v/edgedb_codegen.svg
[crate-link]: https://crates.io/crates/edgedb_codegen
[docs-image]: https://docs.rs/edgedb_codegen/badge.svg
[docs-link]: https://docs.rs/edgedb_codegen/
[ci-status-image]: https://github.com/ifiokjr/edgedb_codegen/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/edgedb_codegen/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense

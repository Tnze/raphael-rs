# Raphael XIV [<img src="https://img.shields.io/discord/1244140502643904522?logo=discord&logoColor=white"/>](https://discord.com/invite/m2aCy3y8he)

:link: [www.raphael-xiv.com](https://www.raphael-xiv.com/)

Raphael is a crafting rotation solver for the online game Final Fantasy XIV.
* Produces optimal solutions.
* Short solve time (20-60 seconds) and reasonable memory usage (300-1000 MB) for most configurations.

## How does it work?

* Short answer: [A* search](https://en.wikipedia.org/wiki/A*_search_algorithm) + [Pareto optimization](https://en.wikipedia.org/wiki/Multi-objective_optimization) + [Dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming).
* Long answer: coming soon<sup>tm</sup>

## Building from source

The [Rust](https://www.rust-lang.org/) toolchain is required to build the solver.

### Native app

To build and run the application:

```
cargo run --release
```

### Native CLI

To build and run the command-line interface (CLI):

```
cargo run --release --package raphael-cli -- <cli-args>
```

The CLI can also be installed so that it can be called from anywhere:

```
cargo install --path raphael-cli
```

### Web app (WASM)

[Trunk](https://trunkrs.dev/) is required to bundle and host the website and can be installed via the Rust toolchain:

```
cargo install --locked trunk
```

To build and host the application locally:

```
export RANDOM_SUFFIX=""
export RUSTFLAGS="--cfg=web_sys_unstable_apis"
trunk serve --release --dist distrib
```

# Development

Here is how to set up the development environment for the project.

## Install Git

Install Git by following the instructions at [https://git-scm.com/downloads](https://git-scm.com/downloads).

## Download the code

From PowerShell on Windows or Terminal on macOS/Linux:

```bash
git clone https://github.com/evgenyneu/warrah.git
```

Change directory to the root of the project:

```bash
cd warrah
```

## Running tests

Run both project and build tests:

```bash
cargo test
```

## Running benchmarks

```bash
cargo bench
```

The results of previous runs are in the [benches/results](../benches/results) directory, where the first line is the CPU and OS:

```sh
echo "CPU: $(lscpu | grep "Model name" | cut -d: -f2 | xargs) | OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
```

See [benches/README.md](../benches/README.md) for more details.

## How to add new langauge

Here is how to add a support for a new language or change comments in existing one:

1. Make changes to [docs/languages.md](languages.md) file.
1. Build the project, it will auto-generate the Rust code for the changes in [src/languages/generated.rs](../src/languages/generated.rs).
1. Submit a pull request.

## Contributing guidelines

See contributing guidelines in [CONTRIBUTING.md](../CONTRIBUTING.md).

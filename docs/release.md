# Insruction how to release new version

1. Update the version in your Cargo.toml file.
2. Document changes in CHANGELOG.md.
3. Publish to crates.io

```sh
cargo login
cargo package
cargo publish
```

4. Make a release on Github

```sh
git tag v0.1.2
git push origin v0.1.2
```

5. Update homebrew formula

In the homebrew formula:

https://github.com/evgenyneu/homebrew-warrah/blob/main/warrah.rb

* Update the version.
* Update the SHA256 hashes by downloading the files from Github releases (https://github.com/evgenyneu/warrah/releases) and calculate the hash for each binary:

```sh
shasum -a 256 warrah-macos-v0.1.3.tar.gz
```

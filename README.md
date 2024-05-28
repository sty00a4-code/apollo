# Apollo

A light-weight package manager for the Luna programming language

## Apollo Package

An apollo package needs to have an `apollo.toml` file in it's directory.
With that file the package defines itself.
When initialising your package, that file will look like this:
```toml
[package]
name = "package-name"
version = "v0.1"
lib = "init.luna"
[dependencies]
some_package = "v2.4"
```
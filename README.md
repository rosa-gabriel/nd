# ND

ND is a project to create a CLI tool capable of replacing [postman](https://github.com/postmanlabs) for day-to-day development in a terminal-centered environment.

You can check the command's documentation by running the command:

```sh
nd --help
```

Features
--------

- Declarative description of HTTP/HTTPS requests so they are replicable;
- Support to auto format/generate tokens for `basic` and `bearer`;

Install from package
--------------------

Pre-built packages for Windows, macOS, and Linux are found on the
[Releases](https://github.com/rosa-gabriel/nd/releases) page.

Install from source
-------------------

The build process uses rust's [cargo](https://doc.rust-lang.org/cargo/) package manager.
After installing the dependencies, run the following command.

    cargo build --release

This will generate the executable for the current OS in `./target/release/nd`.

### Hello, Cargo!

*"99.5% of Rust developers never use rustc directly, and doing so is simply going to lead you to pain."*

Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

The simplest Rust programs, like the one we’ve written so far, don’t have any dependencies. If we had built the “Hello, world!” project with Cargo, it would only use the part of Cargo that handles building your code. As you write more complex Rust programs, you’ll add dependencies, and if you start a project using Cargo, adding dependencies will be much easier to do.

**Cargo comes installed with Rust** if you used the official installers discussed in the “Installation” section

1) run
```bash
$ cargo new ./project-name
```

2) run
```bash
$ cargo build
```

3) run
```bash
$ cargo run
```
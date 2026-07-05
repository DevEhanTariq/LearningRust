# How to use Cargo:

### Cargo:

- Type `cargo` into the terminal to see all of its commands.

### Creating a Cargo project:

- Type `cargo new {project name}`, this will create a new cargo project.
- You should see a directory called `"src"`, This contains the source code.
- You should also see a `"Cargo.toml"` file.

### Running a Rust file using Cargo:

- Instead of using `rustc main.rs`, you can go into your project using `cd {project name}`.
- After that, just type `cargo build` into your terminal
- Now, to run your rust files, you type `cd target/debug` into your terminal.
- Then, if your on Windows, you type `.\{project name}.exe` or, if your on Mac or Linux, type `./{project name}`.
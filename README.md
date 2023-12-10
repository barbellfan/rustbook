# Rust Book Examples
This is an attempt to run all the example code from the Rust Book.

All of the code is in separate folders. The first level is the chapter number, with a leading zero, like `ch02` for examples in chapter 2. Inside, the sub-examples are named with the chapter and an ordinal, and then the name of the example, like `ch1_3_hello_cargo`.

I use underscores instead of periods in the folder names because Rust does not like periods in folder names.

To avoid the goofy ch* stuff on the executable, edit the `Cargo.toml` file in the project folder to set the package name to something sensible.

To build, go to the root directory and run:

`cargo build`

It will build all the projects.

To run a project, run:

`cargo run --bin nameofexecutable`

Note that all the projects will get built in the `target` folder in the root. Hopefully, none of the examples will have duplicate names.

To add a project to the build, edit the `Cargo.toml` file in the root, and add the path to the `members` array.

To run a project using the `tasks.json` config in Visual Studio Code, set the path to the project to the `default-members` array in the root `Cargo.toml`. Even though it is an array, I'm only putting one thing in there.

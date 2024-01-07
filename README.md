<h1>kRusty Krab Code</h1>

<p>This is a small GUI app, built with Slint UI framework and the Rust Programming language. This app lets users write small snippets of rust code and run it using the machine, without installing rustc/cargo on their machine.</p>

<p>For completing the project, I have used the API of Rust playground and used reqwest crate to execute</p>

<p>Big Thanks to <i><b>github.com/menard-codes</b></i> which i refered for completing the project.</p>

<p>As well as <i><b>youtube.com/@TravisMedia</i></i>b> for inspiring me in building a gui with rust</p>



# Slint Rust Template

A template for a Rust application that's using [Slint](https://slint.rs) for the user interface.

## About

This template helps you get started developing a Rust application with Slint as toolkit
for the user interface. It demonstrates the integration between the `.slint` UI markup and
Rust code, how to trigger react to callbacks, get and set properties and use basic widgets.

## Usage

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up a sample project with this template
    ```
    cargo generate --git https://github.com/slint-ui/slint-rust-template --name my-project
    cd my-project
    ```
3. Build with cargo
    ```
    cargo build
    ```
4. Run the application binary
     ```
     cargo run
     ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

We hope that this template helps you get started and you enjoy exploring making user interfaces with Slint. To learn more
about the Slint APIs and the `.slint` markup language check out our [online documentation](https://slint.dev/docs).

Don't forget to edit this README to replace it by yours

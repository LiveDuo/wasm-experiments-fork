wit_bindgen::generate!();

// Define a custom type and implement the generated `Host` trait for it which
// represents implementing all the necesssary exported interfaces for this
// component.
struct MyHost;

impl HelloWorld for MyHost {
    fn greet() {
        name();
    }
}

export_hello_world!(MyHost);
# Indomitable (TCP Latency)
## Concept:
Testing TCP latency between 2 endpoints. For the sake of this project, the 2 endpoints will be the client browser and the webpage. 
Upon reaching the webpage, the client will be offered a few options, populated by reasonable defaults. In effect, it will be akin to a web-based TCP Ping. 

## Existing Python3 CLI Code:
[Now on Github](https://github.com/consistentlyinconsistent/cautious-goggles/blob/master/indomitable/python3/my_tcp_latency.py)

## Possibly Helpful Links:
[Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

[Wasm By Example](https://wasmbyexample.dev/examples/hello-world/hello-world.rust.en-us.html)

[A Rust Introduction to WebAssembly](https://www.telerik.com/blogs/rust-introduction-to-webassembly)

## Justification
python is great for quickly prototyping a tool, but to deploy something for testing starts to add up in the way of overhead. By placing the application into a webpage (even a simple one), it negates the need for generating a binary to distribute, and reduces the overhead of administration for an application. 

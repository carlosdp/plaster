# shade
**shade** is a modern Rust framework for creating frontend apps with WebAssembly.

It was orginally forked from [yew](yew).

## Why the fork?
**yew** is a great framework and I've found a ton of great use from it on my projects.
However, it was built on-top of [stdweb](stdweb), which is also great, but [wasm-bindgen](wasm-bindgen)
and the associated crates such as [web_sys](web_sys) are pretty much the "annointed" libraries for low-level
access to Web/JS APIs in WebAssembly. They are designed to more or less match the eventual host-level bindings
to these APIs directly from WebAssembly and are generated from WebIDL definitions, making their upkeep much
easier and reducing the time to access new APIs as they become standardized and available.

Additionally, **yew** takes an opinionated stance to concurrency and parallelism with its actor model. I'm not
personally a huge fan of actors, and I'd prefer to just use Futures and libraries that build on top of that
primitive, so I'd like the framework to easily support that more idiomatic model. [wasm-bindgen-futures](wasm-bindgen-futures)
makes this nice and easy to do with the browser's built-in Promise support.

In a nutshell:

- **yew** is built on **stdweb**, I want to use [wasm-bindgen](wasm-bindgen) and [web_sys](web_sys).
- **yew** implements an actor-based concurrency model, I want to use Futures and Promises.
- **yew** implements `Services` to try and provide some higher-level Rust primitives for some commmon JS/Web
  patterns. I think this is out of the library's scope and would like to thin it out by removing this concept
  and instead making interoperation between Promise-based Futures and Component updates easy.
- **yew** uses a custom macro for JSX-like syntax. I'd like to explore potentially integrating one of the
  solutions others are working on for a "common" macro that does this. (This isn't really a reason to fork,
  more just a note for the future)

[yew](https://github.com/DenisKolodin/yew")
[stdweb](https://github.com/koute/stdweb")
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
[wasm-bindgen-futures](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/futures)
[web_sys](https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys)

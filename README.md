<h1 align="center">tracing-actix-web2</h1>
<div align="center">
 <strong>
   Rust tracing adapter for Actix Web
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/tracing-actix-web2">
    <img src="https://img.shields.io/crates/v/tracing-actix-web2.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/tracing-actix-web2">
    <img src="https://img.shields.io/crates/d/tracing-actix-web2.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/tracing-actix-web2">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>
<br/>

This crate aims to bridge [tracing](https://github.com/tokio-rs/tracing) and [actix-web](https://github.com/actix/actix-web) by automatically populating data from the HttpRequest in the tracing span.

This crate is an alternative to [tracing_actix_web](https://github.com/LukeMathWalker/tracing-actix-web) and was created to add the following two features.

### Request Logging

This crate will log every request made to the HTTP server. These logs are only be visible if the log level is set to `trace`. This can be done by setting the `RUST_LOG` environment varible to `tracing_actix_web2=trace`.

### X-Request-ID Header

This crate will generate a UUID for each HTTP request and include it in the span along with the `X-Request-ID` HTTP header.

## Install

Add `tracing-actix-web2` to your dependencies:

```toml
[dependencies]
# ...
actix-web = "3.3.2"
tracing-actix-web2 = "1.0.0"
```

## Usage

```rust,compile_fail
use actix_web::{App, web, HttpServer};
use tracing_actix_web2::Tracer;

fn main() {
    // Init your `tracing` subscriber here!

    let server = HttpServer::new(|| {
        App::new()
            // Mount `Tracer` as a middleware
            .wrap(Tracer)
            .service( /*  */ )
    });
}
```

For a full example check out [this example application](https://github.com/oscartbeaumont/tracing-actix-web2/tree/main/example).

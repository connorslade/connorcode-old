@Title = afire Update
@Date = 01-11-21
@Description = Changes to afire in V0.3.0
@Path = afire/update-0
@Assets = .

---

# 🔥 afire v0.3.0

This document will outline some new features and changes to [afire](crates.io/crates/afire) in version `0.3.0`! The full changelog can be found [here](https://github.com/Basicprogrammer10/afire/blob/0.2.3/Changelog.md).

## 📰 New Features

#### Content Types

Now you don't need to make a Header for every response to set the Content Type.
Now you can call `.content(Content::X)` on your response.
Currently, Content supports the following Types:

- HTML
- TXT
- CSV
- JSON
- XML

Here is an example of creating a Response and using Content to add the JSON content type.

```rust
use afire::{Response, Content};

Response::new()
  .text(r#"{ "hello": "world" }"#)
  .content(Content::JSON)
```

#### _Advanced_ Middleware

I have made the Middleware much more powerful! Is now a Trait you can implement on a struct.
You can implement the `pre`, `post` and `attach` functions for it.
Pre will run _before_ the routes handle the request, Post will run _after_.

Middleware will now return a MiddleResponse from post and MiddleRequest from pre.
This allows you to do any of the following things:

- Continue - Move on to the next Middleware or Route
- Add - Modify the Request / Response and continue to the next Middleware / Route
- Send - Send a Response now. Will not run other Middleware or Routes.

Here is a simple Logger Middleware from [examples](https://github.com/Basicprogrammer10/afire/blob/main/examples/08_middleware.rs)

```rust
use afire::{
    internal::common::remove_address_port,
    middleware::{MiddleRequest, Middleware},
    Header, Method, Request, Response, Server,
};

struct Log;

// Now we will Implement Middleware for Log
impl Middleware for Log {
    // Redefine the `pre` function
    // (Runs before Routes)
    fn pre(&mut self, req: Request) -> MiddleRequest {
        // Print some info
        println!(
            "[{}] {} {}",
            remove_address_port(req.address),
            req.method,
            req.path
        );
        // Note: req.address also has the client port
        // This is being removed with
        // Ex: 127.0.0.1:6264 => 127.0.0.1

        // Continue to forward the request to the next middleware or route
        MiddleRequest::Continue
    }
}


// Attatch to the server like this
Log.attach(&mut server);
```

#### Path Parameters

Path parameters are used embed data in a URI. For example a page like this one could use a path parameter to get the name of the document you want to access:
```HTTP
GET /writing/{document}
```

Now lets make an API route to greet people!
```rust
use afire::{Header, Method, Query, Response, Server, Content};

// Add the Greet API Path
server.route(Method::GET, "/greet/{name}", |req| {
    // As this route would ever run without all the path params being filled
    // It is safe to unwrap if the name is in the path
    let data = format!("<h1>Hello, {}</h1>", req.path_param("name").unwrap());

    Response::new()
        .text(data)
        .content(Content::HTML)
});
```

#### Socket Closing

This is a smaller change but it could be usefull... Maybe...
It allows you to make a Response that will kill the socket.
If a response has any other data it will not be sent.
Im not exactly sure what this is usefull but anyway.

Here is a route that will kill the socket when called.

```rust
use afire::{Header, Method, Query, Response, Server, Content};

// Add the Greet API Path
server.route(Method::GET, "/kill", |_req| {
    Response::new().close()
});
```

#### Other

- Changeable Socket Buffer size
- Made Internal functions Public `afire::internal::{http, common, path}`
- Use Rusts `std::net::IpAddr` for Server Ip

## 💠 Changes

- Removed limited Thread pool
- Deprecate `.all` routes
- Update Logger / Rate limit Syntax

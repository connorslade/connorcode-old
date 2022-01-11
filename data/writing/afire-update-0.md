@Title = afire Update
@Date = 01-11-21
@Description = Changes to afire in V0.3.0
@Path = afire/update-0
@Assets = .

---

# 🔥 afire v0.3.0

This document will outline some new features and changes to [afire](crates.io/crates/afire) in version `0.3.0`!
The full changelog can be found on GitHub [here](https://github.com/Basicprogrammer10/afire/blob/0.2.3/Changelog.md).

## 📰 New Features

#### Content Types

You no longer need to make a Header for every response just to set the Content Type.
Now you can just call `.content(Content::X)` on your response.
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

I have made the Middleware much more powerful!
Is now a Trait you can implement the `pre`, `post` and `attach` functions for it.
Pre will run _before_ the routes handle the request, Post will run _after_.

Middleware will now return a `MiddleResponse` from post and `MiddleRequest` from pre.
This allows you to do any of the following things:

| Option   | Description                                                   |
| -------- | ------------------------------------------------------------- |
| Continue | Move on to the next Middleware or Route                       |
| Add      | Modify the Request / Response and Continue                    |
| Send     | Send a Response now. Will not run other Middleware or Routes. |

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

Path parameters are used to embed data in a URI. For example a page like this one could use a path parameter to get the name of the document you want to access:

```HTTP
GET /writing/{document}
```

I think the world needs more positivity…
So let's make an API route to greet people!

```rust
use afire::{Header, Method, Query, Response, Server, Content};

// Add the Greet API Path
server.route(Method::GET, "/greet/{name}", |req| {
    // This route can't run without all the path params being filled
    // It is safe to unwrap if the name is in the path
    let data = format!("<h1>Hello, {}</h1>",
      req.path_param("name").unwrap()
    );

    Response::new()
        .text(data)
        .content(Content::HTML)
});
```

#### Socket Closing

This is a smaller change, but it could be useful… Maybe…
It allows you to make a Response that will kill the socket.
If a response has any other data it will not be sent.
I'm not exactly sure what this is useful for.

anyway

Here is a route that will kill the socket when called.

```rust
use afire::{Header, Method, Query, Response, Server, Content};

// *KILL ALL SOCKETS*
server.route(Method::GET, "/kill", |_req| {
    Response::new().close()
});
```

#### Other

Here are some less important changes that are still worth knowing about.

- Custom Socket Buffer sizes
- Made Internal functions Public `afire::internal::{http, common, path}`
- Server now use Rusts `std::net::IpAddr` for Server IP

## 💠 Changes

- Update Logger / Rate limit Syntax
- Removed the very limited Thread pool
  - Don't worry… It will return. _someday_
- Deprecate `.all` routes
  - Now use `.route(Method::Any, "**", ...)`

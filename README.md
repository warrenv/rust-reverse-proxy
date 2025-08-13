
# Book User Routing Proxy (BURP)

A reverse proxy for book.io and stuff.io.

The proxy receives html requests to book.io or stuff.io from the ingress
gateway and applies the following algorithm:

- Does a page exist in wordpress for the route?
  - Yes, route to wordpress.
  - No, route to the frontend app.

The frontend app tries to match the route in the following order:
  - Does the url match a frontend route?
    - Yes, display it.

  - Is there a matching user profile?
    - Yes, display it.

  - Send a 404.


- Reference: https://aminshamim.medium.com/building-a-high-performance-http-proxy-server-in-rust-with-hyper-tokio-1fa6145847cb

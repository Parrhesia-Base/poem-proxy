<p style="text-align: center" align="center">
  <img src="/images/logo.png?raw=true"  alt="Employee data" title="Employee Data title" height=500px />
  <p>
    This crate provides an endpoint for the <a href="https://github.com/poem-web/poem">poem web framework</a> that serves files and requests from another server.
</p>
  <a href="#">
    <img src="https://img.shields.io/github/stars/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="Number of Stars">
  </a>
  <a href="#">
    <img src="https://img.shields.io/github/issues/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="Number of Issues">
  </a>
  <a href="#">
    <img src="https://img.shields.io/docsrs/poem-proxy/latest?style=for-the-badge" alt="License">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/github/license/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="License">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/crates/d/poem-proxy?style=for-the-badge" alt="License">
  </a>
  <!-- Activate below when there is an actual release -->
  <!-- <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/Parrhesia-Base/poem-proxy?color=purple&style=for-the-badge"/> -->
</p>

# Development Roadmap
While this project is in work, I will be following this development roadmap. In the end, poem-proxy will be a versatile service that is ready for your next web application.

- [ ] Create a proxy that can forward http requests to another server and send its response back
  - [X] Get requests
  - [X] Post requests
  - [ ] Put requests
  - [ ] Patch
  - [ ] Delete
  - [ ] Ensure all necessary information is captured
- [X] Add websocket support to the proxy endpoint
  - [ ] Ensure all necessary information is captured
- [ ] Allow finer configuration of the proxy endpoint
  - [ ] Http/Https
  - [ ] Ws/Wss
  - [ ] Forward to different target servers
- [ ] Enable a templating engine that will allow the proxy to fill information in that the proxied server might not know
  - [ ] Templating for request
    - [ ] Headers
    - [ ] Body
    - [ ] Other parts
  - [ ] Templating for response
    - [ ] Headers
    - [ ] Body
    - [ ] Other parts
  - [ ] Templating for websocket communications
- [ ] Ensure that proxy is implemented properly with non-advocate code review
- [ ] Write comprehensive tests to ensure functionality is not lost
- [ ] Write comprehensive documentation that shows clear examples of how to use the proxy

As you can see, this proxy service is not yet fully functional. The items checked off have been tested and do work to some limited capacity. I have successfully used this proxy to bridge my Poem webserver to a Sveltekit webserver, though advanced functionality is still not there.

# Documentation
Documentation is not yet finished, but can be found [here](). Just remember that it's a work-in-progress. [Parhesia](https://github.com/Parrhesia-Base/Parrhesia) is nowhere near completion, but you can check out that repository to see how this proxy endpoint is currently being used.

# Purpose

The main purpose of this is to enable rust backends to forward requests to other servers, such as a NodeJS server. For example, if you are developing an app with sveltekit, you miss out on a lot of its advantages if you serve it as a static site using the **StaticFilesEndpoint**. For one thing, you won't see automatic updates from HMR.

Even if you eventually want to use the **StaticFilesEndpoint**, this can be useful for app development before moving over to the static generation.

## License
This code is licensed under the [MIT License](https://github.com/Parresia-Web/poem-proxy/blob/main/LICENSE). If you make improvements, please consider contributing them back to main. You are completely free to use this code and this library for any purpose.

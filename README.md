<p style="text-align: center" align="center">
  <a href="https://github.com/Parrhesia-Base/poem-proxy">
    <img src="/images/logo.png?raw=true"  alt="Poem Proxy" title="Poem Proxy" width=400px />
  </a>
  <p align="center">
    This crate provides an endpoint for the <a href="https://github.com/poem-web/poem">poem web framework</a> which transfers data to and from another server
</p>
  <a href="#">
    <img src="https://img.shields.io/github/stars/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="Number of Stars">
  </a>
  <a href="https://github.com/Parrhesia-Base/poem-proxy/issues">
    <img src="https://img.shields.io/github/issues/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="Number of Issues">
  </a>
  <a href="https://docs.rs/poem-proxy/latest/poem_proxy/">
    <img src="https://img.shields.io/docsrs/poem-proxy/latest?style=for-the-badge" alt="License">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img src="https://img.shields.io/github/license/Parrhesia-Base/poem-proxy?style=for-the-badge" alt="License">
  </a>
  <a href="https://crates.io/crates/poem-proxy">
    <img src="https://img.shields.io/crates/d/poem-proxy?style=for-the-badge" alt="License">
  </a>
  <a href="https://crates.io/crates/poem-proxy">
    <img alt="Published Version" src="https://img.shields.io/crates/v/poem-proxy?style=for-the-badge"/>
  </a>
</p>

# Development Roadmap
While this project is in work, I will be following this development roadmap. In the end, poem-proxy will be a versatile service that is ready for your next web application. It is not quite there yet - but it is on its way!

- [ ] Create a proxy that can forward http requests to another server and send its response back
  - [X] Get requests
  - [X] Post requests
  - [ ] Put requests
  - [ ] Patch
  - [ ] Delete
  - [ ] Ensure all necessary information is captured
- [X] Add websocket support to the proxy endpoint
  - [ ] Ensure all necessary information is captured
- [X] Allow finer configuration of the proxy endpoint
  - [X] Http/Https
  - [X] Ws/Wss
  - [X] Forward to different target servers
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
- [ ] Set up CI to ensure codebase stays relevant even after development is complete

As you can see, this proxy service is not yet fully functional. The items checked off have been tested and do work to some limited capacity.

# About
### Purpose
Poem-proxy is a sub-project of [Parrhesia](https://github.com/Parrhesia-Base/Parrhesia). It is meant to facilitate a seamless user experience for front-end development without needing to route internet traffic to multiple backend servers. Instead, all connections can be sent to the Rust backend, which will sort through and forward any necessary requests.

Parrhesia will officially support Sveltekit as the front-end framework of choice, and even today this proxy endpoint can handle all its basic functionality. Even Hot-Module-Reloading, which is a massive boost to the development experience.

### Documentation
Documentation is also in work, and can be found [here](https://docs.rs/poem-proxy/latest/poem_proxy). Just remember that it's a work-in-progress. [Parhesia](https://github.com/Parrhesia-Base/Parrhesia) is nowhere near completion, but you can check out that repository to see how this endpoint is currently being used.

### Contributing
Due to the small scale of this project, code contributions really aren't currently needed. That being said, I am in need of people knowledgable with proxies to review my work and ensure it is correct and follows today's proxy standards. In addition, please create issues if you come across any bugs or have any other ideas on how this proxy can be improved.

### License
This code is licensed under the [MIT License](https://github.com/Parresia-Web/poem-proxy/blob/main/LICENSE). If you make improvements, please consider contributing them back to main. You are completely free to use this code and this library for any purpose. Anything you contribute shall be licensed as MIT, without additional terms or conditions.

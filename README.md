# poem-proxy
This crate provides an endpoint for the [poem web framework](https://github.com/poem-web/poem) that serves files and requests from another server.

### NOT YET FUNCTIONAL

The main purpose of this is to enable rust backends to forward requests to other servers, such as a NodeJS server. For example, if you are developing an app with sveltekit, you miss out on a lot of its advantages if you serve it as a static site using the **StaticFilesEndpoint**. For one thing, you won't see automatic updates from HMR.

Even if you eventually want to use the **StaticFilesEndpoint**, this can be useful for app development before moving over to the static generation.

## License
This code is licensed under the [MIT License](https://github.com/Parresia-Web/poem-proxy/blob/main/LICENSE). If you make improvements, please consider contributing them back to main. You are completely free to use this code and this library for any purpose.
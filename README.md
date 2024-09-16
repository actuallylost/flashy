# Lunarstack

A user-friendly software stack that helps you be more productive writing code instead of organizing!

## Prerequisites

In order to be able to run the project, you will need to have the following installed:

-   [PNPM](https://pnpm.io/)
-   [Node](https://nodejs.org/)
-   [Rust](https://www.rust-lang.org/)
-   [Docker](https://www.docker.com/)

If you'd like to use `Rust-Warp` stack, you will need to install both `Node` and `Rust`, otherwise you only need `Node`.

## Available Flavors

There are different apps and services you can combine to use as a stack:

-   `apps/web` - A web app written in `Next`.
-   `apps/tauri-web` - A web app written in `Next` and `Tauri`.
-   `services/nest-api` - An API written in `Node` and `NestJS`.
-   `services/warp-api` - An API written in `Rust` and `Warp`.

There are also available crates if you want to use `Rust`:

-   `crates/prisma` - A crate to use `Prisma` with `Warp`.

## Installing Dependencies

To install the dependencies of the project, you can run the following command:

```bash
pnpm install
```

## Directories

The project is organized in the following directories:

-   `apps` - Contains the available apps that can be used as a frontend for the application.
-   `assets` - Contains the assets for the project.
-   `common` - Contains commonly used code and resources that are used throughout the project.
-   `crates` - Contains the available crates that can be used in combination with the services of the application.
-   `docs` - Contains the documentation for the project.
-   `packages` - Contains the packages for the project.
-   `services` - Contains the available services that can be used as a backend in the application.
-   `tools` - Contains the available tools that can be used within the project. This includes scripts, cli tools, and other utilities.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

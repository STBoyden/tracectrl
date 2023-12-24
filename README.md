# TraceCTRL

> This is the successor to CodeCTRL - originally developed during my time at [Authentura](https://github.com/Authentura/codectrl).

## Project structure

The project is made up of the Rust backend located in the root of the project, and the front-end available in [tc-frontend](https://github.com/STBoyden/tracectrl/tree/main/tc-frontend).

The backend is a Axum REST server, that exposes API endpoints to add and get logs - differently from how CodeCTRL did it with gRPC. When the backend is ran, documentation for the API is generated and viewable at `/docs/swagger` and `/docs/redoc`. Additionally, the OpenAPI JSON can be made locally by running the backend with the `save_docs` feature enabled (enabled by default) - which will output a file called `openapi.json` to the root of the project. This can be used to generate client/server bindings to the API automatically using a tool such as <https://editor.swagger.io>.

## Requirements

### Backend

Building and running the backend on all platforms only requires a nightly toolchain for Rust, and no other external dependencies. The process for acquiring Rust will vary slightly depending on the platform, for major platforms (Windows, macOS, Linux) see: <https://rustup.rs>.

The script `run-dev-db.sh` (or `run-dev-db.ps1` for Windows) requries that both Docker and OpenSSL commands are installed, and will not function without them. The usage of this script is entirely optional and thus Docker and OpenSSL are not strict requirements for the rest of the project.

### Frontend

Building and running the frontend requires one of the following applications to be available:

1. Bun (not currently released on Windows) - <https://bun.sh>
2. PNPM - <https://pnpm.io/>
3. Yarn - <https://yarnpkg.com>
4. NPM (included with Node) - <https://nodejs.org/>

The project will look for the above applications at compile-time in the order listed. Please refer to the above project's websites for instructions on how to install.

## Usage

Usage of the API can be found more in-depth by examining the served docs at runtime.

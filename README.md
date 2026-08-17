## Deployment

1. `./build.sh
2. `cd clients/web && npm install` if running for the first time (SST's build/dev commands shell out to `npm run ...` but don't install dependencies for you).
3. `./run-dev.clj` is running for the first time or 
`sst deploy` if SST is already running.

## Unit tests
1. `cd <<folder with Cargo.toml>>`
2. `cargo test`

## Shared Package Docs

- Core functional helper docs (including `partial!` and `partial_right!`): [services/packages/base/README.md](services/packages/base/README.md)
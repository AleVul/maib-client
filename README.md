# maib-client - an unofficial rust client for MAIB API.
This is a REST client for the API that currently only supports a small portion of MAIB API.

## Supported API
- MIA
    - create QR
    - cancel QR
    - get QR details
    - get payment
    - refund payment

E-commerce API support is in the works

## Running tests
To run sandbox tests, set `MAIB_SANDBOX_BASE_PATH` and `MAIB_SANDBOX_ACCESS_TOKEN` env variables in `.env` file, then run:
```shell
just test-sandbox
```
Note that you need [just](https://github.com/casey/just) command runner.
If you dont have the runner, then just set env variables and run:
```shell
cargo test --test sandbox
```
Both of these will only run integration tests.

## License
Licensed under either of:

- Apache License, Version 2.0 (<http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (<http://opensource.org/licenses/MIT>)

at your option.

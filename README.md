# Sig-Proxy - Signature-Based Authenticating Proxy

## What is it?

- a proxy for HTTP services,
- that validates signatures,
- and provides an AuthN layer.

also...

- a proxy for HTTP clients,
- that injects signatures into requests.

## How do I use it?

There are example applications. These assume you have [Ganache](https://www.trufflesuite.com/ganache) running locally.

`dummy-service` is a simple service that prints the request headers in the response body. It's handy for seeing what's going on.


Replace the embedded key in `ethereum.rs` with the private key of an address from Ganache.

```shell
cargo run --example dummy-service
cargo run --example proxy
cargo run --example ethereum
curl http://localhost:8002
```

## License

`sig-proxy` is available under the MIT License. See `LICENSE.txt` for the full text.

# XYO SDK HTTP Client for Rust
This is a custom build HTTP Request inspired by my curiosity and fueled by my ADD. It's created as part 
of XYO Financial's SDK for Rust hence the hardcoded hostname and a few other configuration. It uses has 
zero dependency, and it only utilises Rust's built-in libraries such as: `std::net` to create a TCP 
connection and uses `HTTP 1.1` RFC standards to communicate with the TCP server. To visit the original 
XYO Financial SDK please visit: [Crates XYO Financial SDK](https://crates.io/crates/xyo-sdk).


## RFC Standard Specification for HTTP 1.1
For purpose of SDK calls only `GET` and `POST` with `application/json` header should 
be enough to consider this library complete.

__`GET` Request Message__
```text
GET /api/v1/enrichments/status/23230 HTTP/1.1
Host: api.xyo.financial
Accept: application/json

```

__`POST` Request Message__
```text
POST /api/v1/enrichment HTTP/1.1
Host: api.xyo.financial
Content-Type: application/json
Accept: application/json
Content-Length: 48

{"content":"SomeContentHere","countryCode":"GB"}
```


#### Links
 * [curl to HTTP 1.1 Spec Converter](https://curlconverter.com/http/)


#### Credits
Copyright &copy; Syniol Limited. Licensed under the BSD 3-Clause License - see the [LICENSE](../LICENSE) file for details.

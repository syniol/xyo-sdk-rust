# XYO SDK HTTP Client for Rust
This is a custom build HTTP Request inspired by my curiosity and fueled by my ADD.


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


#### Credits
Copyright &copy; Syniol Limited. All rights reserved.

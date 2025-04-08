Next Gen Signatures Bench
=========================

This repository contains an `Android` and `iOS` App to benchmark the [next-gen-signatures crate](https://github.com/UbiqueInnovation/sprind-next-gen-signing-service) on real world devices.

Benchmark Setup
---------------

The benchmarks were done with the setup noted below, meaning the following proof is generated:
* `https://schema.org/name` is being reveiled
* a ZKP over `https://schema.org/birthDate` is made, proving the value is less than `2001-01-01`
* a ZKP over the `device_binding_keys` is made, proving that the credential belongs to a device holding the keys without reveiling the public key.

```js
credential = {
    "https://schema.org/name": "John, Doe",
    "https://schema.org/birthDate": {
        "@value": "2000-01-01T00:00:00Z",
        "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
    },
    "https://schema.org/dog": {
        "https://schema.org/name": "Ricky"
    }
}

proof_requirements = [
    { "type": "required", "key": "https://schema.org/name" },
    {
        "type": "circuit",
        "id": "https://zkp-ld.org/circuit/ubique/lessThanPublic",
        "private_var": "a",
        "private_key": "https://schema.org/birthDate",
        "public_var": "b",
        "public_val": {
            "@value": "2001-01-01T00:00:00Z",
            "@type": "http://www.w3.org/2001/XMLSchema#dateTime",
        }
    }
    { "type": "device-binding", ... }
]
```

Results
-------

The Benchmarks were performed over `n=100` runs.

|       Device      | Avg. Issuance Time (ms) | Avg. Proof generation Time (ms) | Avg. Verification Time (ms) |
|-------------------|-------------------------|---------------------------------|-----------------------------|
|      Sony XQ-DE54 |                    13ms |                           708ms |                       422ms |
|    Google Pixel 6 |                    21ms |                           941ms |                       517ms |
|  Samsung SM-A236B |                    30ms |                          1233ms |                       712ms |
|  Samsung SM-A528B |                    26ms |                          1342ms |                       870ms |
|  Samsung SM-A145R |                    81ms |                          2806ms |                      1744ms |
| iPhone 11 Pro Max |                     5ms |                           707ms |                       431ms |
|          iPhone X |                     7ms |                           972ms |                       601ms |

# DoH Resolver

A simple Rust command-line DNS-over-HTTPS (DoH) client, similar to `dig`

## Features

- Resolve DNS queries over HTTPS
- Supports custom DNS servers
- Allows querying different record types (A, AAAA, CNAME, etc.)
- Supports the `DO` (DNSSEC OK) and `CD` (Checking Disabled) flags

## Installation

```bash
cargo install --path .
```
Or run directly
```
cargo run -- example.com
```

## Usage
Basic query
```
doh example.com
```

Specify a record type
```
doh example.com aaaa
```

Using a custom DoH server (default is [cloudflare.com](https://cloudflare-dns.com/dns-query))
```
doh example.com aaaa https://dns.google/dns-query
```

Enable DO / CD flags
```
doh example.com a --do --cd
```
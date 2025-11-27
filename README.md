# DoH Resolver 
![Crates.io Version](https://img.shields.io/crates/v/rdoh)

A simple Rust command-line DNS-over-HTTPS (DoH) client, similar to `dig`

## Features

- Resolve DNS queries over HTTPS
- Supports custom DNS servers
- Allows querying different record types (A, AAAA, CNAME, etc.)
- Supports the `DO` (DNSSEC OK) and `CD` (Checking Disabled) flags

## Installation
From source
```bash
cargo install --path .
```

From [crates.io](https://crates.io/crates/rdohhttps://crates.io/crates/rdoh)
```bash
cargo install rdoh
```

Or run directly
```
cargo run -- example.com
```

## Usage
Basic query
```
rdoh example.com
```

Specify a record type
```
rdoh example.com aaaa
```

Using a custom DoH server (default is [cloudflare.com](https://cloudflare-dns.com/dns-query))
```
rdoh example.com aaaa https://dns.google/dns-query
```

Enable DO / CD flags
```
rdoh example.com a --do --cd
```
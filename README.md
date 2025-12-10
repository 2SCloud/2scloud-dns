# 2SCloud DNS

[![Build Status](https://img.shields.io/github/workflow/status/2SCloud/2scloud-dns/CI)](https://github.com/2SCloud/2scloud-dns/actions)
[![License](https://img.shields.io/github/license/2SCloud/2scloud-dns)](LICENSE)

`2scloud-dns` is a Rust application for managing and querying DNS servers. It allows you to build and send DNS queries, analyze responses, and view DNS records conveniently.

## Features

- Send DNS queries (A, AAAA, CNAME, MX, TXT…)
- Analyze DNS responses
- Handle domain names and DNS classes/types
- Detailed result display
- Unit tests to ensure reliability

## Installation

Clone the project and build with Cargo:

```bash
git clone https://github.com/2SCloud/2scloud-dns.git
cd 2scloud-dns
cargo build --release
```

The compiled binary will be located in `target/release/2scloud-dns`.

---

## Usage

Example to run the application and query a DNS server:

```bash
./target/release/2scloud-dns --server 8.8.8.8 --query example.com A
```

Main options:

- `--server <IP>`:        Target DNS server
- `--query <DOMAIN>`: Domain name to query
- `--type <TYPE>`:         DNS record type (A, AAAA, CNAME, MX, TXT…)

---

## Tests

The application includes unit tests:

```bash
cargo test
```

To see test coverage, check `COVERAGE.md`.

<!-- COVERAGE_START --> <!-- COVERAGE_END -->

---

## Contribution

Contributions are welcome!

1. Fork the project
2. Create your branch: `git checkout -b feature/issue-XXXX` or `fix/issue-XXXX`
3. Commit your changes: `git commit -m "Add …"`
4. Push your branch: `git push origin feature/issue-XXXX` or `fix/issue-XXXX`
5. Open a Pull Request

---

## Licence

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

> This project is part of the **2SCloud** organization.
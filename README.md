<h1 align="center">2SCloud DNS</h1>

<p align="center">
  <a href="https://2scloud.github.io/scloud-dns/coverage/">
    <img src="https://img.shields.io/endpoint?url=https://2scloud.github.io/scloud-dns/coverage_badge.json" alt="Coverage">
  </a>
  <a href="https://github.com/2SCloud/scloud-dns/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/2SCloud/scloud-dns/coverage.yml?branch=main" alt="Build Status">
  </a>
  <a href="https://github.com/2SCloud/scloud-dns/issues">
    <img src="https://img.shields.io/github/issues/2SCloud/scloud-dns" alt="Issues">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/LICENSE-MIT_(Non_Commercial)-blue" alt="License">
  </a>
</p>


`scloud-dns` is a Rust application for managing and querying DNS servers. It allows you to build and send DNS queries, analyze responses, and view DNS records conveniently.

## Features

- Send DNS queries (A, AAAA, CNAME, MX, TXT…)
- Analyze DNS responses
- Handle domain names and DNS classes/types
- Detailed result display
- Unit tests to ensure reliability

## Installation

Clone the project and build with Cargo:

```bash
git clone https://github.com/2SCloud/scloud-dns.git
cd scloud-dns
cargo build --release
```

The compiled binary will be located in `target/release/scloud-dns`.

---

## Usage

Example to run the application and query a DNS server:

```bash
./target/release/scloud-dns --server 8.8.8.8 --query example.com A
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

To see test coverage, check https://2scloud.github.io/scloud-dns/coverage.

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
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

<!-- COVERAGE_START -->
<table>
<tr><th>Filename</th><th>Function Coverage</th><th>Line Coverage</th><th>Region Coverage</th><th>Branch Coverage</th></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/additional/mod.rs</td><td style="background-color:#ff3826">0.00% (0/2)</td><td style="background-color:#ff3826">0.00% (0/51)</td><td style="background-color:#ff3826">0.00% (0/97)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/answer/mod.rs</td><td style="background-color:#ff3826">0.00% (0/2)</td><td style="background-color:#ff3826">0.00% (0/51)</td><td style="background-color:#ff3826">0.00% (0/96)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/authority/mod.rs</td><td style="background-color:#ff3826">0.00% (0/2)</td><td style="background-color:#ff3826">0.00% (0/51)</td><td style="background-color:#ff3826">0.00% (0/97)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/header/mod.rs</td><td style="background-color:#ff8426">50.00% (1/2)</td><td style="background-color:#ff8426">52.78% (19/36)</td><td style="background-color:#ff8426">44.90% (22/49)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/mod.rs</td><td style="background-color:#4eff3a">100.00% (1/1)</td><td style="background-color:#ffd046">71.88% (23/32)</td><td style="background-color:#ff8426">52.46% (32/61)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/packet/question/mod.rs</td><td style="background-color:#ff8426">50.00% (1/2)</td><td style="background-color:#ff8426">48.57% (17/35)</td><td style="background-color:#ff3826">34.92% (22/63)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/records/q_class.rs</td><td style="background-color:#4eff3a">100.00% (3/3)</td><td style="background-color:#4eff3a">100.00% (24/24)</td><td style="background-color:#4eff3a">100.00% (29/29)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/records/q_name.rs</td><td style="background-color:#ff8426">50.00% (1/2)</td><td style="background-color:#ffd046">65.00% (26/40)</td><td style="background-color:#ffd046">61.19% (41/67)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/dns/records/q_type.rs</td><td style="background-color:#ff3826">33.33% (1/3)</td><td style="background-color:#ff3826">3.70% (4/108)</td><td style="background-color:#ff3826">3.60% (4/111)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/exceptions/mod.rs</td><td style="background-color:#ff3826">0.00% (0/1)</td><td style="background-color:#ff3826">0.00% (0/11)</td><td style="background-color:#ff3826">0.00% (0/11)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
<tr><td>/home/runner/work/2scloud-dns/2scloud-dns/src/main.rs</td><td style="background-color:#ff3826">0.00% (0/2)</td><td style="background-color:#ff3826">0.00% (0/28)</td><td style="background-color:#ff3826">0.00% (0/44)</td><td style="background-color:#ff3826">0.00% (0/0)</td></tr>
</table>
<!-- COVERAGE_END -->

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
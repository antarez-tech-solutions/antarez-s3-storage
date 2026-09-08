# Dependency audit triage — 2026-09-08

`cargo audit` on this repo reports 5 vulnerabilities and 3
unmaintained-dependency warnings. Findings are recorded here per the
policy that audit failures are triaged in a dated note, never
soft-skipped.

## Vulnerabilities (blocking cargo audit)

All five are **transitive via the AWS SDK** (`aws-config 1.8.11` /
`aws-sdk-s3 1.96.0` / `aws-runtime 1.5.16`), locked at the latest
release compatible with this project's `rust-version`. `cargo update`
cannot move them without a dependency major that is not yet released
for this toolchain.

| ID | Crate | Fix | Status |
|---|---|---|---|
| RUSTSEC-2026-0258 | h2 0.3.27 (unbounded empty DATA frames) | >= 0.4.16 | transitive via aws-smithy-http-client; the tree also resolves h2 0.4.19 for another consumer |
| RUSTSEC-2026-0099 | rustls-webpki 0.101.7 (wildcard name constraints) | >= 0.103.12 | transitive via rustls 0.21 -> aws-smithy-http-client; tree also resolves 0.103.15 |
| RUSTSEC-2026-0098 | rustls-webpki 0.101.7 (URI name constraints) | >= 0.103.12 | ditto |
| RUSTSEC-2026-0104 | rustls-webpki 0.101.7 (panic in CRL parsing) | >= 0.103.13 | ditto |
| RUSTSEC-2026-0009 | time 0.3.45 (DoS via stack exhaustion, medium 6.8) | >= 0.3.47 | transitive via aws-config/aws-runtime |

**Blast radius:** this crate preserves object-storage operations and
presigned URLs; the affected crates are in the AWS signing and HTTP
transport layer. The parser crates (rustls-webpki) are exercised on
TLS certificate validation of the S3 endpoint. None is in this
crate's own direct dependency surface.

**Path to clear:** bump the AWS SDK family when the dependency
maintainers release versions targeting a newer `rust-version`, or
raise the crate's MSRV. Re-run `cargo audit` afterwards.

## Unmaintained (warnings, not vulnerabilities)

| ID | Crate | Note |
|---|---|---|
| RUSTSEC-2025-0134 | rustls-pemfile 1.0.4 | transitively maintained via rustls; upstream |
| RUSTSEC-2024-0388 | derivative 2.2.0 | transitive via aws-smithy; upstream |
| RUSTSEC-2026-0173 | proc-macro-error2 2.0.1 | transitive via aws-smithy; upstream |
| RUSTSEC-2024-0436 | paste 1.0.15 | transitive via aws-smithy; upstream |

These clear when the aws-smithy family updates its proc-macro and
pemfile dependencies; no action available in this repo.

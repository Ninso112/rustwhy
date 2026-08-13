# Security Policy

## Supported Versions

RustWhy is under active development and only the latest release receives
security fixes.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Yes             |
| < 0.1   | ❌ No              |

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security-relevant bugs
(privilege escalation, information disclosure via `/proc`/`/sys` parsing,
shell injection in `recommendation` strings, etc.).

Instead:

1. Email the maintainer at the address shown on the
   [Ninso112 GitHub profile](https://github.com/Ninso112), with
   subject line **`[rustwhy] security report`**.
2. Include:
   - The commit SHA (or `rustwhy --version` output) you tested against.
   - A minimal reproduction.
   - The expected impact (what an attacker could do).
3. You will receive an acknowledgement within **72 hours**.

We follow the [GitHub vulnerability disclosure guidance](https://docs.github.com/en/code-security/security-advisories).
If you'd like to be credited in the release notes, please say so in the
initial report.

## Security Design Notes

- RustWhy is **read-only** on the host: it parses `/proc`, `/sys`,
  and a handful of system commands (`systemd-analyze`, `nvidia-smi`, ...).
- It does not write to disk outside of `--json` output.
- No network connections are made unless a module explicitly requires
  one (e.g. `net --host`).
- External command execution is bounded by `run_cmd_timeout` to prevent
  hung subprocesses from blocking the diagnostic.

If you discover that any of these invariants are violated, treat it as a
security issue and report it via the process above.

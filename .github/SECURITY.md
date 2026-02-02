# Security Policy

## Supported Versions

We currently support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of RustWhy seriously. If you discover a security vulnerability, please follow these steps:

### How to Report

1. **Do NOT** open a public issue for security vulnerabilities
2. Send an email to the maintainers with details of the vulnerability
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if available)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Assessment**: We will assess the vulnerability and determine its severity
- **Updates**: We will keep you informed of our progress
- **Fix Timeline**: We aim to release fixes for critical vulnerabilities within 7 days
- **Credit**: We will credit you for the discovery (unless you prefer to remain anonymous)

## Security Best Practices

When using RustWhy:

### Running as Root

Some diagnostic modules require root privileges. When running with `sudo`:

- Review the command before execution
- Only run RustWhy from trusted sources
- Verify the binary's integrity before running as root

### Input Validation

RustWhy processes system information and user inputs:

- Path arguments are validated to prevent directory traversal
- Command injection is prevented through proper argument handling
- User inputs are sanitized before being used in system calls

### Dependencies

We regularly audit our dependencies for known vulnerabilities:

- Run `cargo audit` to check for security issues
- Update dependencies promptly when vulnerabilities are discovered
- Monitor GitHub security advisories

## Known Security Considerations

### System Information Exposure

RustWhy collects and displays system information:

- Be cautious when sharing output in public forums
- JSON output may contain sensitive system details
- Consider redacting hostname, IPs, and other identifying information

### Privilege Escalation

Some modules require elevated privileges:

- `boot` module: requires systemd access
- `io` module: may require root for complete I/O statistics
- `gpu` module: may require specific permissions for GPU access

We recommend running only the specific modules you need rather than using `rustwhy all` with root privileges.

## Security Features

### Input Sanitization

- All external commands use argument arrays (not shell strings)
- Path inputs are canonicalized and validated
- No arbitrary code execution from user inputs

### Minimal Permissions

- Modules declare required permissions upfront
- Graceful degradation when permissions are insufficient
- Clear error messages about missing permissions

### Safe Rust

- Written in safe Rust with minimal `unsafe` blocks
- Regular Clippy lints to catch potential issues
- Comprehensive error handling

## Disclosure Policy

When a security vulnerability is fixed:

1. We will prepare a security advisory
2. Credit will be given to the reporter (if desired)
3. A new version will be released with the fix
4. The vulnerability details will be published after users have had time to update

## Contact

For security concerns, please contact the project maintainers through:
- GitHub Security Advisories: https://github.com/Ninso112/rustwhy/security/advisories
- GitHub Issues (for non-sensitive security questions)

## Security Updates

Subscribe to the following to stay informed about security updates:
- GitHub releases: https://github.com/Ninso112/rustwhy/releases
- GitHub security advisories
- Watch the repository for security announcements
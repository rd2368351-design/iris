# Mailbox — Enterprise Mail Server

[![CI](https://github.com/mailbox/mailbox/actions/workflows/ci.yml/badge.svg)](https://github.com/mailbox/mailbox/actions)
[![Go Report](https://goreportcard.com/badge/github.com/mailbox/mailbox)](https://goreportcard.com/report/github.com/mailbox/mailbox)
[![License](https://img.shields.io/badge/license-AGPL%203.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](CHANGELOG.md)

Mailbox is a complete, production-ready enterprise mail server written in Go.
It supports SMTP, IMAP, JMAP, POP3, CalDAV, CardDAV, and ManageSieve protocols
with multi-tenancy, clustering, and enterprise compliance features.

## Features

- **SMTP Server** — Inbound/outbound with DKIM, SPF, DMARC, DANE
- **IMAP Server** — 30+ commands, IDLE, CONDSTORE, QRESYNC
- **JMAP Server** — RFC 8620/8621 mail, calendar, contacts
- **POP3 Server** — Legacy email retrieval
- **CalDAV/CardDAV** — Calendar and contacts sync
- **ManageSieve** — Server-side email filtering
- **Multi-Tenant** — Isolated organizations
- **Clustering** — Raft consensus, gossip, auto-failover
- **Security** — Argon2id, JWT, OAuth2, SAML, LDAP
- **Anti-Spam** — Bayesian, rules, DNSBL, greylisting
- **Compliance** — GDPR, HIPAA, SOC2, retention, eDiscovery
- **Observability** — Prometheus, OpenTelemetry, structured logging

## Quick Start

### Docker
```bash
docker run -d \
  --name mailbox \
  -p 25:25 -p 143:143 -p 443:443 -p 587:587 \
  -v mailbox-data:/data \
  mailbox/mailbox:latest
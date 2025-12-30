# GMKCF: Give Me KC File

**GMKCF** is a stateless Rust microservice designed to mint `.kc` (Killcode) authentication containers. It takes standard image inputs (JPEG, PNG, WebP) and encapsulates them into a cryptographically secure, versioned binary format.

## Core Features

- **Stateless Sealing**: .kc container creation.
- **Media Only**: Strictly validates image inputs using `SupportedMediaTypes`.
- **Cryptography**: Uses standard primitives (X25519 for Key Wrapping, XChaCha20-Poly1305 for content encryption).

## Technical Documentation

For deep technical details, please refer to the engineering specifications:

- [Engineering Spec (RFC 001)](./docs/technical/RFC_001_KC_CONTAINER.md): Complete protocol definition.
- [Binary Schema (KSY)](./docs/technical/kc_format.ksy): Kaitai Struct formal definition of the `.kc` file layout.

## Quick Start (Docker)

To run the full stack including GMKCF:

```bash
docker compose up gmkcf
```
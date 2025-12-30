# RFC 001: KillCode Container Format (.kc)

**Status**: Draft
**Version**: 1.0.0
**Author**: KillCode Architecture Team

## 1. Introduction

The KillCode Container (`.kc`) is a specialized binary file format designed to support the "Rolling Key" DRM architecture. Its primary purpose is to decouple the **Key Material** from the **Media Content** in a way that allows the Key Material to be physically mutated (rotated) without needing to re-process the entire media payload (unless full re-encryption is specifically requested).

## 2. Terminology

- **Sealer**: The entity (GMKCF) that creates the initial `.kc` file.
- **Opener**: The entity (Client App) that reads and unlocks the `.kc` file.
- **Genesis Key**: The initial Asymmetric Public Key used to lock the file.
- **File Key (SymKey)**: The Ephemeral Symmetric Key used to encrypt the potentially large media body.
- **Wrapper**: The process of encrypting the _File Key_ with the _Genesis Key_.

## 3. File Structure

The file adheres to the formal Kaitai Struct definition found in `kc_format.ksy`.

### 3.1. Overview

```
+----------------+----------------+--------------------------------+---------------------------+
| Magic (4 bytes)| Len (4 bytes)  | Header JSON (N bytes)          | Body (Remaining bytes)    |
+----------------+----------------+--------------------------------+---------------------------+
| "KCF\x01"      | Big Endian u32 | {"algo":..., "enc_key":...}    | [Encrypted Binary Blob]   |
+----------------+----------------+--------------------------------+---------------------------+
```

### 3.2. Fields

- **Magic**: `0x4B 0x43 0x46 0x01` (ASCII "KCF" + byte 0x01). Identifies the file type and version.
- **Header Length**: A 32-bit unsigned integer (Big Endian). dictates the byte size of the following JSON section.
- **Header JSON**: A UTF-8 encoded JSON object.
  - `algo`: (String) Identifies the crypto suite used (e.g., `x25519-xchacha20`).
  - `enc_sym_key`: (String/Hex) The File Key, encrypted using the Asymmetric Algorithm defined in `algo`.
  - `nonce`: (String/Hex) The Nonce used for the Symmetric Body encryption.
- **Body**: The raw output of the Symmetric Encryption cipher.

## 4. Security Considerations

### 4.1. Hybrid Encryption

We use a **Hybrid Encryption** scheme:

1.  **Symmetric**: XChaCha20-Poly1305 is used for the Body. It is fast, supports streaming, and has a large nonce (192-bit) which allows random nonce generation without risk of collision.
2.  **Asymmetric**: X25519 (Elliptic Curve) is used for Key Wrapping. It is efficient and produces small keys (32 bytes).

### 4.2. Integrity

The Poly1305 MAC (Message Authentication Code) inherent in the AEAD cipher ensures the integrity of the Body. If the Body is tampered with, the decryption will fail.
The Header JSON is _not_ currently signed in this version, but tampering with the `enc_sym_key` or `nonce` will usually result in a decryption failure (MAC error) or garbage output.

## 5. Implementation Notes

- **Endianness**: All binary numbers are **Big Endian** (Network Byte Order).
- **String Encoding**: UTF-8.
- **Hex Encoding**: Lowercase is preferred but not enforced by the spec.

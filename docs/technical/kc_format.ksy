meta:
  id: kc_container
  file-extension: kc
  endian: be
  doc: |
    KillCode (KC) Secure Media Container Format.
    Designed for "Rolling Key" DRM, where the file structure allows
    header metadata (containing encrypted keys) to be easily parsed and rotated
    while keeping the heavy encrypted media body opaque.

seq:
  - id: magic
    contents: "KCF\x01"
    doc: Magic bytes (KillCode File v1).

  - id: header_len
    type: u4
    doc: Length of the JSON header in bytes (Big Endian).

  - id: header_json
    type: str
    size: header_len
    encoding: UTF-8
    doc: |
      Metadata Header in JSON format.
      Must contain:
      - algo: String (e.g. "x25519-xchacha20")
      - enc_sym_key: Hex String (The File Key encrypted with the Recipient's Public Key)
      - nonce: Hex String (The Nonce used for the Body encryption)
      
      May contain:
      - genealogy: String/Object (Tracking info)

  - id: body
    size-eos: true
    doc: |
      The encrypted media payload.
      Encrypted using the Symmetric Key (which is inside `enc_sym_key`)
      and the `nonce`.
      Algorithm: XChaCha20-Poly1305 (usually).

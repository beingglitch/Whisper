# whisper-sync

P2P encrypted `.env` sync — no central server, no cloud, no trust required.

Built with Rust + libp2p. Published on [crates.io](https://crates.io/crates/whisper-sync).

---

## What it does

Syncs `.env` files directly between developer machines over an encrypted peer-to-peer connection. No Doppler, no Infisical, no server you have to trust or pay for.

---

## Install

```bash
cargo install whisper-sync
```

---

## Quickstart

**On every machine, once:**

```bash
whisper init
# Generated new identity: 12D3KooWBF1SAqhp5TDaq2OQLjxpKqpoLjeL4cYvEbFmugLXvRZ
```

**Share your PeerId with your teammate out of band (Slack, email, anything).**

**Each developer adds the other as a trusted peer:**

```bash
# Developer A
whisper peer add 12D3KooWDbrwfC5Zp5c9u4AarnUNraCJ7qfSwdBE3KWxuWhsqpR

# Developer B
whisper peer add 12D3KooWBF1SAqhp5TDaq2OQLjxpKqpoLjeL4cYvEbFmugLXvRZ
```

**Sync:**

```bash
# Developer A — serves their .env
whisper sync push

# Developer B — fetches and writes to disk
whisper sync pull
```

---

## Commands

```
whisper init              Generate identity, create .whisper/
whisper id                Print your PeerId

whisper peer add <id>     Add a trusted peer
whisper peer remove <id>  Remove a peer
whisper peer list         List all trusted peers

whisper sync push         Serve your .env to peers
whisper sync pull         Fetch .env from a peer and sync to disk
```

---

## How it works

```
whisper sync push                    whisper sync pull
      |                                     |
      | listens on TCP                      | dials push peer
      |                                     |
      |<====== Noise handshake ============>|
      |         identity verified           |
      |                                     |
      |<------- EnvRequest ---------------  |
      |                                     |
      | check peer in .whisper/peers.toml   |
      | read .env                           |
      |                                     |
      |-------- EnvResponse ------------->  |
      |         key=value pairs             |
      |                                     |
      |                              sync_env()
      |                              write .env to disk
```

- Transport encrypted with **Noise protocol**
- Multiplexed with **yamux**
- Identity is a persistent **Ed25519 keypair** stored in `.whisper/identity.pk8`
- Only peers in `.whisper/peers.toml` can request your `.env`

---

## Project structure

```
.whisper/
    identity.pk8     ← your keypair, never commit this
    peers.toml       ← trusted peer list
```

Add to your `.gitignore`:

```
.whisper/identity.pk8
.env
```

---

## Roadmap

- [x] P2P connection over libp2p
- [x] Encrypted transport (Noise)
- [x] Persistent identity (Ed25519)
- [x] Structured .env parsing and sync
- [x] Peer allowlist (authorization)
- [x] CLI — init, sync, peer management
- [ ] DHT peer discovery (no manual address sharing)
- [ ] NAT traversal (relay + DCUtR)
- [ ] Application-layer encryption (ChaCha20-Poly1305)
- [ ] Conflict resolution
- [ ] Audit trail

---

## Built with

- [libp2p](https://github.com/libp2p/rust-libp2p) — P2P networking
- [clap](https://github.com/clap-rs/clap) — CLI
- [serde](https://serde.rs) — serialization
- [tokio](https://tokio.rs) — async runtime

---

## License

GNU
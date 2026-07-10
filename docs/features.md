# Whisper — Feature Tracker

P2P encrypted `.env` sync over libp2p. Zero central server. Built in Rust.

**Status legend:** `[x]` done · `[ ]` planned · `[~]` in progress · `[!]` bug · `[-]` dropped/deferred  
**Tested legend (second box):** `[x]` tested · `[ ]` untested · `[~]` partial

---

## 1. Identity & Cryptography

> Ed25519 keypair per peer. Stable PeerId across restarts. Foundation of all auth.

- [x] `[ ]` Generate Ed25519 keypair on `whisper init`
- [x] `[ ]` Save keypair to `.whisper/identity.pk8` (protobuf encoding)
- [x] `[ ]` Load existing keypair on every subsequent run
- [x] `[ ]` Derive stable PeerId from public key
- [x] `[ ]` Print PeerId on `whisper id`
- [x] `[ ]` Separate bootstrap identity at `.whisper/bootstrap/identity.pk8`
- [ ] `[ ]` Add `.whisper/identity.pk8` to `.gitignore` automatically on `whisper init`
- [ ] `[ ]` **Payload encryption** — ChaCha20-Poly1305 (AEAD) per message
  - [ ] Derive 32-byte key from shared passphrase via Argon2
  - [ ] Generate fresh nonce per message
  - [ ] Encrypt `env_variables` before sending in `EnvResponse`
  - [ ] Decrypt on receive, fail loudly on tamper
- [ ] `[ ]` **Key rotation** — `whisper keys rotate`
  - [ ] Generate new keypair
  - [ ] Re-encrypt all stored state
  - [ ] Notify known peers of new PeerId

---

## 2. Networking — Transport Stack

> TCP → Noise (mutual auth + forward secrecy) → yamux (multiplexing)

- [x] `[ ]` TCP transport via libp2p
- [x] `[ ]` Noise XX handshake — mutual authentication + channel encryption
- [x] `[ ]` yamux multiplexing — multiple streams over one connection
- [x] `[ ]` DNS transport (`.with_dns()`) for dnsaddr bootstrap addresses
- [x] `[ ]` Persistent connection with configurable idle timeout
- [ ] `[ ]` QUIC transport (faster, lower latency) — future
- [ ] `[ ]` WebSocket transport (browser/restrictive network compat) — future

---

## 3. Peer Discovery

> How peers find each other's current address without manual sharing.

### 3a. Bootstrap Node (done)
- [x] `[ ]` Separate `whisper-bootstrap` binary (`src/bin/bootstrap.rs`)
- [x] `[ ]` Stable identity loaded from disk
- [x] `[ ]` Kademlia server mode — stores records for peers
- [x] `[ ]` Listens on fixed port 14550
- [x] `[ ]` Logs peer connections

### 3b. Kademlia DHT (partially working)
- [x] `[ ]` `WhisperBehaviour` combining `request_response` + `kad::Behaviour`
- [x] `[ ]` `add_address()` registers bootstrap in Kademlia routing table
- [x] `[ ]` `kademlia.bootstrap()` fills routing table (called once via flag)
- [x] `[ ]` `OutboundQueryProgressed` event handled
- [~] `[ ]` `get_closest_peers()` query for target peer address
- [!] `[ ]` `GetClosestPeers` returns empty in small networks — peers not publishing records
- [ ] `[ ]` **Provider pattern** — push peer calls `start_providing(key)`, pull peer calls `get_providers(key)`; more reliable than `get_closest_peers` for named discovery
- [ ] `[ ]` Address dialing from DHT result — `swarm.dial(peer_id)` after Kademlia populates routing table

### 3c. mDNS — Local Network (planned)
- [ ] `[ ]` Add `mdns` to libp2p features
- [ ] `[ ]` Add `mdns::tokio::Behaviour` to `WhisperBehaviour`
- [ ] `[ ]` Handle `mdns::Event::Discovered` — dial discovered peers automatically
- [ ] `[ ]` Works offline, zero config, zero internet required
- [ ] `[ ]` Falls back to DHT when not on same LAN

### 3d. NAT Traversal (planned)
- [ ] `[ ]` Relay nodes — route through third peer when direct connection fails
- [ ] `[ ]` DCUtR hole-punching — establish direct connection behind NAT
- [ ] `[ ]` Add `relay` and `dcutr` to libp2p features
- [ ] `[ ]` Test across two machines on different home networks

---

## 4. Sync Protocol

> request-response over libp2p. Pull model: dialer requests, listener serves.

- [x] `[ ]` `EnvRequest {}` — empty request struct
- [x] `[ ]` `EnvResponse { env_variables: HashMap<String, String>, message: String }` — response
- [x] `[ ]` `ISensorPlugin`-style protocol identifier `/env/1.0.0`
- [x] `[ ]` Listener reads `.env`, parses key-value pairs, sends response
- [x] `[ ]` Dialer receives response, calls `sync_env`, writes back to `.env`
- [x] `[ ]` Peer authorization check — reject requests from peers not in `peers.toml`
- [ ] `[ ]` **Richer sync protocol** — v2 when versioning is added
  - [ ] Request includes requester's current state (timestamps per key)
  - [ ] Response includes only changed keys since last sync (delta)
  - [ ] Protocol identifier `/whisper/sync/2.0.0`
- [ ] `[ ]` Graceful error response — unauthorized vs server error vs not found
- [ ] `[ ]` Timeout handling — if peer doesn't respond within N seconds, fail cleanly

---

## 5. .env Handling

> Parse, merge, write. Key-value aware. Not line-aware.

- [x] `[ ]` Parse `.env` → `HashMap<String, String>`
  - [x] Skip blank lines
  - [x] Skip `#` comments
  - [x] Split on first `=` only (`splitn(2, '=')`)
- [x] `[ ]` `sync_env` — merge received keys into local `.env`, write back
- [x] `[ ]` `write_env` — serialize `HashMap` back to `KEY=VALUE` format
- [ ] `[ ]` Preserve comments and ordering on write (currently loses them)
- [ ] `[ ]` Handle quoted values — `KEY="value with spaces"`
- [ ] `[ ]` Handle multiline values — `KEY="line1\nline2"`
- [ ] `[ ]` Warn on malformed lines (no `=`) rather than silently dropping

---

## 6. Per-Key Versioning & Conflict Resolution (designed, not yet built)

> Key insight: merge at the KEY level, not the line level. Structured data.

### 6a. Data Model
- [ ] `[ ]` `state.json` — your own state, one entry per key
  ```json
  {
    "my_peer_id": "12D3Koo...",
    "variables": {
      "DATABASE_URL": {
        "value": "postgres://...",
        "updated_at": "2025-06-30T14:00:00Z",
        "updated_by": "12D3Koo...self"
      }
    }
  }
  ```
- [ ] `[ ]` `gathered.json` — what you've learned about other peers' values
  ```json
  {
    "DATABASE_URL": [
      { "peer_id": "...", "value": "...", "value_hash": "...", "updated_at": "..." }
    ]
  }
  ```
- [ ] `[ ]` Both files persistent in `.whisper/`
- [ ] `[ ]` SHA-256 hash of each value for fast conflict grouping

### 6b. Comparison Logic
- [ ] `[ ]` `build_sync_report(state, gathered) -> SyncReport` — pure function, no I/O
- [ ] `[ ]` Per-key cases:
  - [ ] **Case A** — all peers have same value → in sync, no action
  - [ ] **Case B** — values differ across peers → conflict, show user
  - [ ] **Case C** — key exists on peers but not locally → offer to add
  - [ ] **Case D** — key only local, not on peers → keep silently (added by you)
- [ ] `[ ]` Group by `value_hash` per key
- [ ] `[ ]` Count peers per group (popularity)
- [ ] `[ ]` Newest `updated_at` per group (recency)

### 6c. Conflict UI
- [ ] `[ ]` `whisper sync pull` shows resolved keys silently, only surfaces conflicts
- [ ] `[ ]` Per-conflict display:
  ```
  DATABASE_URL — 2 versions

    postgres://new...   Alice         1 peer    2 hours ago  ← newest
    postgres://old...   Bob, Carol    2 peers   3 days ago

  → [1] take Alice's  [2] take majority  [s] skip: _
  ```
- [ ] `[ ]` User chooses per key, never auto-decided by tool
- [ ] `[ ]` Popularity shown as hint, never used to auto-resolve

---

## 7. Central Server Sync (planned)

> Online path when internet available. Encrypted blob storage, not plaintext.

- [ ] `[ ]` Simple HTTP REST API server (separate deployment)
  - [ ] `POST /env/:peer_id` — upload encrypted state blob
  - [ ] `GET  /env/:peer_id` — download latest blob
  - [ ] Auth via Ed25519 signature on request
- [ ] `[ ]` Client: auto-upload on change when internet available
- [ ] `[ ]` Client: auto-download on start when internet available
- [ ] `[ ]` Server stores only encrypted blobs — never sees plaintext values
- [ ] `[ ]` Fall through to LAN sync if server unreachable

---

## 8. Background Daemon (planned)

> Long-running process. Syncs opportunistically. Data ready before user asks.

- [ ] `[ ]` `whisper daemon` — starts background process
- [ ] `[ ]` Watches for peer connections (mDNS + DHT)
- [ ] `[ ]` Auto-fetches peer state on connection, writes to `gathered.json`
- [ ] `[ ]` Auto-pushes to central server when internet available
- [ ] `[ ]` `whisper sync pull` reads already-gathered data, fast
- [ ] `[ ]` PID file at `.whisper/daemon.pid`
- [ ] `[ ]` Log file at `.whisper/daemon.log`
- [ ] `[ ]` `whisper daemon stop`
- [ ] `[ ]` System service integration (systemd / launchd) — optional

---

## 9. CLI

> Built with `clap` derive. All commands below.

- [x] `[ ]` `whisper init` — generate identity, create `.whisper/`
- [x] `[ ]` `whisper id` — print PeerId
- [x] `[ ]` `whisper peer add <peer_id>` — add to allowlist
- [x] `[ ]` `whisper peer remove <peer_id>` — remove from allowlist
- [x] `[ ]` `whisper peer list` — list trusted peers
- [x] `[ ]` `whisper sync push` — serve `.env` to connecting peers
- [x] `[ ]` `whisper sync pull` — connect to peers and fetch `.env`
- [ ] `[ ]` `whisper status` — show identity, peers, last sync, daemon status
- [ ] `[ ]` `whisper daemon` / `whisper daemon stop`
- [ ] `[ ]` `whisper keys rotate` — new keypair, re-encrypt
- [ ] `[ ]` `--project <path>` flag — run against a different directory
- [ ] `[ ]` Human-friendly error messages (not raw Rust panics)

---

## 10. Distribution & Install

- [ ] `[ ]` Fill `Cargo.toml` metadata — description, license, repository, keywords, categories
- [ ] `[ ]` Publish `whisper-sync` to crates.io — `cargo install whisper-sync`
- [ ] `[ ]` GitHub Actions CI — build + test on push
- [ ] `[ ]` Cross-compile for Linux x86_64, macOS arm64, Windows x86_64
- [ ] `[ ]` GitHub Releases with pre-built binaries (via `cargo-dist` or `cross`)
- [ ] `[ ]` One-line install script for non-Rust users
- [ ] `[ ]` Homebrew formula — future

---

## 11. README & Docs

- [ ] `[ ]` README.md — what it is, why it exists vs Doppler/Infisical, install, usage
- [ ] `[ ]` Architecture section — libp2p stack, Noise, Ed25519, how peers find each other
- [ ] `[ ]` Security model — what's encrypted, what's not, trust assumptions
- [ ] `[ ]` Honest scope — what v0.1 does and doesn't do
- [ ] `[ ]` Roadmap section — versioning, daemon, server, NAT
- [ ] `[ ]` CONTRIBUTING.md
- [ ] `[ ]` CHANGELOG.md

---

## Phase Summary

| Phase | Scope | Status |
|-------|-------|--------|
| **v0.1** | Init, peer management, local push/pull, peer auth | ~Done |
| **v0.2** | DHT discovery working end-to-end + mDNS | In progress |
| **v0.3** | Payload encryption (ChaCha20-Poly1305) | Planned |
| **v0.4** | Per-key versioning + conflict resolution UI | Designed |
| **v0.5** | Central server sync (online path) | Planned |
| **v0.6** | Background daemon | Planned |
| **v1.0** | NAT traversal, crates.io publish, binaries | Planned |
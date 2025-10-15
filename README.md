# TIDasONE

[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/TangoisdownHQ/TIDasONE/rust.yml?branch=main)](https://github.com/TangoisdownHQ/TIDasONE/actions)
[![License](https://img.shields.io/github/license/TangoisdownHQ/TIDasONE)](./LICENSE)
[![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)](./CONTRIBUTING.md)
![CyberSpaceOps](https://img.shields.io/badge/Secured_by-Tangoisdown_Systems-1b5e20?logo=shield&logoColor=white)
![TangoisdownHQ](https://img.shields.io/badge/TangoisdownHQ-Cyber_Intelligence-002b36?logo=linux&logoColor=white)

---


# TIDasONE 🚀

**TIDasONE** is a decentralized cybersecurity and logistics ecosystem designed for a post-quantum, 
multi-planetary future.  
It combines secure communications, token-powered commerce, Logistics, 
and cross-domain coordination of terrestrial and interplanetary assets.

---

## 🌌 Vision

TIDasONE provides:
- **Secure Communications (CommSec)** – Post-quantum key exchange & AEAD for confidential, authenticated channels.
- **Decentralized Commerce (SupplyLink)** – Token-powered marketplace and logistics platform.
- **Interplanetary Logistics (AstroNet)** – A registry and coordination layer for interplanetary supply chains.
- **Unified Token Economy (TIDasToken)** – A utility token powering cybersecurity, logistics, and commerce inside the TIDasONE ecosystem.

---

## 🧩 Modules

### 🔐 CommSec
- **Status**: Implemented ✅
- Features:
  - Post-quantum KEM (Kyber-like) for key agreement.
  - AES-256-GCM AEAD encryption/decryption with optional Associated Data (AD).
  - REST API endpoints for keypair, encapsulate, decapsulate, encrypt, decrypt.
  - Tested with `scripts/test_commsec.sh`.

### 📦 SupplyLink
- **Status**: Planned 🛠
- Token-powered decentralized commerce & logistics layer.

### 🚀 AstroNet
- **Status**: Planned 🛠
- Interplanetary logistics registry and secure routing system.

### 💠 TIDasToken
- **Status**: Live / external
- Utility token for payments, staking, and powering ecosystem services.

---

## 🛠 Build & Run

Requirements:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- `cargo`
- `jq`
- `curl`

Run the CommSec API:

```bash
cargo run --bin api
This will start the API on:

arduino
Copy code
http://127.0.0.1:3000/commsec
🔬 Testing
Run the end-to-end test script:

bash
Copy code
./scripts/test_commsec.sh
The script validates:

KEM keypair generation

Encapsulation / Decapsulation (shared secret agreement)

AEAD encryption / decryption (with and without Associated Data)

Tampered AD rejection

Sample output:

csharp
Copy code
[1] Requesting KEM keypair...
[2] Encapsulating...
[3] Decapsulating...
✅ Shared secrets match
[4] AEAD encrypt/decrypt...
✅ AEAD round-trip success
[5] AEAD with Associated Data...
✅ AEAD with AD round-trip success
[6] AEAD with Wrong Associated Data...
✅ AEAD rejected tampered associated data
📍 Roadmap
 CommSec PQC KEM + AEAD API

 Add structured logging & metrics

 Persistence layer (key/session storage)

 SupplyLink decentralized logistics module

 AstroNet interplanetary registry

 Integration with TIDasToken

📜 License
MIT or Apache-2.0 (to be decided).

🌍 Community
We are looking for early adopters, testers, and collaborators.

Developers interested in cybersecurity, blockchain, logistics, or space systems.

Companies exploring post-quantum security and interplanetary commerce & Logistics.

---

 -- Project Structure


TIDasONE/
├── apps/
│   ├── api/                          # Rust API crate
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs               # entrypoint (runs `api`)
│   │   │   ├── routes/
│   │   │   │   ├── mod.rs            # central router
│   │   │   │   ├── user.rs           # user endpoints
│   │   │   │   ├── inventory.rs      # inventory endpoints
│   │   │   │   ├── packages.rs       # package endpoints
│   │   │   │   ├── auth.rs           # auth endpoints
│   │   │   │   ├── auth_middleware.rs# JWT / claims middleware
│   │   │   │   └── commsec.rs        # 🔐 CommSec (PQ KEM + SIG)
│   │   │   ├── lib.rs (if exists)    # shared helpers
│   │   │   └── ...
│   │   └── ...
│   ├── db/                           # db helper crate (binary target `db`)
│   ├── gen_jwt/                      # utility crate (binary target `gen_jwt`)
│   └── ...
├── scripts/
│   ├── test_commsec.sh               # 🔐 test harness for PQ handshake + AEAD
│   └── ...                           # (can add more test_*.sh later)
├── docker-compose.yml                 # (your DB + infra services)
└── ...

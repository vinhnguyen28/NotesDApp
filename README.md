# Stellar Notes DApp

A decentralized note-taking application built on the **Stellar** blockchain using the **Soroban SDK**. This smart contract allows users to create, retrieve, and delete notes while ensuring that all data is stored transparently and securely on-chain.

---

## Overview

Stellar Notes DApp demonstrates how decentralized applications can replace traditional centralized storage with blockchain-based data management.

Instead of relying on external databases, every note is stored directly in the smart contract's persistent storage. This approach provides transparency, data integrity, and decentralized ownership while leveraging the speed and low transaction costs of the Stellar network.

---

## Features

### Create Notes

* Create notes with a title and content.
* Automatic unique ID generation.
* Persistent on-chain storage.

### View Notes

* Retrieve all stored notes.
* Easy integration with frontend applications.
* Real-time blockchain state.

### Delete Notes

* Remove notes by their unique ID.
* Efficient storage management.
* Instant blockchain state update.

### Secure On-Chain Storage

* Notes are managed exclusively through smart contract functions.
* Immutable transaction history.
* Transparent blockchain verification.

---

## Smart Contract Functions

| Function                      | Description               |
| ----------------------------- | ------------------------- |
| `create_note(title, content)` | Creates a new note.       |
| `get_notes()`                 | Returns all stored notes. |
| `delete_note(id)`             | Deletes a note by its ID. |

---

## Technology Stack

* **Rust**
* **Soroban SDK**
* **Stellar Blockchain**
* **Soroban Smart Contracts**

---

## Project Vision

The goal of this project is to demonstrate how blockchain technology can be used for decentralized personal data management.

Our objectives include:

* Give users full ownership of their notes.
* Remove dependence on centralized databases.
* Ensure transparency and data integrity.
* Build a secure and trustless storage system.
* Provide a lightweight example of Soroban smart contract development.

---

## Future Improvements

Planned enhancements include:

* User authentication and ownership verification.
* Private or encrypted notes.
* Categories and tags.
* Markdown support.
* Search functionality.
* Note editing and version history.
* Shared notes with access permissions.
* IPFS integration for large content.
* Decentralized frontend hosting.
* Cross-chain compatibility.

---

## Deployment

1. Clone this repository.
2. Build the smart contract using Soroban.
3. Deploy it to the Stellar Testnet or Mainnet.
4. Interact with the contract through Soroban CLI or a compatible frontend.

---

## Contract

**Network:** Stellar Soroban

**Contract Address**

```text
CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M
```

---

## Project Structure

```text
src/
 ├── lib.rs          # Smart contract implementation
 ├── storage.rs      # Storage utilities (optional)
 ├── types.rs        # Data structures (optional)

Cargo.toml
README.md
```

---

## License

This project is released under the MIT License.

---

**Stellar Notes DApp** showcases a simple yet practical decentralized application built with Rust and Soroban, providing a foundation for more advanced blockchain-based productivity tools.

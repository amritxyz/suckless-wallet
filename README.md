# 👁️ Drishti Protocol (दृष्टि)

**Fiscal Integrity & Transparency Engine for the Government of Nepal**

Drishti is a high-assurance fiscal monitoring protocol built on the **Solana Blockchain**. It leverages **Anchor 0.30.x** and the **Token-2022 (Token Extensions)** standard to provide real-time, immutable auditing of government fund allocations, ensuring that every Rupee is traceable from the Ministry of Finance to the local level.

---

## 🏛️ Project Architecture

Drishti utilizes a hierarchical Multi-Sig and Program Derived Address (PDA) structure to manage state:

* **Registry Program:** Maintains a verified list of Government Entities (Ministries, Departments, Municipalities).
* **Budget Program:** Handles the allocation of "Digital NPR" using Token-2022 extensions for **Permanent Delegates** and **Transfer Hooks** (enabling automated tax/fee collection).
* **Audit Vaults:** Cryptographically locked accounts that store transaction metadata for public oversight.

## 🛠️ Tech Stack

* **L1 Blockchain:** Solana
* **Framework:** [Anchor 0.30.0](https://www.anchor-lang.com/)
* **Token Standard:** [Token-2022](https://spl.solana.com/token-2022)
* **Language:** Rust
* **Testing:** TypeScript (Vitest/Jest) & Bankrun

---

## 🚀 Getting Started

### Prerequisites

Ensure your environment matches the Lead Engineer's Audit:
* **Rust:** `1.75.0+`
* **Solana CLI:** `1.18.x+` (Agave)
* **Anchor CLI:** `0.30.0`
* **Node.js:** `20.x+`

### Installation

1.  **Clone the Repository:**
    ```bash
    git clone [https://github.com/drishti-nepal/protocol-core.git](https://github.com/drishti-nepal/protocol-core.git)
    cd protocol-core
    ```

2.  **Install Dependencies:**
    ```bash
    yarn install
    ```

3.  **Build the Program:**
    ```bash
    anchor build
    ```

### Local Development

To run the test suite against a local validator:
```bash
# Start validator with Token-2022 support
solana-test-validator

# Run tests
anchor test 

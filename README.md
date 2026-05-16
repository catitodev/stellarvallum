<div align="center">

<!-- Banner -->
<img src="assets/image/stelar_vallum_banner.png" alt="StellarVallum Banner" width="100%"/>

<br/>

<!-- Logo + Animated Title -->
<img src="assets/image/stelar_vallum_logo.png" alt="StellarVallum Logo" width="140"/>

<a href="https://github.com/catitodev/stellarvallum">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=30&duration=2500&pause=800&color=00D4FF&center=true&vCenter=true&multiline=true&repeat=true&width=650&height=90&lines=%E2%9A%A1+StellarVallum;The+First+Wall+for+Soroban;Security+Scanner+for+Stellar+%2B+Soroban" alt="StellarVallum"/>
</a>

<br/>

**Framework de Segurança Testnet-First para o Ecossistema Stellar + Soroban**

<br/>

<!-- Animated Badges -->
<a href="https://github.com/catitodev/stellarvallum"><img src="https://img.shields.io/badge/version-0.2.0--testnet-00D4FF?style=for-the-badge&logo=stellar&logoColor=white&labelColor=0D1117" alt="Version"/></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache%202.0-green?style=for-the-badge&labelColor=0D1117" alt="License"/></a>
<a href="https://rust-lang.org"><img src="https://img.shields.io/badge/rust-1.74%2B-orange?style=for-the-badge&logo=rust&logoColor=white&labelColor=0D1117" alt="Rust"/></a>
<a href="https://scs.owasp.org/sctop10/"><img src="https://img.shields.io/badge/OWASP-SC%20Top%2010%202026-7B2D8B?style=for-the-badge&labelColor=0D1117" alt="OWASP"/></a>
<a href="https://developers.stellar.org"><img src="https://img.shields.io/badge/network-Stellar%20Testnet-FFD700?style=for-the-badge&logo=stellar&logoColor=white&labelColor=0D1117" alt="Network"/></a>

<br/><br/>

<!-- Quick Stats -->
<img src="https://img.shields.io/badge/heuristics-9-blue?style=flat-square" alt="Heuristics"/>
<img src="https://img.shields.io/badge/attack_vectors-7-red?style=flat-square" alt="Vectors"/>
<img src="https://img.shields.io/badge/profiles-5-purple?style=flat-square" alt="Profiles"/>
<img src="https://img.shields.io/badge/tests-21_passing-brightgreen?style=flat-square" alt="Tests"/>
<img src="https://img.shields.io/badge/unsafe_code-forbidden-critical?style=flat-square" alt="Unsafe"/>

<br/><br/>

[🇧🇷 Português](#-português) · [🇺🇸 English](#-english)

<br/>

<!-- Animated separator -->
<img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%"/>

</div>

# 🇧🇷 Português

<div align="center">

> *"Na Roma antiga, antes que as legiões marchassem para território inimigo, a primeira coisa que construíam era um* ***vallum*** *— uma muralha que separava o seguro do desconhecido."*

</div>

---

## 📖 A Muralha que Veio Antes do Ataque

Existe um momento na vida de todo desenvolvedor Web3 que define o antes e o depois. Não é quando ele escreve a primeira linha de código. Não é quando compila pela primeira vez. É quando ele faz deploy — e percebe que **não tem como voltar atrás**.

Na blockchain, não existe Ctrl+Z.

Em 2025, **$905 milhões** foram perdidos em 122 incidentes de smart contracts. Contratos que "funcionavam nos testes". Equipes que confiaram no "parece pronto".

A pergunta não é *se* seu contrato tem vulnerabilidades. É *quais* — e se você vai descobri-las antes ou depois de alguém com más intenções.

**O StellarVallum responde essa pergunta em dois segundos.**

Você abre o terminal. Navega até o diretório do seu projeto — qualquer projeto no ecossistema Stellar. Digita um comando. E a muralha se ergue.

Em menos tempo do que leva para tomar um gole de café, o StellarVallum leu cada arquivo, identificou automaticamente o tipo de projeto, e aplicou nove camadas de verificação alinhadas ao OWASP Smart Contract Top 10 (2026). Encontrou a função sem `require_auth`. A aritmética sem `checked_add`. O storage sem TTL. A API key hardcoded na linha 47.

Mas ele não para na análise estática. O **SPEAR** — o braço ofensivo — pega seu contrato, faz deploy na testnet, e lança sete tipos de ataque real contra ele. Não são simulações. São transações reais, na rede real, com evidência on-chain.

E tudo isso acontece **sem que uma única linha do seu código saia da sua máquina**.

Porque segurança não é um checkbox no final do processo. É a primeira coisa que você constrói. É a muralha que vem antes do acampamento.

<div align="center">

```
stellarvallum scan --path .
```

**É só isso. A muralha se ergue.**

</div>

---

## 💡 O que é?

StellarVallum é um **scanner de segurança** para **qualquer projeto** no ecossistema Stellar + Soroban — smart contracts, dApps, backends, APIs, configs, e pipelines CI/CD.

Ele detecta vulnerabilidades, secrets expostas, e más práticas automaticamente via linha de comando. Gratuito, privado, e instantâneo.

<div align="center">

```
📂 Seu projeto Soroban  →  🔍 stellarvallum scan --path .  →  📊 Relatório de segurança
```

</div>

### ✨ Destaques

| | Feature | Descrição |
|---|---------|-----------|
| 🆓 | **Gratuito** | Modo No-AI funciona sem API keys |
| 🔒 | **Privado** | Código nunca sai da sua máquina |
| ⚡ | **Rápido** | Análise em < 2 segundos |
| 🎯 | **Auto-detect** | Detecta o tipo de projeto automaticamente |
| 🌐 | **Multi-perfil** | Contratos, dApps, backends, configs, pipelines |
| 🛡️ | **OWASP** | Alinhado ao Smart Contract Top 10 (2026) |
| 🔑 | **Secrets** | Detecta chaves e credenciais hardcoded |
| ⚔️ | **Adversarial** | Testa contra 7 vetores de ataque na testnet |

---

## 🏗️ Arquitetura

<div align="center">

```
┌──────────────────────────────────────────────────────────────────┐
│                        STELLARVALLUM v0.2.0                        │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  🛡️ SHIELD ─── Análise Estática Multi-Perfil                     │
│  │  ├── 🔐 Access Control (SC01)                                 │
│  │  ├── 📝 Input Validation (SC05)                               │
│  │  ├── 🔗 Unchecked Calls (SC06)                                │
│  │  ├── 🧮 Arithmetic Errors (SC07)                              │
│  │  ├── 🔄 Reentrancy (SC08)                                    │
│  │  ├── ⬆️  Upgradeability (SC10)                                 │
│  │  ├── ⏰ TTL/Archival Risks                                    │
│  │  ├── 💥 Resource Exhaustion                                   │
│  │  └── 🔑 Secret Detection                                     │
│  │                                                               │
│  ⚔️ SPEAR ─── Testes Adversariais na Testnet Real                │
│  │  └── 7 vetores: val_injection, auth_bypass, storage,         │
│  │      cross_contract, replay, resource, front_running          │
│  │                                                               │
│  ⛓️ CHAIN ─── Trilha de Auditoria Imutável (SHA-256)             │
│  │                                                               │
│  📊 DASHBOARD ─── API REST (Axum)                                │
│  │                                                               │
│  🤖 AI ─── Opcional: No-AI (padrão) | OpenRouter | Ollama       │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

</div>

---

## 📦 Instalação

### Pré-requisitos

- 🦀 [Rust 1.74+](https://rust-lang.org/tools/install)
- 📋 Git

### Instalar

```bash
git clone https://github.com/catitodev/stellarvallum.git
cd stellarvallum
cargo build --release
cargo install --path .
```

### Verificar

```bash
stellarvallum --version
# ⚡ stellarvallum 0.2.0-testnet
```

---

## 🚀 Como Usar

### 🔍 Scan Automático (recomendado)

Entra no diretório do seu projeto e roda:

```bash
stellarvallum scan --path .
```

O StellarVallum **detecta automaticamente** o tipo de projeto e aplica as verificações certas:

| Tipo Detectado | Quando | O que verifica |
|----------------|--------|----------------|
| 🦀 `Contract` | `Cargo.toml` com `soroban-sdk` | OWASP SC Top 10, TTL, resources |
| 🌐 `Dapp` | `package.json` com `react`/`next` + `@stellar/stellar-sdk` | Secrets no frontend, RPC URLs, wallet handling |
| ⚙️ `Backend` | Rust/JS com interação Stellar | HTTP sem TLS, rate limiting, key handling |
| 📄 `Config` | Arquivos `.toml`, `.yaml`, `.env` | Secrets em configs, passphrase errada |
| 🔄 `Pipeline` | `.github/workflows/*.yml` | Secrets em CI, deploy sem approval |

### 📋 Exemplos

```bash
# Scan automático (detecta tipo)
stellarvallum scan --path .

# Forçar perfil específico
stellarvallum scan --path ./src --profile contract

# Scan de arquivo único
stellarvallum scan --path ./src/lib.rs

# Salvar relatório JSON
stellarvallum scan --path . -f report.json

# Salvar relatório CSV
stellarvallum scan --path . --output csv -f report.csv
```

### ⚔️ Testes Adversariais (SPEAR)

Deploy + ataque real na testnet:

```bash
stellarvallum spear \
  --wasm ./target/wasm32-unknown-unknown/release/contrato.wasm \
  --secret-key S...YOUR_TESTNET_SECRET_KEY...
```

<details>
<summary>📋 Exemplo de saída</summary>

```
⚔️ SPEAR ADVERSARIAL TESTING
═══════════════════════════════════════
Contract: CABC123...
Network: testnet
Vectors: 7

  [1] val_injection ........... ✅ Resisted
  [2] auth_bypass ............. ✅ Resisted
  [3] storage_exhaustion ...... ✅ Resisted
  [4] cross_contract .......... ✅ Resisted
  [5] replay_attack ........... ✅ Resisted
  [6] resource_probing ........ ✅ Resisted
  [7] front_running ........... ⚠️  Inconclusive

⚔️ CAMPAIGN COMPLETE
  Vulnerabilities: 0
  Status: ✅ All attacks resisted
```

</details>

### 🚀 Deploy na Testnet

```bash
stellarvallum deploy-testnet \
  --wasm ./target/wasm32-unknown-unknown/release/contrato.wasm \
  --secret-key S...YOUR_TESTNET_SECRET_KEY...
```

### 📊 Dashboard

```bash
stellarvallum dashboard
# → http://127.0.0.1:8501/api/v1/status
```

| Endpoint | Descrição |
|----------|-----------|
| `GET /api/v1/health` | Health check |
| `GET /api/v1/status` | Status do sistema, rede, módulos ativos |

---

## 🛡️ Vulnerabilidades Detectadas

### Smart Contracts (OWASP SC Top 10 2026)

| | ID | Vulnerabilidade | Severidade |
|---|---|---|---|
| 🔐 | SC01 | Funções sem `require_auth` | 🔴 Critical |
| 📝 | SC05 | `from_val` sem `try_from_val`, Vec/Map sem limites | 🟠 High |
| 🔗 | SC06 | `invoke_contract` sem tratamento de retorno | 🟠 High |
| 🧮 | SC07 | Divisão sem zero-check, aritmética sem checked ops | 🟠 High |
| 🔄 | SC08 | State modificado após `invoke_contract` | 🔴 Critical |
| ⬆️ | SC10 | `update_current_contract_wasm` sem auth | 🔴 Critical |
| ⏰ | — | Storage sem `extend_ttl` | 🟠 High |
| 💥 | — | Loops sem limite de iteração | 🟠 High |
| ⚠️ | — | `panic!` ao invés de `panic_with_error!` | 🟠 High |

### Secrets (Todos os perfis)

| | Tipo | Severidade |
|---|---|---|
| 🔑 | Stellar Secret Key (S...) | 🔴 Critical |
| 🔑 | OpenRouter/OpenAI API Key | 🔴 Critical |
| 🔑 | AWS Access Key | 🔴 Critical |
| 🔑 | Private Key (PEM) | 🔴 Critical |
| 🔑 | JWT Token | 🟠 High |
| 🔑 | Hardcoded password/secret | 🟠 High |
| 🔑 | Slack Webhook URL | 🟡 Medium |

### dApps / Backends

| | Verificação | Severidade |
|---|---|---|
| 🌐 | RPC URL hardcoded (sem env var) | 🟡 Medium |
| 🔓 | HTTP sem TLS para endpoints Stellar | 🟠 High |
| 🚫 | Sem rate limiting | 🟡 Medium |
| 💾 | Dados sensíveis em localStorage | 🟠 High |
| ⚙️ | Secret key sem env var | 🟠 High |

---

## 🤖 AI (Opcional)

<table>
<tr><th>Modo</th><th>Custo</th><th>Privacidade</th><th>Velocidade</th></tr>
<tr><td>🆓 <b>No-AI</b> (padrão)</td><td>Grátis</td><td>100%</td><td>Instantâneo</td></tr>
<tr><td>🌐 OpenRouter</td><td>Pay-per-use</td><td>API</td><td>~5s</td></tr>
<tr><td>🏠 Local (Ollama)</td><td>Grátis</td><td>100%</td><td>~10s</td></tr>
</table>

```toml
# config/vallum.toml
[ai]
provider = "none"           # Padrão: gratuito e privado
# provider = "openrouter"   # Opcional: multi-modelo
# provider = "local"        # Opcional: Ollama local
```

---

## 🔐 Segurança do Próprio Projeto

| | Medida | Status |
|---|--------|--------|
| 🦀 | `#![forbid(unsafe_code)]` | ✅ |
| 🔑 | API keys em `Secret<T>` (nunca logadas) | ✅ |
| 🔒 | HTTPS/TLS 1.2+ obrigatório | ✅ |
| ✅ | Validação StrKey (G.../C.../S...) | ✅ |
| 🧹 | CSV sanitizado contra injection | ✅ |
| 🛤️ | Paths validados contra traversal | ✅ |
| 🌐 | Passphrase validada contra RPC real | ✅ |
| 🧪 | 21 testes automatizados | ✅ |

---

## 💰 Obter XLM de Teste

```bash
# 1. Criar keypair: https://lab.stellar.org/
# 2. Fundar (grátis):
curl "https://friendbot.stellar.org?addr=G_SEU_ENDERECO"
```

> 💡 O StellarVallum faz isso automaticamente quando necessário.

---

## 🗺️ Roadmap

| Fase | Versão | Status | Foco |
|------|--------|--------|------|
| 🚧 Beta | v0.2.0 | ✅ Atual | Testnet, multi-perfil, secrets |
| 👥 Comunidade | v0.3.0 | ⏳ | Bug bounty, mais heurísticas |
| 🔍 Auditoria | v0.4.0 | ⏳ | Audit externo, formal verification |
| 🌐 Mainnet | v1.0.0 | ⏳ | Mainnet unlock, enterprise |

---

## 🤝 Contribuindo

```bash
git clone https://github.com/catitodev/stellarvallum.git
cd stellarvallum
cargo build && cargo test
```

**Áreas prioritárias:**
- 🧩 Novas heurísticas de detecção
- 🔗 Integração XDR completa para deploy real
- 🎨 Frontend para o dashboard
- 🧪 Testes com contratos reais da comunidade
- 🌍 Traduções

---

## 📄 Licença

Apache License 2.0 — veja [LICENSE](LICENSE)

---

## 🔗 Links

| | Recurso | URL |
|---|---------|-----|
| 📦 | Repositório | [github.com/catitodev/stellarvallum](https://github.com/catitodev/stellarvallum) |
| 🏛️ | Vallum Original | [github.com/catitodev/vallum](https://github.com/catitodev/vallum) |
| ⭐ | Stellar Developers | [developers.stellar.org](https://developers.stellar.org) |
| 📚 | Soroban Docs | [developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts/overview) |
| 🛡️ | OWASP SC Top 10 | [scs.owasp.org/sctop10](https://scs.owasp.org/sctop10/) |

---

<div align="center">
<img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%"/>
</div>

---

<div align="center">

# 🇺🇸 English

</div>

<div align="center">

> *"In ancient Rome, before legions marched into enemy territory, the first thing they built was a* ***vallum*** *— a wall that separated the safe from the unknown."*

</div>

---

## 📖 The Wall That Came Before the Attack

There's a moment in every Web3 developer's life that defines the before and after. It's not when they write the first line of code. It's not when they compile for the first time. It's when they deploy — and realize **there's no going back**.

On the blockchain, there is no Ctrl+Z.

In 2025, **$905 million** was lost across 122 smart contract incidents. Contracts that "worked in tests". Teams that trusted "looks ready".

The question isn't *whether* your contract has vulnerabilities. It's *which ones* — and whether you'll find them before or after someone with bad intentions does.

**StellarVallum answers that question in two seconds.**

You open the terminal. Navigate to your project directory — any project in the Stellar ecosystem. Type one command. And the wall rises.

In less time than it takes to sip your coffee, StellarVallum has read every file, auto-detected the project type, and applied nine layers of verification aligned with the OWASP Smart Contract Top 10 (2026). It found the function without `require_auth`. The arithmetic without `checked_add`. The storage without TTL. The API key hardcoded on line 47.

But it doesn't stop at static analysis. **SPEAR** — the offensive arm — takes your contract, deploys it to testnet, and launches seven types of real attacks against it. Not simulations. Real transactions, on the real network, with on-chain evidence.

And all of this happens **without a single line of your code leaving your machine**.

Because security isn't a checkbox at the end of the process. It's the first thing you build. It's the wall that comes before the camp.

<div align="center">

```
stellarvallum scan --path .
```

**That's it. The wall rises.**

</div>

---

## 💡 What is it?

StellarVallum is a **security scanner** for **any project** in the Stellar + Soroban ecosystem — smart contracts, dApps, backends, APIs, configs, and CI/CD pipelines.

It detects vulnerabilities, exposed secrets, and bad practices automatically via command line. Free, private, and instant.

<div align="center">

```
📂 Your Soroban project  →  🔍 stellarvallum scan --path .  →  📊 Security report
```

</div>

### ✨ Highlights

| | Feature | Description |
|---|---------|-------------|
| 🆓 | **Free** | No-AI mode works without API keys |
| 🔒 | **Private** | Code never leaves your machine |
| ⚡ | **Fast** | Analysis in < 2 seconds |
| 🎯 | **Auto-detect** | Detects project type automatically |
| 🌐 | **Multi-profile** | Contracts, dApps, backends, configs, pipelines |
| 🛡️ | **OWASP** | Aligned with Smart Contract Top 10 (2026) |
| 🔑 | **Secrets** | Detects hardcoded keys and credentials |
| ⚔️ | **Adversarial** | Tests against 7 attack vectors on testnet |

---

## 📦 Installation

### Prerequisites

- 🦀 [Rust 1.74+](https://rust-lang.org/tools/install)
- 📋 Git

### Install

```bash
git clone https://github.com/catitodev/stellarvallum.git
cd stellarvallum
cargo build --release
cargo install --path .
```

### Verify

```bash
stellarvallum --version
# ⚡ stellarvallum 0.2.0-testnet
```

---

## 🚀 Usage

### 🔍 Auto Scan (recommended)

Navigate to your project directory and run:

```bash
stellarvallum scan --path .
```

StellarVallum **auto-detects** the project type and applies the right checks:

| Detected Type | When | What it checks |
|---------------|------|----------------|
| 🦀 `Contract` | `Cargo.toml` with `soroban-sdk` | OWASP SC Top 10, TTL, resources |
| 🌐 `Dapp` | `package.json` with `react`/`next` + `@stellar/stellar-sdk` | Frontend secrets, RPC URLs, wallet handling |
| ⚙️ `Backend` | Rust/JS with Stellar interaction | HTTP without TLS, rate limiting, key handling |
| 📄 `Config` | `.toml`, `.yaml`, `.env` files | Secrets in configs, wrong passphrase |
| 🔄 `Pipeline` | `.github/workflows/*.yml` | Secrets in CI, deploy without approval |

### 📋 Examples

```bash
# Auto scan (detects type)
stellarvallum scan --path .

# Force specific profile
stellarvallum scan --path ./src --profile contract

# Single file scan
stellarvallum scan --path ./src/lib.rs

# Save JSON report
stellarvallum scan --path . -f report.json

# Save CSV report
stellarvallum scan --path . --output csv -f report.csv
```

### ⚔️ Adversarial Testing (SPEAR)

Deploy + real attack on testnet:

```bash
stellarvallum spear \
  --wasm ./target/wasm32-unknown-unknown/release/contract.wasm \
  --secret-key S...YOUR_TESTNET_SECRET_KEY...
```

<details>
<summary>📋 Example output</summary>

```
⚔️ SPEAR ADVERSARIAL TESTING
═══════════════════════════════════════
Contract: CABC123...
Network: testnet
Vectors: 7

  [1] val_injection ........... ✅ Resisted
  [2] auth_bypass ............. ✅ Resisted
  [3] storage_exhaustion ...... ✅ Resisted
  [4] cross_contract .......... ✅ Resisted
  [5] replay_attack ........... ✅ Resisted
  [6] resource_probing ........ ✅ Resisted
  [7] front_running ........... ⚠️  Inconclusive

⚔️ CAMPAIGN COMPLETE
  Vulnerabilities: 0
  Status: ✅ All attacks resisted
```

</details>

### 🚀 Deploy to Testnet

```bash
stellarvallum deploy-testnet \
  --wasm ./target/wasm32-unknown-unknown/release/contract.wasm \
  --secret-key S...YOUR_TESTNET_SECRET_KEY...
```

### 📊 Dashboard

```bash
stellarvallum dashboard
# → http://127.0.0.1:8501/api/v1/status
```

| Endpoint | Description |
|----------|-------------|
| `GET /api/v1/health` | Health check |
| `GET /api/v1/status` | System status, network, active modules |

---

## 🛡️ Detected Vulnerabilities

### Smart Contracts (OWASP SC Top 10 2026)

| | ID | Vulnerability | Severity |
|---|---|---|---|
| 🔐 | SC01 | Functions without `require_auth` | 🔴 Critical |
| 📝 | SC05 | `from_val` without `try_from_val`, unbounded Vec/Map | 🟠 High |
| 🔗 | SC06 | `invoke_contract` without return handling | 🟠 High |
| 🧮 | SC07 | Division without zero-check, unchecked arithmetic | 🟠 High |
| 🔄 | SC08 | State modified after `invoke_contract` | 🔴 Critical |
| ⬆️ | SC10 | `update_current_contract_wasm` without auth | 🔴 Critical |
| ⏰ | — | Storage without `extend_ttl` | 🟠 High |
| 💥 | — | Loops without iteration limit | 🟠 High |
| ⚠️ | — | `panic!` instead of `panic_with_error!` | 🟠 High |

### Secrets (All profiles)

| | Type | Severity |
|---|---|---|
| 🔑 | Stellar Secret Key (S...) | 🔴 Critical |
| 🔑 | OpenRouter/OpenAI API Key | 🔴 Critical |
| 🔑 | AWS Access Key | 🔴 Critical |
| 🔑 | Private Key (PEM) | 🔴 Critical |
| 🔑 | JWT Token | 🟠 High |
| 🔑 | Hardcoded password/secret | 🟠 High |
| 🔑 | Slack Webhook URL | 🟡 Medium |

### dApps / Backends

| | Check | Severity |
|---|---|---|
| 🌐 | Hardcoded RPC URL (no env var) | 🟡 Medium |
| 🔓 | HTTP without TLS for Stellar endpoints | 🟠 High |
| 🚫 | No rate limiting | 🟡 Medium |
| 💾 | Sensitive data in localStorage | 🟠 High |
| ⚙️ | Secret key without env var | 🟠 High |

---

## 🤖 AI (Optional)

<table>
<tr><th>Mode</th><th>Cost</th><th>Privacy</th><th>Speed</th></tr>
<tr><td>🆓 <b>No-AI</b> (default)</td><td>Free</td><td>100%</td><td>Instant</td></tr>
<tr><td>🌐 OpenRouter</td><td>Pay-per-use</td><td>API</td><td>~5s</td></tr>
<tr><td>🏠 Local (Ollama)</td><td>Free</td><td>100%</td><td>~10s</td></tr>
</table>

```toml
# config/vallum.toml
[ai]
provider = "none"           # Default: free and private
# provider = "openrouter"   # Optional: multi-model
# provider = "local"        # Optional: local Ollama
```

---

## 🔐 Project Security

| | Measure | Status |
|---|---------|--------|
| 🦀 | `#![forbid(unsafe_code)]` | ✅ |
| 🔑 | API keys in `Secret<T>` (never logged) | ✅ |
| 🔒 | HTTPS/TLS 1.2+ enforced | ✅ |
| ✅ | StrKey validation (G.../C.../S...) | ✅ |
| 🧹 | CSV sanitized against injection | ✅ |
| 🛤️ | Paths validated against traversal | ✅ |
| 🌐 | Passphrase validated against real RPC | ✅ |
| 🧪 | 21 automated tests | ✅ |

---

## 💰 Getting Test XLM

```bash
# 1. Create keypair: https://lab.stellar.org/
# 2. Fund (free):
curl "https://friendbot.stellar.org?addr=G_YOUR_ADDRESS"
```

> 💡 StellarVallum does this automatically when needed.

---

## 🗺️ Roadmap

| Phase | Version | Status | Focus |
|-------|---------|--------|-------|
| 🚧 Beta | v0.2.0 | ✅ Current | Testnet, multi-profile, secrets |
| 👥 Community | v0.3.0 | ⏳ | Bug bounty, more heuristics |
| 🔍 Audit | v0.4.0 | ⏳ | External audit, formal verification |
| 🌐 Mainnet | v1.0.0 | ⏳ | Mainnet unlock, enterprise |

---

## 🤝 Contributing

```bash
git clone https://github.com/catitodev/stellarvallum.git
cd stellarvallum
cargo build && cargo test
```

**Priority areas:**
- 🧩 New detection heuristics
- 🔗 Full XDR integration for real deployment
- 🎨 Dashboard frontend
- 🧪 Testing with real community contracts
- 🌍 Translations

---

## 📄 License

Apache License 2.0 — see [LICENSE](LICENSE)

---

## 🔗 Links

| | Resource | URL |
|---|----------|-----|
| 📦 | Repository | [github.com/catitodev/stellarvallum](https://github.com/catitodev/stellarvallum) |
| 🏛️ | Original Vallum | [github.com/catitodev/vallum](https://github.com/catitodev/vallum) |
| ⭐ | Stellar Developers | [developers.stellar.org](https://developers.stellar.org) |
| 📚 | Soroban Docs | [developers.stellar.org/docs/build/smart-contracts](https://developers.stellar.org/docs/build/smart-contracts/overview) |
| 🛡️ | OWASP SC Top 10 | [scs.owasp.org/sctop10](https://scs.owasp.org/sctop10/) |

---

<div align="center">

<br/>

<img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%"/>

<br/>

**⚡ TESTNET ONLY — Mainnet support coming in v1.0**

<br/>

<img src="assets/image/stelar_vallum_logo.png" alt="StellarVallum" width="60"/>

<sub>Built with 🦀 Rust · Secured by 🛡️ StellarVallum · Powered by ⭐ Stellar</sub>

<br/><br/>

<a href="https://github.com/catitodev/stellarvallum/stargazers">
  <img src="https://img.shields.io/github/stars/catitodev/stellarvallum?style=social" alt="Stars"/>
</a>
<a href="https://github.com/catitodev/stellarvallum/network/members">
  <img src="https://img.shields.io/github/forks/catitodev/stellarvallum?style=social" alt="Forks"/>
</a>
<a href="https://github.com/catitodev/stellarvallum/issues">
  <img src="https://img.shields.io/github/issues/catitodev/stellarvallum?style=social" alt="Issues"/>
</a>

<br/><br/>

<a href="https://github.com/catitodev/stellarvallum">
  <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&size=14&duration=4000&pause=2000&color=666666&center=true&vCenter=true&repeat=true&width=500&height=20&lines=Segurança+não+é+um+produto%2C+é+um+processo.;Security+is+not+a+product%2C+it's+a+process.;Validamos+na+testnet+para+construir+confiança+na+mainnet.;We+validate+on+testnet+to+build+confidence+for+mainnet.;A+muralha+se+ergue+antes+do+ataque.;The+wall+rises+before+the+attack." alt="Quote"/>
</a>

</div>

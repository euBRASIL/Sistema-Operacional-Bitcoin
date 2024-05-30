
<img src="images/bitcoinos.png" width="66%">

#

[![Apache-2 licensed](https://img.shields.io/crates/l/rgb-wallet)](./LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

## O que é o `BITcoinOS`

bitcoinOS é um projeto inovador que combina Bitcoin com IA. Ele serve como um sistema de emissão e gerenciamento de ativos Bitcoin Layer2, incorporando a segurança do Bitcoin, os recursos de privacidade e contratos inteligentes do RGB e a escalabilidade linear da rede ICP. Além disso, fornece soberania de dados individuais e ativos de dados, apoiando plataformas de inovação de IA com capacidade de utilizar grandes modelos de linguagem GPT.

Bitcoin é um sistema de dinheiro eletrônico peer-to-peer projetado para criar um ecossistema financeiro descentralizado, proporcionando aos indivíduos soberania sobre sua identidade e ativos digitais.

O bitcoinOS, por outro lado, toma o Bitcoin como seu núcleo e adiciona proteção de soberania para ativos de dados. bitcoinOS é uma rede blockchain modular que usa Bitcoin L1 como camada de liquidação e constrói Bitcoin L2: Fastnet com contratos inteligentes RGB e tecnologia de rede ICP em seu núcleo.

Embora a rede Bitcoin se concentre principalmente em plataformas de ativos e aplicações de pagamento, o bitcoinOS expande o escopo do ecossistema Bitcoin e melhora a experiência do usuário integrando totalmente os ativos Bitcoin L1 e combinando-os com a rede ICP através da SmartWallet.

O bitcoinOS mantém a identidade e a soberania dos ativos do Bitcoin, ao mesmo tempo que aumenta o suporte e a proteção à soberania dos dados, melhorando a privacidade e a escalabilidade, facilitando assim pagamentos e transações rápidas para todos os ativos do Bitcoin.

Na era da inteligência artificial, os dados são considerados o meio de produção e fator mais importante. Através da soberania dos dados fornecida pelo bitcoinOS, os proprietários legítimos dos dados recebem o primeiro passo de proteção.

Semelhante ao empoderamento de identidade e moeda do Bitcoin, o empoderamento de dados do bitcoinOS pode ativar a inovação para todos, utilizando dados massivos com Large Language Models (LLMs) ou Large World Models (LWMs). Isto não só promove a rápida integração da Web2 e Web3, mas também impulsiona a ampla adoção da tecnologia Bitcoin e blockchain.

[White paper](./whitepaper/bitcoinOS-whitepaper.pdf)

## Vision
- Bitcoin Assets Hub: bitcoinOS serves as a comprehensive platform for managing Bitcoin native assets, including but not limited to BTC transfers, as well as issuing and trading Ordinals, Atomicals, and RGB assets. As the Bitcoin L2 layer within bitcoinOS, Fastnet facilitates the issuance and trading of RGB assets, providing financial support and fast payments for dApp and ecosystems. bitcoinOS is an excellent hub and funding pool for various Bitcoin assets, catering to a wide range of asset needs.
  
- One-stop Platform: While Bitcoin L1 has successfully established sovereignty over identity and assets, bitcoinOS takes this concept to new heights. Built upon Bitcoin L1 as its core, bitcoinOS utilizes the RGB smart contract protocol on the ICP network to construct Bitcoin L2: Fastnet. Fastnet offers comprehensive Bitcoin smart contract functionality and unlimited scalability, while also strengthening support for data sovereignty, elevating data assets to first-class status. Thanks to the excellent privacy features of RGB and ICP, bitcoinOS not only enables the development of better Web3 applications but also facilitates the development of superior Web2 applications, making it a unified platform for various applications.

- Gateway to AGI: Data assets are considered among the most important digital assets. Just as Bitcoin is digital gold, data is digital oil, and bitcoinOS serves as the catalyst for the digital oil revolution. As the first Bitcoin network to grant data ownership, bitcoinOS is positioned as the gateway to the era of artificial intelligence. In the age of artificial intelligence, data becomes the most crucial factor of production, and possessing bitcoinOS and data assets becomes the ticket for every individual to enter the era of artificial intelligence. You may not have your own large-scale model, but you must have your own unique intelligent entity.

- Foundation for Bitcoin Mass Adoption: As the cornerstone driving the widespread adoption of Bitcoin, bitcoinOS combines the transparency of Bitcoin assets with the versatility of the Fastnet universal network. Fastnet not only facilitates fast payments but also ensures the privacy of content and services, while almost unlimited scalability meets diverse application needs. bitcoinOS positions Bitcoin assets as the universal token of the digital world in the era of artificial intelligence.

- Copilot of Developers & Creators: The runtime environment of bitcoinOS is based on WebAssembly, standing at the forefront of the IT industry's standards and benefiting from the research achievements of the entire industry. The core functionality of bitcoinOS smart contract containers simplifies the complexity of developing smart contracts for developers. They can utilize various programming languages that support WebAssembly, such as Golang, JavaScript, Python, and Rust, for development. Additionally, the built-in dApp Store, Data Store, and GPT Store in bitcoinOS greatly facilitate developers and users in accessing various resources of the Bitcoin ecosystem and innovating using data and AI models.

## Architecture
![avatar](./images/bitcoinOS-arch.png)

## Setup
To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with bitcoinOS, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)
- [IC Bitcoin Integration](https://internetcomputer.org/how-it-works/bitcoin-integration/)
- [Bitcoin Integration Wiki](https://wiki.internetcomputer.org/wiki/Bitcoin_Integration)
- [Basic Bitcoin example](https://internetcomputer.org/docs/current/tutorials/developer-journey/level-4/4.3-ckbtc-and-bitcoin/)
- [Bitcoin Canister Source Code](https://github.com/dfinity/bitcoin-canister)

If you want to start working on your project right away, you might want to try the following commands:

```bash
git clone https://github.com/bitcoinOS/bitcoinOS.git
cd bitcoinOS/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Clone the `wasm-forge/wasi2ic` project:
git clone https://github.com/wasm-forge/wasi2ic

## Enter the wasi2ic directory and install it
cd wasi2ic
cargo install --path .

# Install just
cargo install just

# Starts the replica, running in the background
# Add --clean if you want to reset the dfx state: dfx start --background --clean
dfx start --background   --enable-bitcoin


# All the just command details are in `justfile`
# Create all canisters
just create_all_canisters

# Compile smartwallet to wasm
just build_wallet

# Compile stakingpool to wasm
just build_staking

# Deploy os canister
just deploy_os

# Mint cycles on local network if you need
wallet=$(dfx identity get-wallet)
dfx ledger fabricate-cycles --t 2000 --canister $wallet
dfx wallet balance

# Deposit cycles to os canister if os canister cycles are not enought
dfx canister deposit-cycles 2000000000000 os

# Create staking pool canister by os canister
dfx canister call os create_staking_pool_canister '(record { duration_in_millisecond = 86; name = "staking pool test"; description = "a staking pool with 10 annual interest rate for a year"; annual_interest_rate = 10 })'

# If you want to deploy smartwall for test by manually, In normal, the smartwallet will be install by os canister
# just deploy_wallet

just deploy_ii
  
# frontend
dfx generate
npm i
npm run start
# deploy frontend
npm run build
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.


### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`DFX_NETWORK` to `ic` if you are using Webpack
- use your own preferred method to replace `process.env.DFX_NETWORK` in the autogenerated declarations
  - Setting `canisters -> {asset_canister_id} -> declarations -> env_override to a string` in `dfx.json` will replace `process.env.DFX_NETWORK` with the string in the autogenerated declarations
- Write your own `createActor` constructor

## Contribute
Contributions welcome! Read the [contribution guidelines](CONTRIBUTING.md) first.

## License

bitcoinOS is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

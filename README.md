# Agentropic

[![Crates.io](https://img.shields.io/crates/v/agentropic.svg)](https://crates.io/crates/agentropic)
[![Documentation](https://docs.rs/agentropic/badge.svg)](https://docs.rs/agentropic)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Agent-Oriented Programming in Rust - Batteries Included**

Agentropic is a comprehensive framework for building autonomous, intelligent multi-agent systems in Rust. This is the main "batteries-included" crate that re-exports all components of the Agentropic ecosystem, providing everything you need to build production-ready agentic applications.

> ⚠️ **Status: Active Development** - Agentropic is currently under active development. APIs may change before the 1.0 release.

---

## 🎯 What is Agentropic?

Agentropic enables you to build software systems composed of **autonomous agents** that:

- 🤖 **Think** - Use BDI (Belief-Desire-Intention) cognitive architecture for reasoning and planning
- 💬 **Communicate** - Exchange messages using standardized Agent Communication Language (ACL)
- 🤝 **Coordinate** - Work together using proven multi-agent patterns (hierarchy, swarm, market-based)
- ⚡ **Execute** - Run efficiently with isolation, scheduling, and fault tolerance
- 🚀 **Deploy** - Ship to production with comprehensive tooling and orchestration

---

## 📊 Project Progress

| Crate | Status | Description | Repository |
|-------|--------|-------------|------------|
| **agentropic-core** | ✅ Complete | Agent primitives, traits, lifecycle | [Link](https://github.com/agentropic/agentropic-core) |
| **agentropic-messaging** | ✅ Complete | Communication protocols, routing, ACL | [Link](https://github.com/agentropic/agentropic-messaging) |
| **agentropic-cognition** | 🔄 In Progress | Reasoning, planning, BDI, decision-making | [Link](https://github.com/agentropic/agentropic-cognition) |
| **agentropic-patterns** | ⏳ Planned | Multi-agent patterns (hierarchy, swarm, market) | [Link](https://github.com/agentropic/agentropic-patterns) |
| **agentropic-runtime** | ⏳ Planned | Execution engine, scheduler, isolation | [Link](https://github.com/agentropic/agentropic-runtime) |
| **agentropic-deploy** | ⏳ Planned | Deployment tools, orchestration, CLI | [Link](https://github.com/agentropic/agentropic-deploy) |
| **agentropic-tools** | ⏳ Planned | Testing frameworks, benchmarks, utilities | [Link](https://github.com/agentropic/agentropic-tools) |
| **agentropic-examples** | ⏳ Planned | Example applications and tutorials | [Link](https://github.com/agentropic/agentropic-examples) |
| **agentropic-docs** | ⏳ Planned | Documentation site | [Link](https://github.com/agentropic/agentropic-docs) |

**Overall Progress:** 20% Complete 🚀

---

## ✨ Features

### Completed ✅

- 🤖 **Agent Primitives** - Clean abstractions for autonomous agents
- 🆔 **Identity System** - UUID-based agent identification
- 💬 **Message Passing** - FIPA-inspired communication protocols
- 📮 **Mailboxes & Routing** - Efficient message delivery
- 🎭 **Performatives** - Speech acts (Inform, Request, Query, etc.)

### In Development 🔄

- 🧠 **BDI Architecture** - Belief-Desire-Intention cognitive model
- 📊 **Planning** - Goal-oriented action planning (STRIPS, HTN)
- 🤔 **Reasoning** - Logical inference and rule-based reasoning
- 🎯 **Decision-Making** - Utility-based and probabilistic decisions

### Coming Soon ⏳

- 🏗️ **Organizational Patterns** - Hierarchy, teams, swarms, markets
- ⚡ **High-Performance Runtime** - Async execution with scheduling
- 🛡️ **Isolation** - Resource limits and sandboxing
- 🚀 **Production Deployment** - CLI tools, orchestration, monitoring
- 🧪 **Testing & Tools** - Comprehensive testing and benchmarking

---

## 🚀 Quick Start

> **Note:** Currently, you need to use individual crates. The unified `agentropic` facade will be available once more components are complete.

### Using Individual Crates (Current)
```toml
[dependencies]
agentropic-core = "0.1.0"
agentropic-messaging = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```
```rust
use agentropic_core::prelude::*;
use agentropic_messaging::prelude::*;

struct MyAgent {
    id: AgentId,
}

#[async_trait]
impl Agent for MyAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent initialized");
        Ok(())
    }

    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent executing");
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info("Agent shutting down");
        Ok(())
    }
}
```

### Future (Unified Facade - Coming Soon)
```toml
[dependencies]
agentropic = "0.1.0"
```
```rust
use agentropic::prelude::*;

// Everything available from one crate!
```

---

## 🏗️ Architecture

Agentropic is organized into modular crates for flexibility and maintainability:
```
┌─────────────────────────────────────────────────────┐
│              Agentropic (Facade)                    │
│         "Batteries-included entry point"            │
└─────────────────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Examples   │  │     Docs     │  │    Tools     │
│   Tutorials  │  │ Documentation│  │Testing/Bench │
└──────────────┘  └──────────────┘  └──────────────┘
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│    Deploy    │  │   Patterns   │  │   Runtime    │
│ Orchestration│  │Multi-Agent   │  │  Execution   │
└──────────────┘  └──────────────┘  └──────────────┘
        │                │                │
        └────────────────┼────────────────┘
                         ↓
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  Cognition   │  │  Messaging   │  │     Core     │
│ BDI/Planning │  │ ACL/Routing  │  │   Agents     │
└──────────────┘  └──────────────┘  └──────────────┘
```

---

## 🎓 Why Agentropic?

### Rust-First Design

Unlike Python-based agent frameworks, Agentropic leverages Rust's:
- 🚀 **Performance** - Native speed, zero-cost abstractions
- 🛡️ **Safety** - Memory-safe, thread-safe by design
- 🔄 **Concurrency** - Fearless async/await
- 📦 **Tooling** - Cargo, clippy, and the Rust ecosystem

### Production-Ready from Day One

- ⚡ Built-in runtime with scheduling and isolation
- 📊 Comprehensive testing and benchmarking tools
- 🚀 Deployment tooling and orchestration
- 📈 Monitoring and observability

### Academic Foundations, Practical Implementation

Based on decades of multi-agent systems research:
- BDI Architecture (Rao & Georgeff)
- FIPA Agent Communication Language
- Contract Net Protocol
- Market-Based Control
- Swarm Intelligence

---

## 🎯 Use Cases

Agentropic is ideal for:

### ⛓️ Blockchain Network
- Agent-oriented programming (AOP) for smart contracts
- Agentic dApp development and deployment
- Coordination and automation across blockchain nodes

### 💰 Financial Systems
- Algorithmic trading with risk management
- Portfolio management and optimization
- Market making and liquidity provision

### 🤖 Robotics & IoT
- Swarm robotics coordination
- Distributed sensor networks
- Autonomous vehicle fleets

### 🏢 Enterprise Applications
- Workflow automation
- Supply chain coordination
- Multi-party business processes

### 🎮 Gaming & Simulation
- Intelligent NPCs
- Strategy game AI
- Complex system modeling

### 🏠 Smart Systems
- Home automation
- Energy grid optimization
- Resource allocation

---

## 📖 Documentation

- **[GitHub Organization](https://github.com/agentropic)** - All repositories
- **[Getting Started Guide](https://github.com/agentropic/agentropic-examples)** - Coming soon
- **[API Documentation](https://docs.rs/agentropic)** - Coming soon
- **[Architecture Guide](https://github.com/agentropic/agentropic-docs)** - Coming soon

---

## 🤝 Contributing

We welcome contributions! Agentropic is under active development and we'd love your help.

### Ways to Contribute

- 🐛 **Report bugs** - Open issues for bugs you find
- 💡 **Suggest features** - Share ideas for improvements
- 📖 **Improve documentation** - Help others understand the project
- 💻 **Submit code** - Contribute to any of the crates
- 🧪 **Add tests** - Improve test coverage
- 📢 **Spread the word** - Star the repo, share with others

### Development Setup
```bash
# Clone all repositories
git clone https://github.com/agentropic/agentropic-core.git
git clone https://github.com/agentropic/agentropic-messaging.git
# ... etc

# Build and test
cd agentropic-core
cargo build
cargo test
```

### Contribution Guidelines

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

---

## 🗺️ Roadmap

### Phase 1: Foundation 
- ✅ Core agent primitives and traits
- ✅ Message passing and communication
- 🔄 BDI cognitive architecture
- 🔄 Basic planning and reasoning

### Phase 2: Intelligence 
- ⏳ Advanced planning (HTN, PDDL)
- ⏳ Decision-making frameworks
- ⏳ Learning and adaptation

### Phase 3: Coordination 
- ⏳ Multi-agent patterns
- ⏳ Organizational structures
- ⏳ Market-based coordination

### Phase 4: Production 
- ⏳ High-performance runtime
- ⏳ Deployment tools and CLI
- ⏳ Monitoring and observability

### Phase 5: Ecosystem 
- ⏳ Comprehensive examples
- ⏳ Full documentation site
- ⏳ Testing and benchmarking suite

### Version 1.0 
- ⏳ Stable API
- ⏳ Production-hardened
- ⏳ Enterprise-ready

---

## 📊 Performance

Agentropic is built for performance:

- **Agent spawn latency**: < 1ms (target)
- **Message passing**: < 10μs latency (target)
- **Throughput**: 100,000+ messages/second (target)
- **Memory**: ~50KB per agent baseline (target)

*Benchmarks coming soon as crates are completed*

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## 🙏 Acknowledgments

Agentropic is inspired by decades of research in multi-agent systems:

- **FIPA Standards** - Agent communication protocols
- **BDI Architecture** - Rao & Georgeff
- **Swarm Intelligence** - Kennedy & Eberhart  
- **Market-Based Control** - Clearwater
- **Contract Net Protocol** - Smith
- **Erlang/OTP** - Fault tolerance patterns

---

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=agentropic/agentropic&type=Date)](https://star-history.com/#agentropic/agentropic&Date)

---

## 📬 Contact

- **GitHub**: [@agentropic](https://github.com/agentropic)
- **Issues**: [GitHub Issues](https://github.com/agentropic/agentropic/issues)
- **Discussions**: [GitHub Discussions](https://github.com/agentropic/agentropic/discussions)

---

<div align="center">

**Build intelligent, autonomous agents in Rust with Agentropic**

[View Organization](https://github.com/agentropic) • [Documentation](https://github.com/agentropic/agentropic-docs) • [Examples](https://github.com/agentropic/agentropic-examples)

**Made with 🦀 by the Agentropic team**

</div>
# Relay: AI Collaboration That Respects Your Privacy

> Relay seamlessly connects human and AI intelligence while keeping your private work private. Not another chatbot - a true collaboration framework.

## 🚀 What is Relay?

Relay is a privacy-first AI collaboration framework that creates a shared workspace where human and AI agents work together by choice, not by default. It's the practical implementation of the "foyer commun" vision - a meeting place where human and AI intelligence combine while preserving what makes us human: privacy, agency, and choice.

## Key Features

- **Privacy Zones**: Cryptographically sealed private areas that AI cannot access
- **Async Architecture**: Zero performance impact on normal operations
- **Human-Centric**: You're the pilot, AI agents are co-pilots
- **Capability Security**: Fine-grained permissions for each agent
- **Transparent Audit**: Every agent action is logged and reviewable

## Quick Start

```bash
# Install Relay
$ cargo install relay

# Initialize in your project
$ relay init

# Start the daemon
$ relay start

# Invite an AI agent
$ relay invite claude

# Create a private zone
$ relay protect ~/Ecosystem/Private/
```

## Architecture Overview

Relay consists of:
- **relay-core**: Rust daemon for performance and security
- **relay-cli**: Command-line interface
- **relay-agents**: Agent implementations (Claude, Copilot, etc.)
- **relay-sdk**: SDK for custom agent development

## Philosophy

Relay implements the "foyer commun" vision through practical technology:
1. Humans maintain primacy and control
2. Privacy by default, sharing by explicit choice
3. Enhancement not replacement
4. Transparent but not invasive
5. Evolution that serves human goals

## Documentation

- [Installation Guide](docs/installation.md)
- [Configuration](docs/configuration.md)
- [Privacy Model](docs/privacy.md)
- [Agent Development](docs/agents.md)
- [API Reference](docs/api.md)

## Contributing

Relay is part of the [Agentic Ecosystem Lab](https://github.com/omar-el-mountassir/agentic-ecosystem-lab) project. We welcome contributions that align with our human-centric, privacy-first philosophy.

## License

MIT - See [LICENSE](LICENSE) for details.

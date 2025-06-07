# Relay Installation Guide

## Prerequisites

- Rust 1.75 or later
- PostgreSQL 14+ (for agent memory)
- Git

## Installation Methods

### From Source (Recommended during development)

```bash
# Clone the repository
git clone https://github.com/omar-el-mountassir/agentic-ecosystem-lab
cd agentic-ecosystem-lab/relay

# Build and install
cargo install --path relay-cli
```

### Using Cargo (Coming soon)

```bash
cargo install relay
```

### Platform-Specific

#### Pop!_OS / Ubuntu

```bash
# Install dependencies
sudo apt update
sudo apt install build-essential pkg-config libssl-dev postgresql

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Relay
cargo install relay
```

#### Windows (WSL2 recommended)

```powershell
# Install WSL2 first, then follow Linux instructions
wsl --install
```

## Initial Setup

1. **Initialize Relay in your workspace**
   ```bash
   cd ~/your-workspace
   relay init
   ```

2. **Configure privacy zones**
   ```bash
   relay protect ~/Private
   relay protect ~/Documents/Personal
   ```

3. **Start the daemon**
   ```bash
   relay start
   ```

4. **Verify installation**
   ```bash
   relay status
   ```

## Next Steps

- [Configuration Guide](configuration.md)
- [Privacy Model](privacy.md)
- [Agent Setup](agents.md)

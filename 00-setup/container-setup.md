# Container Setup with Podman

This guide covers setting up a containerized Rust development environment using Podman, providing consistency across your M4 MacBook Pro and Windows 11 desktop.

## 🐳 Why Containers for Rust Development?

- **Consistency**: Same environment across all platforms
- **Isolation**: No system pollution with development dependencies
- **Reproducibility**: Team members get identical setups
- **Experimentation**: Try different Rust versions without affecting system

## 📦 Podman vs Docker

We're using Podman because:
- No daemon required (better security)
- Rootless containers by default
- Docker-compatible CLI
- Better integration with systemd (Linux)
- Native on Red Hat systems

## 🚀 Quick Start

### 1. Create Development Container

Create `containers/Dockerfile.dev`:

```dockerfile
# Multi-stage Dockerfile for Rust development
FROM rust:1.75 AS base

# Install development dependencies
RUN apt-get update && apt-get install -y \
    # Build essentials
    build-essential \
    pkg-config \
    libssl-dev \
    # Useful tools
    git \
    vim \
    tmux \
    htop \
    curl \
    # Performance tools
    linux-tools-generic \
    valgrind \
    heaptrack \
    # Clean up
    && rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN cargo install \
    cargo-watch \
    cargo-edit \
    cargo-expand \
    cargo-criterion \
    flamegraph \
    cargo-bloat \
    cargo-audit \
    cargo-outdated

# Create non-root user
ARG USERNAME=rustdev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Switch to non-root user
USER $USERNAME
WORKDIR /home/$USERNAME

# Set up Rust environment
ENV CARGO_HOME=/home/$USERNAME/.cargo
ENV RUSTUP_HOME=/home/$USERNAME/.rustup
ENV PATH=$CARGO_HOME/bin:$PATH

# Configure cargo for container environment
RUN mkdir -p $CARGO_HOME && \
    echo '[build]\njobs = 4\n\n[net]\ngit-fetch-with-cli = true' > $CARGO_HOME/config.toml

# Set up shell
COPY containers/.bashrc /home/$USERNAME/.bashrc

CMD ["/bin/bash"]
```

### 2. Create Shell Configuration

Create `containers/.bashrc`:

```bash
# Rust development environment
export PS1='🦀 \[\033[1;36m\]\u@rust-dev\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]\$ '

# Aliases
alias ll='ls -la'
alias cb='cargo build'
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo check'
alias cf='cargo fmt'
alias ccl='cargo clippy'
alias cw='cargo watch -x check -x test -x run'

# Cargo env
source "$HOME/.cargo/env"

# Welcome message
echo "🦀 Rust Development Container"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""
```

### 3. Create Podman Compose File

Create `containers/podman-compose.yml`:

```yaml
version: '3.8'

services:
  rust-dev:
    build:
      context: ..
      dockerfile: containers/Dockerfile.dev
      args:
        USER_UID: ${USER_UID:-1000}
        USER_GID: ${USER_GID:-1000}
    image: rust-bootcamp:dev
    container_name: rust-bootcamp-dev
    hostname: rust-dev
    volumes:
      # Mount entire bootcamp directory
      - ..:/workspace:z
      # Mount cargo registry cache (reuse between containers)
      - cargo-registry:/home/rustdev/.cargo/registry
      # Mount cargo git cache
      - cargo-git:/home/rustdev/.cargo/git
      # Mount target directory (optional, for build cache)
      - target-cache:/workspace/target
    environment:
      - RUST_BACKTRACE=1
      - CARGO_TARGET_DIR=/workspace/target
    working_dir: /workspace
    command: /bin/bash
    stdin_open: true
    tty: true
    # Development ports (adjust as needed)
    ports:
      - "8080:8080"   # Web server
      - "3000:3000"   # Development server
      - "9229:9229"   # Debug port

volumes:
  cargo-registry:
  cargo-git:
  target-cache:
```

### 4. VS Code Dev Container Configuration

Create `.devcontainer/devcontainer.json`:

```json
{
    "name": "Rust Bootcamp Dev Container",
    "dockerComposeFile": [
        "../containers/podman-compose.yml"
    ],
    "service": "rust-dev",
    "workspaceFolder": "/workspace",
    
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "vadimcn.vscode-lldb",
                "serayuzgur.crates",
                "tamasfe.even-better-toml",
                "usernamehw.errorlens"
            ],
            "settings": {
                "terminal.integrated.defaultProfile.linux": "bash",
                "rust-analyzer.server.path": "/home/rustdev/.cargo/bin/rust-analyzer"
            }
        }
    },
    
    "remoteUser": "rustdev",
    "postCreateCommand": "cargo --version",
    
    "features": {
        "ghcr.io/devcontainers/features/git:1": {},
        "ghcr.io/devcontainers/features/github-cli:1": {}
    }
}
```

## 🛠️ Usage Instructions

### Building the Container

```bash
# Navigate to the bootcamp directory
cd "Rust Bootcamp"

# Build the container image
podman build -f containers/Dockerfile.dev -t rust-bootcamp:dev .

# Or using podman-compose
podman-compose -f containers/podman-compose.yml build
```

### Running the Container

```bash
# Start interactive container
podman run -it --rm \
    -v $(pwd):/workspace:z \
    -w /workspace \
    rust-bootcamp:dev

# Or using podman-compose
podman-compose -f containers/podman-compose.yml run --rm rust-dev

# For VS Code development
code . --folder-uri="vscode-remote://dev-container+$(pwd)/workspace"
```

### Platform-Specific Commands

#### macOS
```bash
# Start Podman machine if needed
podman machine start

# Run with macOS-specific volume options
podman run -it --rm \
    -v $(pwd):/workspace:z \
    --security-opt label=disable \
    rust-bootcamp:dev
```

#### Windows
```powershell
# Start Podman machine if needed
podman machine start

# Run with Windows paths
podman run -it --rm `
    -v ${PWD}:/workspace:z `
    -w /workspace `
    rust-bootcamp:dev
```

## 🔧 Advanced Configuration

### Custom Rust Toolchain

Create `containers/Dockerfile.custom`:

```dockerfile
FROM rust:1.75

# Install specific Rust version
RUN rustup install 1.70.0
RUN rustup install nightly

# Set default toolchain
RUN rustup default stable

# Install additional targets
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-pc-windows-gnu

# Install wasm-pack for WebAssembly development
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Performance Optimization Container

Create `containers/Dockerfile.perf`:

```dockerfile
FROM rust-bootcamp:dev

# Install additional performance tools
USER root
RUN apt-get update && apt-get install -y \
    linux-perf \
    kcachegrind \
    massif-visualizer \
    && rm -rf /var/lib/apt/lists/*

# Install Rust performance tools
USER rustdev
RUN cargo install \
    cargo-profiling \
    cargo-instruments \
    measureme \
    inferno

# Configure perf permissions
USER root
RUN echo 'kernel.perf_event_paranoid=1' >> /etc/sysctl.conf
USER rustdev
```

## 📊 Container Management

### Useful Podman Commands

```bash
# List running containers
podman ps

# List all containers
podman ps -a

# View container logs
podman logs rust-bootcamp-dev

# Execute command in running container
podman exec -it rust-bootcamp-dev cargo test

# Clean up stopped containers
podman container prune

# Remove unused images
podman image prune

# System-wide cleanup
podman system prune -a
```

### Volume Management

```bash
# List volumes
podman volume ls

# Inspect volume
podman volume inspect cargo-registry

# Clean up unused volumes
podman volume prune

# Backup cargo cache
podman run --rm \
    -v cargo-registry:/data:z \
    -v $(pwd):/backup:z \
    busybox tar czf /backup/cargo-cache.tar.gz /data
```

## 🚀 Development Workflow

### 1. Start Development Session
```bash
# Start container with compose
podman-compose -f containers/podman-compose.yml up -d

# Attach to running container
podman exec -it rust-bootcamp-dev bash
```

### 2. Inside Container Workflow
```bash
# Navigate to project
cd /workspace/01-foundations/project-calculator

# Watch for changes and auto-run
cargo watch -x check -x test -x run

# Run with release optimizations
cargo run --release

# Run benchmarks
cargo criterion
```

### 3. Debugging in Container
```bash
# Install gdb if needed
sudo apt-get update && sudo apt-get install -y gdb

# Compile with debug symbols
cargo build

# Debug with gdb
gdb target/debug/your-binary
```

## ⚡ Performance Tips

1. **Use Volume Caches**: The compose file includes volume mounts for cargo registry and build cache
2. **Bind Mount Performance**: Use `:z` flag for better SELinux performance
3. **Build Cache**: Mount target directory as volume to persist builds
4. **Multi-Stage Builds**: Use for smaller production images

## 🆘 Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   # Fix with proper volume flags
   -v $(pwd):/workspace:z
   ```

2. **Podman Machine Not Running**
   ```bash
   podman machine init
   podman machine start
   ```

3. **Slow File System on macOS**
   - Use named volumes for build artifacts
   - Consider virtiofs for better performance

4. **Windows Path Issues**
   ```powershell
   # Use WSL2 paths
   podman run -v /mnt/c/Users/...:/workspace
   ```

## 📚 Additional Resources

- [Podman Documentation](https://docs.podman.io/)
- [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers)
- [Rust Docker Images](https://hub.docker.com/_/rust)

---

Ready to start? Continue to [Module 01: Foundations](../01-foundations/README.md) →

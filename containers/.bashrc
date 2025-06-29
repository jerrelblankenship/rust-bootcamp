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

# CI/CD Integration for Rust Testing 🚀

*A comprehensive guide to integrating Rust tests in GitHub Actions, Azure DevOps, GitLab CI, and other CI/CD platforms*

## 🚀 Quick Reference

### GitHub Actions Rust Testing Template

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo test --all-features
    - run: cargo test --no-default-features
```

### Azure DevOps Pipeline Template

```yaml
# azure-pipelines.yml
trigger: [main]
pool:
  vmImage: 'ubuntu-latest'
steps:
- script: |
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    cargo test --all-features
  displayName: 'Run tests'
```

### Essential CI Commands

```bash
# Basic test commands for CI
cargo test --all-features           # Test with all features
cargo test --no-default-features   # Test minimal build
cargo test --release               # Test optimized build
cargo clippy -- -D warnings       # Linting as error
cargo fmt -- --check              # Format checking
```

---

## GitHub Actions Integration

### 1. Basic Test Workflow

```yaml
# .github/workflows/test.yml
name: Test Suite

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: |
          ${{ runner.os }}-cargo-
          
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
      
    - name: Run tests
      run: cargo test --all-features --verbose
      
    - name: Run tests without default features
      run: cargo test --no-default-features --verbose
      
    - name: Run ignored tests
      run: cargo test -- --ignored
```

### 2. Matrix Testing (Multiple Rust Versions)

```yaml
# .github/workflows/matrix-test.yml
name: Matrix Test

on: [push, pull_request]

jobs:
  test:
    name: Test
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta, nightly]
        include:
          - rust: stable
            can-fail: false
          - rust: beta
            can-fail: false
          - rust: nightly
            can-fail: true
    
    continue-on-error: ${{ matrix.can-fail }}
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust ${{ matrix.rust }}
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-${{ matrix.rust }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run tests
      run: cargo test --all-features
      
    - name: Run clippy (stable only)
      if: matrix.rust == 'stable'
      run: cargo clippy --all-targets --all-features -- -D warnings
```

### 3. Advanced Testing with Coverage

```yaml
# .github/workflows/coverage.yml
name: Coverage

on: [push, pull_request]

jobs:
  coverage:
    name: Coverage
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: llvm-tools-preview
        
    - name: Install cargo-llvm-cov
      run: cargo install cargo-llvm-cov
      
    - name: Generate coverage
      run: cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
      
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: lcov.info
        fail_ci_if_error: true
        
    - name: Upload coverage to Coveralls
      uses: coverallsapp/github-action@v2
      with:
        github-token: ${{ secrets.GITHUB_TOKEN }}
        path-to-lcov: lcov.info
```

### 4. Performance Testing in CI

```yaml
# .github/workflows/benchmark.yml
name: Benchmark

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    name: Benchmark
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-benchmark-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Run benchmarks
      run: cargo bench --all-features
      
    - name: Store benchmark result
      uses: benchmark-action/github-action-benchmark@v1
      with:
        tool: 'cargo'
        output-file-path: target/criterion/report/index.html
        github-token: ${{ secrets.GITHUB_TOKEN }}
        auto-push: true
```

### 5. Integration Testing with Services

```yaml
# .github/workflows/integration.yml
name: Integration Tests

on: [push, pull_request]

jobs:
  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:13
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
          
      redis:
        image: redis:6
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      
    - name: Wait for services
      run: |
        until pg_isready -h localhost -p 5432; do sleep 1; done
        until redis-cli -h localhost -p 6379 ping; do sleep 1; done
        
    - name: Run integration tests
      run: cargo test --test integration_tests
      env:
        DATABASE_URL: postgres://postgres:postgres@localhost/test_db
        REDIS_URL: redis://localhost:6379
```

---

## Azure DevOps Integration

### 1. Basic Pipeline Configuration

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
    - main
    - develop

pr:
  branches:
    include:
    - main

pool:
  vmImage: 'ubuntu-latest'

variables:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

stages:
- stage: Test
  displayName: 'Test Stage'
  jobs:
  - job: UnitTests
    displayName: 'Unit Tests'
    steps:
    - script: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        rustup component add rustfmt clippy
      displayName: 'Install Rust'
      
    - script: |
        source ~/.cargo/env
        cargo fmt -- --check
      displayName: 'Check formatting'
      
    - script: |
        source ~/.cargo/env
        cargo clippy --all-targets --all-features -- -D warnings
      displayName: 'Run clippy'
      
    - script: |
        source ~/.cargo/env
        cargo test --all-features --verbose
      displayName: 'Run tests'
      
    - task: PublishTestResults@2
      inputs:
        testResultsFormat: 'JUnit'
        testResultsFiles: '**/test-results.xml'
        mergeTestResults: true
        failTaskOnFailedTests: true
      condition: always()
```

### 2. Multi-Platform Testing

```yaml
# azure-pipelines.yml
strategy:
  matrix:
    linux:
      imageName: 'ubuntu-latest'
    mac:
      imageName: 'macOS-latest'
    windows:
      imageName: 'windows-latest'

pool:
  vmImage: $(imageName)

steps:
- task: RustInstaller@1
  inputs:
    rustVersion: 'stable'
    additionalTargets: ''
    addToPath: true
    
- script: cargo test --all-features
  displayName: 'Run tests'
```

### 3. Azure DevOps with Coverage

```yaml
# azure-pipelines.yml
- script: |
    source ~/.cargo/env
    cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml --output-dir $(Build.SourcesDirectory)
  displayName: 'Generate coverage'
  
- task: PublishCodeCoverageResults@1
  inputs:
    codeCoverageTool: 'Cobertura'
    summaryFileLocation: '$(Build.SourcesDirectory)/cobertura.xml'
```

---

## GitLab CI Integration

### 1. Basic GitLab CI Configuration

```yaml
# .gitlab-ci.yml
image: rust:latest

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo
  RUST_BACKTRACE: 1

cache:
  paths:
    - .cargo/
    - target/

stages:
  - test
  - coverage

before_script:
  - rustc --version && cargo --version
  - rustup component add rustfmt clippy

test:
  stage: test
  script:
    - cargo fmt -- --check
    - cargo clippy --all-targets --all-features -- -D warnings
    - cargo test --all-features --verbose
  except:
    - tags

test:nightly:
  stage: test
  image: rustlang/rust:nightly
  script:
    - cargo test --all-features
  allow_failure: true

coverage:
  stage: coverage
  script:
    - cargo install cargo-tarpaulin
    - cargo tarpaulin --out Xml
  coverage: '/(\d+\.\d+)% coverage/'
  artifacts:
    reports:
      cobertura: cobertura.xml
```

### 2. GitLab CI with Docker

```yaml
# .gitlab-ci.yml
test:docker:
  image: rust:1.70
  services:
    - postgres:13
    - redis:6
  variables:
    POSTGRES_DB: test_db
    POSTGRES_USER: postgres
    POSTGRES_PASSWORD: postgres
    DATABASE_URL: postgres://postgres:postgres@postgres/test_db
    REDIS_URL: redis://redis:6379
  script:
    - cargo test --test integration_tests
```

---

## Jenkins Integration

### 1. Jenkinsfile for Rust Testing

```groovy
// Jenkinsfile
pipeline {
    agent any
    
    environment {
        CARGO_HOME = "${WORKSPACE}/.cargo"
        RUST_BACKTRACE = '1'
    }
    
    stages {
        stage('Setup') {
            steps {
                sh '''
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source ~/.cargo/env
                    rustup component add rustfmt clippy
                '''
            }
        }
        
        stage('Format Check') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo fmt -- --check
                '''
            }
        }
        
        stage('Lint') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo clippy --all-targets --all-features -- -D warnings
                '''
            }
        }
        
        stage('Test') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo test --all-features --verbose
                '''
            }
            post {
                always {
                    publishTestResults testResultsPattern: 'target/test-results.xml'
                }
            }
        }
        
        stage('Coverage') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo install cargo-tarpaulin
                    cargo tarpaulin --out Xml
                '''
            }
            post {
                always {
                    publishCoverage adapters: [coberturaAdapter('cobertura.xml')], sourceFileResolver: sourceFiles('STORE_LAST_BUILD')
                }
            }
        }
    }
    
    post {
        cleanup {
            cleanWs()
        }
    }
}
```

---

## CircleCI Integration

### 1. CircleCI Configuration

```yaml
# .circleci/config.yml
version: 2.1

orbs:
  rust: circleci/rust@1.6.0

workflows:
  test_and_build:
    jobs:
      - rust/test:
          version: "1.70.0"
          with_cache: true
      - rust/clippy:
          version: "1.70.0"
          with_cache: true
      - rust/format:
          version: "1.70.0"
          with_cache: true

jobs:
  coverage:
    docker:
      - image: cimg/rust:1.70
    steps:
      - checkout
      - rust/install:
          version: "1.70.0"
      - run:
          name: Install tarpaulin
          command: cargo install cargo-tarpaulin
      - run:
          name: Generate coverage
          command: cargo tarpaulin --out Xml
      - run:
          name: Upload coverage
          command: bash <(curl -s https://codecov.io/bash)
```

---

## Docker Integration for Testing

### 1. Multi-Stage Docker Testing

```dockerfile
# Dockerfile.test
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build dependencies separately for caching
RUN cargo build --release

# Test stage
FROM rust:1.70 as tester

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

# Install test dependencies
RUN cargo install cargo-tarpaulin

# Run tests
RUN cargo test --all-features
RUN cargo tarpaulin --out Xml

# Final stage
FROM debian:bullseye-slim

COPY --from=builder /app/target/release/my_app /usr/local/bin/
COPY --from=tester /app/cobertura.xml /tmp/

CMD ["my_app"]
```

### 2. Docker Compose for Integration Tests

```yaml
# docker-compose.test.yml
version: '3.8'

services:
  test-runner:
    build:
      context: .
      dockerfile: Dockerfile.test
    depends_on:
      - postgres
      - redis
    environment:
      - DATABASE_URL=postgres://postgres:password@postgres:5432/testdb
      - REDIS_URL=redis://redis:6379
    volumes:
      - ./target:/app/target
      - ./test-results:/app/test-results

  postgres:
    image: postgres:13
    environment:
      POSTGRES_DB: testdb
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"

  redis:
    image: redis:6
    ports:
      - "6379:6379"
```

---

## Advanced CI/CD Patterns

### 1. Feature Branch Testing

```yaml
# .github/workflows/feature-branch.yml
name: Feature Branch Tests

on:
  pull_request:
    branches: [ main ]

jobs:
  quick-test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    
    # Quick feedback for developers
    - name: Quick check
      run: |
        cargo check --all-features
        cargo clippy --all-targets -- -D warnings
        
    # Run only fast tests
    - name: Fast tests
      run: cargo test --lib --bins
      
  full-test:
    runs-on: ubuntu-latest
    needs: quick-test
    if: success()
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    
    # Full test suite after quick check passes
    - name: Full test suite
      run: cargo test --all-features --verbose
      
    - name: Integration tests
      run: cargo test --test '*'
```

### 2. Nightly Rust Testing

```yaml
# .github/workflows/nightly.yml
name: Nightly Rust

on:
  schedule:
    - cron: '0 0 * * *'  # Daily at midnight
  workflow_dispatch:

jobs:
  nightly-test:
    runs-on: ubuntu-latest
    continue-on-error: true
    
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@nightly
      with:
        components: rustfmt, clippy
        
    - name: Test with nightly
      run: cargo test --all-features
      
    - name: Nightly clippy
      run: cargo clippy --all-targets --all-features
      
    - name: Create issue on failure
      if: failure()
      uses: actions/github-script@v6
      with:
        script: |
          github.rest.issues.create({
            owner: context.repo.owner,
            repo: context.repo.repo,
            title: 'Nightly Rust build failure',
            body: 'The nightly Rust build failed. Please investigate.'
          })
```

### 3. Performance Regression Testing

```yaml
# .github/workflows/performance.yml
name: Performance

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Fetch full history for comparison
        
    - uses: dtolnay/rust-toolchain@stable
    
    - name: Install criterion
      run: cargo install cargo-criterion
      
    - name: Run benchmarks (baseline)
      if: github.event_name == 'pull_request'
      run: |
        git checkout ${{ github.event.pull_request.base.sha }}
        cargo bench --bench performance -- --save-baseline main
        
    - name: Run benchmarks (current)
      if: github.event_name == 'pull_request'
      run: |
        git checkout ${{ github.event.pull_request.head.sha }}
        cargo bench --bench performance -- --baseline main
        
    - name: Comment performance results
      if: github.event_name == 'pull_request'
      uses: actions/github-script@v6
      with:
        script: |
          // Read benchmark results and comment on PR
          const fs = require('fs');
          const results = fs.readFileSync('target/criterion/report/index.html', 'utf8');
          // Process and comment results...
```

---

## Test Result Reporting

### 1. JUnit XML Output

```toml
# Cargo.toml
[dev-dependencies]
junit-report = "0.8"
```

```rust
// Custom test harness for JUnit output
use junit_report::*;

#[test]
fn generate_junit_report() {
    let mut report = TestSuite::new("my_test_suite");
    
    // Add test cases
    let test_case = TestCase::success("test_function", Duration::from_millis(100));
    report.add_testcase(test_case);
    
    // Write to file
    let xml = report.to_string().unwrap();
    std::fs::write("test-results.xml", xml).unwrap();
}
```

### 2. Custom Test Output

```bash
# CI script for custom test output
#!/bin/bash

# Run tests with JSON output
cargo test --message-format=json > test-results.json

# Parse JSON and generate reports
python3 parse_test_results.py test-results.json

# Upload results to test reporting service
curl -X POST -H "Content-Type: application/json" \
     -d @test-results.json \
     https://test-reporting-service.com/api/results
```

---

## Monitoring and Alerting

### 1. Test Failure Notifications

```yaml
# .github/workflows/notify.yml
- name: Notify on failure
  if: failure()
  uses: 8398a7/action-slack@v3
  with:
    status: failure
    channel: '#dev-alerts'
    webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

### 2. Performance Monitoring

```yaml
# Performance monitoring integration
- name: Monitor performance
  run: |
    cargo bench --bench performance > bench-results.txt
    
    # Send metrics to monitoring system
    curl -X POST https://metrics.company.com/api/metrics \
         -H "Authorization: Bearer ${{ secrets.METRICS_TOKEN }}" \
         -d @bench-results.txt
```

---

## Best Practices for CI/CD Testing

### 1. Test Categorization

```rust
// Use feature flags for test categories
#[cfg(test)]
mod tests {
    #[test]
    fn unit_test() { /* Always run */ }
    
    #[test]
    #[cfg(feature = "integration-tests")]
    fn integration_test() { /* Only in CI */ }
    
    #[test]
    #[cfg(feature = "slow-tests")]
    fn performance_test() { /* Only on main branch */ }
}
```

### 2. Environment-Specific Configuration

```yaml
# Different test configurations per environment
test-dev:
  script: cargo test --features dev-tests
  
test-staging:
  script: cargo test --features staging-tests
  
test-prod:
  script: cargo test --features prod-tests --release
```

### 3. Parallel Test Execution

```bash
# Use nextest for better parallelization
cargo install cargo-nextest

# Run tests in parallel with custom configuration
cargo nextest run --test-threads 4 --retries 2
```

### 4. Test Artifacts and Caching

```yaml
# GitHub Actions caching strategy
- name: Cache test dependencies
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: test-${{ runner.os }}-${{ hashFiles('**/Cargo.lock') }}
    
- name: Upload test artifacts
  uses: actions/upload-artifact@v3
  with:
    name: test-results
    path: |
      target/criterion/
      test-results.xml
      coverage.xml
```

---

## Troubleshooting CI/CD Issues

### 1. Common CI/CD Problems

**Flaky Tests:**
```bash
# Run tests multiple times to identify flaky tests
for i in {1..10}; do
    cargo test || echo "Failure on run $i"
done
```

**Memory Issues:**
```yaml
# Increase memory for test jobs
jobs:
  test:
    runs-on: ubuntu-latest-4-cores  # Use larger runner
    env:
      RUST_MIN_STACK: 8388608       # Increase stack size
```

**Timeout Issues:**
```yaml
# Set appropriate timeouts
- name: Run tests
  run: cargo test --all-features
  timeout-minutes: 30
```

### 2. Debug CI/CD Issues

```yaml
# Enable debug mode
- name: Debug CI environment
  run: |
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Environment variables:"
    env | grep -E "(CARGO|RUST)" | sort
    echo "Available memory:"
    free -h
    echo "Available disk space:"
    df -h
```

---

## Conclusion

Effective CI/CD integration for Rust testing provides:

1. **Automated Quality Gates** - Prevent regressions from reaching production
2. **Multi-Platform Validation** - Ensure code works across different environments
3. **Performance Monitoring** - Track performance regressions over time
4. **Comprehensive Coverage** - Combine unit, integration, and performance tests
5. **Fast Feedback Loops** - Quick developer feedback on code changes

Key success factors:
- **Appropriate test categorization** for different CI stages
- **Effective caching strategies** for faster build times
- **Proper resource allocation** for test execution
- **Comprehensive monitoring and alerting** for test failures
- **Integration with external services** for enhanced testing capabilities

With proper CI/CD integration, Rust testing becomes an integral part of the development workflow, ensuring code quality and preventing regressions at every stage of the development process.
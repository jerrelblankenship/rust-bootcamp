# Project Documentation

This is a sample markdown file for testing the file processor.

## Features

The file processor supports multiple formats:

- **JSON**: JavaScript Object Notation
- **CSV**: Comma-Separated Values  
- **TSV**: Tab-Separated Values
- **Text**: Plain text files
- **Markdown**: Formatted text files
- **Logs**: Application log files

## Usage Examples

Here are some common usage patterns:

### Basic Processing

```bash
file-processor input.json --output results/
```

### Batch Processing

```bash
file-processor *.csv --format json --validate
```

## Error Handling

The processor includes comprehensive error handling for:

1. File not found errors
2. Permission denied issues
3. Invalid format detection
4. Data validation failures
5. Processing timeouts

## Configuration

Configuration can be provided via:
- Command line arguments
- Configuration files
- Environment variables

---

*Generated by the Rust File Processor Example*

# Exercise 05 - Level 1 Hints: Cross-Platform Chaos

## 🎯 What's the Problem?

This code only works on Unix-like systems (Linux/macOS) and will fail on Windows. It uses platform-specific paths, commands, and system calls.

## 🔍 Identify the Issues

1. **Hard-coded paths**: `/home/user`, `/var/log`
2. **Unix commands**: `ls`, `mkdir -p`, `touch`, `chmod`
3. **Platform-specific APIs**: `libc::getpid()`

## 🤔 C# Comparison

This is like using:
- `Registry` APIs on Windows
- `Environment.SpecialFolder` for paths
- `Path.Combine()` for portable paths

## 🔧 What to Research

1. **Cross-platform path handling**: `std::path::Path`
2. **Standard directories**: `dirs` crate
3. **Process management**: `std::process` instead of `libc`
4. **File operations**: `std::fs` instead of shell commands

## ⏰ Time Check

Spent 15 minutes? Move to Level 2 for specific solutions.
# Bunker Terminal Agent

`bunker-terminal-agent-v1` is an offline-first, privacy-preserving AI agent designed to operate within terminal environments. It serves as a local command assistant that transforms natural language queries into safe, explainable shell commands. Rather than training a new model, Bunker orchestrates command reasoning using lightweight foundation models (like GPT-4.1-Nano or Phi-3 Mini), structured prompting, semantic memory, and optional plugin tooling.

It is built for environments that demand speed, safety, and privacy—making it ideal for developers, researchers, traders, and systems analysts.

## Features

- **Privacy-First**: Fully offline operation with zero telemetry
- **Multi-Model Support**: Optimized for various hardware from Raspberry Pi to high-end workstations
- **Safety First**: Command risk classification and confirmation for potentially dangerous operations
- **Extensible**: Plugin system for domain-specific functionality
- **Context-Aware**: Learns from your usage patterns and preferences

## Safety Features

- **Command Classification**: All commands are classified by risk level
- **Confirmation Required**: Potentially dangerous operations require explicit confirmation
- **Audit Logging**: All commands and their outcomes are logged
- **Memory Management**: Full control over what's remembered and for how long

## Documentation

For detailed documentation, please refer to the [docs](./docs) directory.

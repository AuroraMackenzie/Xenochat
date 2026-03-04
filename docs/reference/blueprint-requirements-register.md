# Blueprint Requirements Register

## REQ coverage
- REQ-001 Multi-model collaboration.
- REQ-002 Memory system (short/long-term retrieval).
- REQ-003 Emotion state model.
- REQ-004 Expression learning.
- REQ-005 Behavior planner.
- REQ-006 Tool invocation.
- REQ-007 Plugin system.
- REQ-008 Web UI and API.
- REQ-009 Streaming response.
- REQ-010 Multimodal message model.
- REQ-011 Partitioned config with reload support.
- REQ-012 Launcher/CLI startup.
- REQ-013 Keyword trigger engine.
- REQ-014 STT/TTS pipeline.
- REQ-015 Emoji package strategy.
- REQ-016 Dashboard monitoring.
- REQ-017 Local debug chat.
- REQ-018 Visual config center.
- REQ-019 Model/provider management.
- REQ-020 Adapter configuration management.
- REQ-021 Realtime log stream.
- REQ-022 Plugin management page.
- REQ-023 Resource management.
- REQ-024 System settings and RBAC.
- REQ-025 Bidirectional websocket bridge contract.
- REQ-026 Queue ordering and backpressure.
- REQ-027 Hot reload support.
- REQ-028 OneBot event/action support.
- REQ-029 Multi-network access modes.
- REQ-030 Native cross-platform binaries.
- REQ-031 Whitelist controls.
- REQ-032 L1 data import analysis.
- REQ-033 Data lifecycle management.
- REQ-034 Metal GPU acceleration.
- REQ-035 Secure key management.
- REQ-036 Structured audit logging.

## ADV inheritance
- ADV-001 through ADV-016: modularity, configurability, plugin/event-driven style, realtime logs, visual ops, cross-platform runtime, queue resilience, multi-model collaboration, human-like interaction strategy, deployment friendliness, process hardening, performance, API breadth, community practices, and transparent support-level communication.

## FIX elimination targets
- FIX-001 through FIX-035 are tracked as security and quality gates.
- Priority order: P0 first (auth/logs/network/CORS/query-token/queue/config), then P1 (type safety, file size, dependency and test governance), then P2 (resilience and operational maturity).

## Current implementation note
This repository currently delivers a strong Rust baseline and shared adapter contract, plus an implemented security baseline for encrypted keys, strict API auth/CORS, STRIDE threat modeling, and automated vulnerability scans. Full production parity for all REQ/ADV/FIX items is tracked in `TODOWRITE_STATUS.md` and will be expanded in iterative blocks.

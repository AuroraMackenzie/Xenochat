# Xenochat System Architecture

## Layer model
1. `protocol`: wire-level abstractions and transport contracts.
2. `adapter`: platform bridge, queueing, import contracts.
3. `core`: collaboration, memory, safety, tools, persona, triggering.
4. `api`: route access layer, auth and origin checks.
5. `bin`: composition root and runtime startup.

## Runtime data flow
1. Platform event reaches an adapter.
2. Adapter validates and normalizes to `core::Message`.
3. Core planner decides `Reply | AskForClarification | Skip`.
4. Safety guard screens the request.
5. Collaborator calls model providers.
6. Optional tool invocations run via the tool registry.
7. Outbound message returns through adapter transport.

## Safety defaults
- Public bind is blocked unless API keys are present.
- Wildcard CORS is rejected by config validation.
- Queue capacity cannot be zero.
- Secret redaction helper protects logs.

## Platform contract parity
Each platform crate implements:
- `PlatformAdapter` for queue-based runtime messaging.
- `ImportContract` for L1 export discovery, parsing, normalization, and checkpointing.

## Metal acceleration strategy
- `gpu::GpuProbe` detects Apple Silicon + Metal capability.
- Runtime reports backend details.
- CPU fallback remains available when Metal is unavailable.

# Legacy Ecosystem Complete Audit

This file consolidates all predecessor analysis points into one implementation-facing reference.

## 1) MaiBot

### Functional scope
1. Multi-model collaborative dialogue.
2. Bionic memory and persistent cognition.
3. Plugin architecture with hot-loading patterns.
4. Emotion engine with expressive behavior.
5. Expression learning from group language style.
6. Behavior planning and tool invocation.
7. Streaming response support (SSE).
8. Multi-platform integration through adapters.
9. Config-driven runtime (`bot_config.toml`).
10. Docker and launcher deployment paths.

### Advantages
1. Advanced cognitive architecture concept.
2. Broad feature coverage for persona-like bot behavior.
3. Runner/Worker process supervision and restart flow.
4. Bootstrapping friendliness (environment generation).
5. Graceful shutdown sequence design.
6. Deeply segmented configuration model.
7. Plugin manager lifecycle and compatibility checks.
8. WebUI/API decoupling for deployment flexibility.
9. Baseline WebUI security controls.
10. Timing-safe token comparison.
11. Multiple deployment modes.
12. Ecosystem centrality via open websocket-style integrations.

### Weaknesses and risks
1. Empty-key API auth bypass risk in some modes.
2. Sensitive token output in logs.
3. Version source inconsistency.
4. Runtime placeholder behavior still present.
5. Excess broad exception captures.
6. Oversized monolithic files reducing maintainability.
7. Loose dependency pinning and weak reproducibility.
8. Telemetry consent and privacy messaging gaps.
9. Broken or stale documentation links.
10. Plugin loading and reflection performance concerns.
11. Missing systematic error isolation and degradation strategy.
12. Input validation and key management hardening gaps.
13. Low test coverage and limited integration benchmarking.
14. Documentation depth and architecture guidance gaps.
15. Coupling hotspots and inconsistent quality checks.
16. Emotion and persona predictability risks.
17. Long-term memory drift and unsafe content injection risk.

## 2) MaiBot-Dashboard

### Functional scope
1. Real-time status and performance monitoring.
2. Plugin lifecycle management from UI.
3. Conversation record access and management.
4. Configuration editing and persistence.
5. User and permission controls.
6. Dynamic form generation from backend metadata.
7. Token auth and websocket log stream.
8. Built-in local chat panel for debugging.

### Advantages
1. Modern frontend stack (React + Vite + TypeScript).
2. Responsive interaction and coherent page structure.
3. Strong component-level separation.
4. Reasonable API integration flow.
5. Auth-aware fetch and routing design.
6. Full set of key management pages in original scope.

### Weaknesses and risks
1. Async auth guard misuse causing potential auth bypass.
2. Archived repository state reduces future value.
3. Partially completed page/function set.
4. Missing split/loading optimization in some areas.
5. XSS/CSRF/input-validation hardening not complete.

## 3) MaiBot-Napcat-Adapter

### Functional scope
1. Protocol conversion (NapCat OneBot <-> MaiBot model).
2. Bidirectional websocket message routing.
3. Status sync and reconnection logic.
4. Multi-message-type transformation.
5. Configuration hot reload.
6. Legacy and API-server communication modes.
7. Queue-based asynchronous message pipeline.

### Advantages
1. Single-responsibility adapter boundary.
2. Queue architecture for burst handling.
3. Broad message/event TODO coverage targets.
4. Flexible deployment modes.
5. Active community and release cadence.
6. Command registration pattern supports extensibility.
7. Runtime reload and reconnection controls.

### Weaknesses and risks
1. Unbounded inbound queue and fixed sleep delay path.
2. Broad exception capture density.
3. Incorrect declaration of stdlib module as dependency.
4. Loose dependency strategy.
5. Security baseline not enforced by default.
6. Several advanced features still incomplete.
7. Strong dependency on protocol-side API stability.
8. Limited standalone troubleshooting documentation.
9. Missing complete unit/integration/performance tests.
10. Extensibility limited by hard-coded paths.

## 4) NapCatQQ

### Functional scope
1. NTQQ protocol implementation.
2. OneBot API and event interface.
3. Multi-platform runtime support.
4. Plugin/extension capability.
5. Native acceleration modules.
6. Large API surface for messaging and group operations.
7. Event listener and dispatch systems.
8. Optional web management interfaces.

### Advantages
1. Strong architectural discipline and service decomposition.
2. High-quality type system and error modeling.
3. Performance-oriented implementation paths.
4. Extensibility and pluggable service style.
5. Mature cross-platform packaging.
6. Strong developer ergonomics.
7. Security-focused design capabilities.
8. Mature engineering and monorepo process.
9. Large active community and contribution scale.
10. Extensive API/event coverage.
11. Config schema validation robustness.
12. Multiple network adapter patterns.
13. WebUI security features with stronger auth options.

### Weaknesses and risks
1. Credential expiry logic bug due unit mismatch.
2. Sensitive token logging exposure.
3. Wide network exposure defaults in some modes.
4. Overly permissive CORS reflection behavior.
5. Query-token auth pattern leakage risk.
6. High amount of `any/ts-ignore` debt.
7. TODO/FIXME/HACK backlog.
8. Restrictive licensing constraints.
9. Maintainer continuity uncertainty.
10. High learning curve and configuration complexity.
11. Native-module environment complexity.
12. Documentation depth and troubleshooting gaps.
13. Deployment complexity and onboarding friction.
14. Ecosystem/plugin maturity variability.
15. Compatibility break risk across API changes.
16. Platform compliance and account-risk concerns.
17. Documentation site install page quality issue.

## 5) nonebot-plugin-maibot-adapters

### Functional scope
1. NoneBot-side protocol adaptation.
2. Message routing between NoneBot and MaiBot.
3. Conversion for common media/message events.
4. Whitelist filtering behavior.

### Advantages
1. Honest project-state communication.
2. Lightweight integration path.
3. Coverage of common event/message categories.

### Weaknesses and risks
1. Config path inconsistency and runtime mismatch risk.
2. Whitelist type mismatch (`str` vs `int`) risk.
3. Naming typo (`platfrom`) across paths.
4. UTF-16LE requirements file compatibility issue.
5. Missing or unclear license file risk.
6. Broad exception captures.
7. Semi-deprecated status and migration pressure.
8. Documentation and deployment complexity.
9. Code duplication and weak type discipline.
10. Image path performance limitations.
11. Security hardening gaps.
12. Missing test coverage baseline.

## 6) Documentation websites

### napneko docs (`guide/start-install`)
- Positive intent: install guidance and structured docs path.
- Observed issue: critical page content mismatch/invalid rendering in reported analysis.
- Required action: restore valid install content and add troubleshooting sections.

### mai-mai docs
- Positive intent: complete domain navigation for getting started/config/adapters.
- Observed issue: missing deep architecture/API/config internals in reported analysis.
- Required action: rebuild detailed reference and keep docs synchronized with code merges.

## 7) System-wide strengths inherited
1. Layered architecture (protocol -> adapter -> intelligence).
2. Adapter-based decoupling from platform specifics.
3. Model-provider replaceability.
4. Plugin extension concept.
5. Web management capabilities.
6. Multi-model collaboration design.
7. Emotion and memory concepts.
8. Keyword trigger model.
9. Tool invocation architecture.
10. Voice interaction capability target.
11. Emoji and expression behavior path.
12. Logging and telemetry channels.
13. Real-time websocket visibility.
14. Local chat debugging mode.
15. Visual configuration management.
16. Group-language style assimilation target.
17. Existing documentation site patterns.
18. Deployment guideline baseline.
19. Config standards and adapter references.
20. Explicit TODO backlog visibility.
21. Modern language and framework stacks.
22. Auto-form generation concept.
23. Queue-based adapter processing.
24. Human-like conversation objective.
25. Natural style and rhythm emphasis.
26. Dynamic persona direction.

## 8) System-wide weaknesses and risks
1. Multi-hop websocket chain increases single-point break probability.
2. No mature high-availability or distributed architecture baseline.
3. Inconsistent retry/recovery standards.
4. No global dedupe and consistency transaction model.
5. Missing system-wide SLA/limit model.
6. No complete performance baseline disclosure.
7. Persona unpredictability and unsafe behavior drift risk.
8. Memory misreference and persona drift risk.
9. No full prompt-injection defense architecture in legacy state.
10. Adapter feature incompleteness across long-tail events.
11. Compliance and platform policy risks.
12. Model API cost and quota volatility.
13. Missing unified downgrade and health-center path.
14. Missing alerting/log aggregation/backup baselines.
15. Documentation lag and coverage gaps.
16. Uneven maturity across repositories.
17. License/governance uncertainty in parts of legacy ecosystem.
18. Weak automated test coverage in non-core repositories.
19. Loose dependency pinning and reproducibility gaps.
20. Unsafe default exposure if deployment hardening is skipped.

## 9) Consolidated risk inventory
1. Account bans and service restrictions.
2. Legal and policy compliance violations.
3. License interpretation disputes.
4. Data leakage through logs or headers.
5. Prompt injection and command hijacking.
6. Tool abuse and message storms.
7. Infinite reply loops.
8. Memory leaks and socket deadlocks.
9. Adapter crash cascade.
10. Upstream model outages.
11. Context bloat and budget overruns.
12. Emotion/persona instability.
13. Unsafe learned slang propagation.
14. Log explosion and resource exhaustion.
15. Concurrency races and unhandled exception hangs.
16. CORS and public-bind misconfiguration attack surface.
17. Credential expiry and auth flaws.
18. Dependency vulnerabilities.
19. Version compatibility breakage.

## 10) Xenochat migration rule
Every listed advantage above must be retained or improved.
Every listed weakness and risk above must be mitigated with default-safe implementation and testable controls.

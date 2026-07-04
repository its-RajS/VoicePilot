# Graph Report - .  (2026-07-04)

## Corpus Check
- Corpus is ~5,269 words - fits in a single context window. You may not need a graph.

## Summary
- 230 nodes · 218 edges · 46 communities (38 shown, 8 thin omitted)
- Extraction: 80% EXTRACTED · 16% INFERRED · 5% AMBIGUOUS · INFERRED: 34 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Frontend Dependencies|Frontend Dependencies]]
- [[_COMMUNITY_TypeScript Compiler|TypeScript Compiler]]
- [[_COMMUNITY_Target Architecture|Target Architecture]]
- [[_COMMUNITY_Config Models|Config Models]]
- [[_COMMUNITY_Runtime Services|Runtime Services]]
- [[_COMMUNITY_Tauri Commands|Tauri Commands]]
- [[_COMMUNITY_Graphify Detection|Graphify Detection]]
- [[_COMMUNITY_Tauri Window Config|Tauri Window Config]]
- [[_COMMUNITY_Prompt Pipeline Goals|Prompt Pipeline Goals]]
- [[_COMMUNITY_Inference Placeholders|Inference Placeholders]]
- [[_COMMUNITY_Config Command Path|Config Command Path]]
- [[_COMMUNITY_Path Utilities|Path Utilities]]
- [[_COMMUNITY_App Bootstrap|App Bootstrap]]
- [[_COMMUNITY_Inference Pipeline Core|Inference Pipeline Core]]
- [[_COMMUNITY_React Surface|React Surface]]
- [[_COMMUNITY_Transcript Events|Transcript Events]]
- [[_COMMUNITY_Transcript Store|Transcript Store]]
- [[_COMMUNITY_Health Endpoint|Health Endpoint]]
- [[_COMMUNITY_Pipeline State|Pipeline State]]
- [[_COMMUNITY_Build Script|Build Script]]
- [[_COMMUNITY_Dev Setup Script|Dev Setup Script]]
- [[_COMMUNITY_Error Model|Error Model]]
- [[_COMMUNITY_Session State|Session State]]
- [[_COMMUNITY_Frontend Config Type|Frontend Config Type]]

## God Nodes (most connected - your core abstractions)
1. `compilerOptions` - 16 edges
2. `AppConfig` - 7 edges
3. `files` - 6 edges
4. `scripts` - 6 edges
5. `Push-To-Talk Voice To Prompt Flow` - 6 edges
6. `Hybrid Local-First Architecture` - 6 edges
7. `Phase 0 Milestone` - 6 edges
8. `Services Module` - 6 edges
9. `build` - 5 edges
10. `Pipeline Orchestrator` - 5 edges

## Surprising Connections (you probably didn't know these)
- `Audio Table` --implements--> `Push-To-Talk Voice To Prompt Flow`  [INFERRED]
  mvp-voicePilot/config/schema.sql → docs/product-specification.md
- `Hotkey Table` --implements--> `Push-To-Talk Voice To Prompt Flow`  [INFERRED]
  mvp-voicePilot/config/schema.sql → docs/product-specification.md
- `Pipeline State Enum` --implements--> `Push-To-Talk Voice To Prompt Flow`  [INFERRED]
  mvp-voicePilot/inference/src/voicepilot_inference/core/state.py → docs/product-specification.md
- `Pipeline Orchestrator` --implements--> `Local LLM Prompt Cleanup`  [AMBIGUOUS]
  mvp-voicePilot/inference/src/voicepilot_inference/core/pipeline.py → docs/product-specification.md
- `Pipeline Orchestrator` --conceptually_related_to--> `Push-To-Talk Voice To Prompt Flow`  [INFERRED]
  mvp-voicePilot/inference/src/voicepilot_inference/core/pipeline.py → docs/product-specification.md

## Import Cycles
- 1-file cycle: `mvp-voicePilot/src-tauri/src/commands/mod.rs -> mvp-voicePilot/src-tauri/src/commands/mod.rs`
- 1-file cycle: `mvp-voicePilot/src-tauri/src/utils/paths.rs -> mvp-voicePilot/src-tauri/src/utils/paths.rs`

## Hyperedges (group relationships)
- **Hybrid Local-First Application Stack** — architecture_tauri_rust_shell, architecture_react_ui, architecture_python_fastapi [EXTRACTED 1.00]
- **Runtime Settings Schema** — schema_config_table, schema_hotkey_table, schema_audio_table, schema_models_table [EXTRACTED 1.00]
- **Phase 0 Bootstrap Slice** — development_phase0, main_fastapi_app, health_health_endpoint, ci_checks_workflow, schema_config_table [INFERRED 0.90]
- **Desktop Shell Assembly** — main_rust_entrypoint, lib_run, tauri_conf_runtime_config, app_app_component [INFERRED 0.80]
- **Config Delivery Path** — commands_get_config, config_appconfig, config_appconfig_default [EXTRACTED 1.00]
- **Inference Engine Placeholders** — llm_client_enginenotimplemented, stt_engine_enginenotimplemented, vad_engine_enginenotimplemented [EXTRACTED 1.00]
- **Services Subsystem** — audio_service_serviceboundary, config_service_serviceboundary, hotkey_service_serviceboundary, ipc_client_serviceboundary, tray_manager_serviceboundary, window_service_serviceboundary [EXTRACTED 1.00]
- **Voice Interaction Feedback Loop** — hotkey_service_serviceboundary, app_state_sessionstate, transcriptstore_usetranscriptstore, liveoverlay_liveoverlay [INFERRED 0.72]
- **Configuration Surface** — config_service_serviceboundary, config_voicepilotconfig, paths_config_dir [INFERRED 0.80]

## Communities (46 total, 8 thin omitted)

### Community 0 - "Frontend Dependencies"
Cohesion: 0.08
Nodes (23): dependencies, lucide-react, react, react-dom, @tauri-apps/api, typescript, vite, @vitejs/plugin-react (+15 more)

### Community 1 - "TypeScript Compiler"
Cohesion: 0.11
Nodes (18): compilerOptions, allowJs, allowSyntheticDefaultImports, esModuleInterop, forceConsistentCasingInFileNames, isolatedModules, jsx, lib (+10 more)

### Community 2 - "Target Architecture"
Cohesion: 0.18
Nodes (17): Hybrid Local-First Architecture, Python FastAPI Inference Layer, React TypeScript UI, Tauri Rust Desktop Shell, Initial Project Scaffold, Continuous Integration Checks, No Default Logging Of Audio Or Prompts, Phase 0 Milestone (+9 more)

### Community 3 - "Config Models"
Cohesion: 0.15
Nodes (14): AudioConfig, Default, HotkeyConfig, ModelConfig, AppConfig, AudioConfig, HotkeyConfig, ModelConfig (+6 more)

### Community 4 - "Runtime Services"
Cohesion: 0.18
Nodes (15): SessionState, Audio Service Boundary, Config Service Boundary, VoicePilotConfig, Hotkey Service Boundary, IPC Client Service Boundary, LiveOverlay, Services Module (+7 more)

### Community 5 - "Tauri Commands"
Cohesion: 0.16
Nodes (15): Get Config Command, Health Command, AppConfig, AppConfig Default, AudioConfig, ModelConfig, Engineering System Prompt, VoicePilotError (+7 more)

### Community 6 - "Graphify Detection"
Cohesion: 0.14
Nodes (13): files, code, document, image, paper, video, graphifyignore_patterns, needs_graph (+5 more)

### Community 7 - "Tauri Window Config"
Cohesion: 0.14
Nodes (13): app, security, windows, build, beforeBuildCommand, beforeDevCommand, devUrl, frontendDist (+5 more)

### Community 8 - "Prompt Pipeline Goals"
Cohesion: 0.26
Nodes (12): Phase 1 Milestone, Initial Schema Migration, Pipeline Orchestrator, Pipeline Result, Modular Replaceable Service Boundaries, Local LLM Prompt Cleanup, Push-To-Talk Voice To Prompt Flow, VoicePilot (+4 more)

### Community 9 - "Inference Placeholders"
Cohesion: 0.22
Nodes (5): EngineNotImplemented, EngineNotImplemented, EngineNotImplemented, EngineNotImplemented, Exception

### Community 10 - "Config Command Path"
Cohesion: 0.53
Nodes (5): AppConfig, get_config(), health(), String, Result

### Community 11 - "Path Utilities"
Cohesion: 0.40
Nodes (5): Option, PathBuf, ProjectDirs, config_dir(), project_dirs()

### Community 12 - "App Bootstrap"
Cohesion: 0.40
Nodes (5): App Component, Workspace Build Script, Development Prerequisites Script, React Bootstrap, Tauri Runtime Config

### Community 13 - "Inference Pipeline Core"
Cohesion: 0.50
Nodes (3): PipelineOrchestrator, PipelineResult, str

### Community 15 - "Transcript Events"
Cohesion: 0.50
Nodes (3): TranscriptEvent, Option, String

### Community 16 - "Transcript Store"
Cohesion: 0.50
Nodes (3): Status, TranscriptStore, useTranscriptStore

## Ambiguous Edges - Review These
- `Local LLM Prompt Cleanup` → `Pipeline Orchestrator`  [AMBIGUOUS]
  mvp-voicePilot/inference/src/voicepilot_inference/core/pipeline.py · relation: implements
- `Config Table` → `Initial Schema Migration`  [AMBIGUOUS]
  mvp-voicePilot/config/migrations/001_initial.sql · relation: references
- `Hotkey Table` → `Initial Schema Migration`  [AMBIGUOUS]
  mvp-voicePilot/config/migrations/001_initial.sql · relation: references
- `Audio Table` → `Initial Schema Migration`  [AMBIGUOUS]
  mvp-voicePilot/config/migrations/001_initial.sql · relation: references
- `Models Table` → `Initial Schema Migration`  [AMBIGUOUS]
  mvp-voicePilot/config/migrations/001_initial.sql · relation: references
- `Pipeline Orchestrator` → `Pipeline State Enum`  [AMBIGUOUS]
  mvp-voicePilot/inference/src/voicepilot_inference/core/state.py · relation: conceptually_related_to
- `EngineNotImplemented (Postprocessor)` → `TranscriptEvent`  [AMBIGUOUS]
  mvp-voicePilot/inference/src/voicepilot_inference/engines/postprocessor.py · relation: conceptually_related_to
- `EngineNotImplemented (VAD Engine)` → `VoicePilotError`  [AMBIGUOUS]
  mvp-voicePilot/src-tauri/src/models/error.rs · relation: conceptually_related_to
- `Config Service Boundary` → `VoicePilotConfig`  [AMBIGUOUS]
  mvp-voicePilot/src-tauri/src/services/config_service.rs · relation: shares_data_with
- `LiveOverlay` → `Transcript Store`  [AMBIGUOUS]
  mvp-voicePilot/src/stores/transcriptStore.ts · relation: conceptually_related_to

## Knowledge Gaps
- **94 isolated node(s):** `code`, `document`, `paper`, `image`, `video` (+89 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What is the exact relationship between `Local LLM Prompt Cleanup` and `Pipeline Orchestrator`?**
  _Edge tagged AMBIGUOUS (relation: implements) - confidence is low._
- **What is the exact relationship between `Config Table` and `Initial Schema Migration`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Hotkey Table` and `Initial Schema Migration`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Audio Table` and `Initial Schema Migration`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Models Table` and `Initial Schema Migration`?**
  _Edge tagged AMBIGUOUS (relation: references) - confidence is low._
- **What is the exact relationship between `Pipeline Orchestrator` and `Pipeline State Enum`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
- **What is the exact relationship between `EngineNotImplemented (Postprocessor)` and `TranscriptEvent`?**
  _Edge tagged AMBIGUOUS (relation: conceptually_related_to) - confidence is low._
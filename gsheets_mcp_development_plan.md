# gsheets_mcp Development Plan

## 1. Overview
This plan details the phased, robust, and maximally complete development of the `gsheets_mcp` Rust MCP server for Google Sheets, following the architecture specification and all requirements in `AGENTS.md`.

## 2. Phases & Task Swarms

### Phase 1: Project Bootstrap & Core Setup
**Task Swarm 1.1: Project Structure & Boilerplate (Parallelizable)**
- [x] 1.1.1 Create all module files in `src/` (mcp_server, http_server, stdio_server, see_server, google_api_client, auth_manager, models, error, logging, config, tests)
- [x] 1.1.2 Initialize Cargo.toml with dependencies (axum, google-apis-rs, etc.)
- [x] 1.1.3 Implement basic test harness

**Task Swarm 1.2: Core Utilities (Parallelizable)**
- [x] 1.2.1 Implement logging module
- [x] 1.2.2 Implement error module
- [x] 1.2.3 Implement config/environment loader

**Join Task 1.3: Phase 1 Integration & Verification**
- [x] 1.3.1 Verify all modules compile and pass basic tests
- [x] 1.3.2 Update checklist and remediate any issues

### Phase 2: Authentication & Google API Client
**Task Swarm 2.1: Auth & API Client (Parallelizable)**
- [x] 2.1.1 Implement `auth_manager` for Service Account, OAuth2, ADC
- [x] 2.1.2 Integrate `google-apis-rs` for Sheets and Drive
- [x] 2.1.3 Implement `google_api_client` abstraction and models

**Task Swarm 2.2: Unit Testing (Parallelizable)**
- [ ] 2.2.1 Unit tests for authentication
- [ ] 2.2.2 Unit tests for API client

**Join Task 2.3: Phase 2 Integration & Verification**
- [x] 2.3.1 Verify all auth and API client features work and are tested (except spreadsheet rename, see open problems)
- [x] 2.3.2 Update checklist and remediate any issues

### Phase 3: MCP Server Transports
**Task Swarm 3.1: Transport Implementations (Parallelizable)**
- [ ] 3.1.1 Implement HTTP server (axum) with REST and streaming endpoints
- [ ] 3.1.2 Implement stdio server for CLI/pipe integration
- [ ] 3.1.3 Implement SEE server for event-driven/streaming clients

**Task Swarm 3.2: Unified Handler & Testing (Parallelizable)**
- [ ] 3.2.1 Implement transport-agnostic handler trait
- [ ] 3.2.2 End-to-end tests for all transports

**Join Task 3.3: Phase 3 Integration & Verification**
- [ ] 3.3.1 Verify all transports work and are tested
- [ ] 3.3.2 Update checklist and remediate any issues

### Phase 4: Sheets Operations & Endpoints
**Task Swarm 4.1: Endpoint Implementations (Parallelizable)**
- [ ] 4.1.1 Implement endpoints: list/create spreadsheets
- [ ] 4.1.2 Implement endpoints: get/set sheet data
- [ ] 4.1.3 Implement batch operations
- [ ] 4.1.4 Implement sharing, metadata, and Drive integration

**Task Swarm 4.2: Error Handling, Logging, and Integration Tests (Parallelizable)**
- [ ] 4.2.1 Robust error handling for all endpoints
- [ ] 4.2.2 Logging for all endpoints
- [ ] 4.2.3 Integration tests for all operations

**Join Task 4.3: Phase 4 Integration & Verification**
- [ ] 4.3.1 Verify all endpoints and operations work and are tested
- [ ] 4.3.2 Update checklist and remediate any issues

### Phase 5: Robustness, Security, and Extensibility
**Task Swarm 5.1: Security & Extensibility (Parallelizable)**
- [ ] 5.1.1 Security review: input validation, secrets handling, least-privilege
- [ ] 5.1.2 Add support for additional Google Workspace APIs (optional)
- [ ] 5.1.3 Add system and data flow diagrams to architecture spec

**Task Swarm 5.2: Documentation & CI/CD (Parallelizable)**
- [ ] 5.2.1 Documentation: usage, configuration, deployment, and API reference
- [ ] 5.2.2 CI/CD setup and build/test automation

**Join Task 5.3: Final Integration & Handoff**
- [ ] 5.3.1 Verify all robustness, security, and extensibility requirements are met
- [ ] 5.3.2 Final checklist update and remediation if needed

## 3. Task Swarm Structure
- Each phase is decomposed into parallelizable task swarms where possible (e.g., module implementations, tests, docs).
- Join tasks at the end of each phase verify completion and integration.
- Remediation protocol: Any incomplete or failed task triggers a remediation plan and checklist update.

## 4. Checklist Protocol
- Each task must update the development plan checklist with status: Complete, Partial, None (with notes).
- Join tasks confirm all subtasks are done before proceeding.

## 5. Acceptance Criteria
- All requirements in the architecture spec and AGENTS.md are met.
- All endpoints and transports are robustly implemented and tested.
- Security, error handling, and extensibility are fully addressed.
- Documentation and diagrams are complete and up to date.

## Open Problems / Unimplemented Features

- Spreadsheet rename (Drive API): Not supported in this version due to API limitations and trait bound issues. See open issues for details.

---

This plan will be iteratively updated as the project progresses. All contributors must follow the checklist and remediation protocol for every phase and task.

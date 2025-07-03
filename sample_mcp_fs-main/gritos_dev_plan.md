# gRiTOS Development Plan

## 1. Overview

### Purpose of gRiTOS
gRiTOS is a modern, real-time microkernel-design operating system optimized for functional safety, cybersecurity, and quality-managed configurations in embedded and safety-critical domains. It aims to provide a secure, reliable, and predictable platform for applications where system integrity and deterministic behavior are paramount.

### Target Domains
gRiTOS is targeted towards the following industries and use cases:
-   **Automotive:** Advanced Driver-Assistance Systems (ADAS), infotainment systems, powertrain control.
-   **Industrial Automation:** Robotic control, Programmable Logic Controllers (PLCs), critical infrastructure monitoring.
-   **Medical Devices:** Patient monitoring systems, infusion pumps, diagnostic equipment.
-   **Aerospace and Defense:** Flight control systems, unmanned aerial vehicles (UAVs), secure communication systems.
-   **Secure IoT Gateways:** Edge computing devices requiring real-time processing and strong security.

### References
This development plan is derived from and should be read in conjunction with:
-   gRiTOS Product Specification (`plan/archive/gritos_product_specification.md`) - (SoP)
-   gRiTOS Product Requirements Full (`gritos_product_requirements_full.md`) - (ReqFull)

## 2. Guiding Principles & Constraints

The design and development of gRiTOS are guided by a set of core philosophies, principles, and constraints:

### Core Philosophies (Derived from SoP 1.4.1)
-   **Security by Design:** Security is a fundamental aspect of the architecture, not an afterthought.
-   **Safety-First Approach:** Functional safety is paramount, ensuring predictability, fault tolerance, and verifiable correctness.
-   **Provable Correctness (where practical):** Aim for formal verification of key properties in critical components like the microkernel.
-   **Deterministic Real-Time Performance:** Provide bounded, predictable execution times for critical operations.

### Guiding Principles (Derived from SoP 1.4.2)
1.  **Microkernel Architecture:** Essential services in kernel; drivers and other services in user-space. (GRITOS-REQ-ARC-PRINCIPLE-MICROKERNEL-00001)
2.  **Capability-Based Security:** Access to all kernel objects and resources mediated by unforgeable capabilities. (GRITOS-REQ-ARC-PRINCIPLE-CAPABILITYSEC-00001)
3.  **Minimal Trusted Computing Base (TCB):** Kernel code kept small and simple for verifiability. (GRITOS-REQ-ARC-PRINCIPLE-MINIMALTCB-00001)
4.  **User-Space Services:** Majority of OS functionality as sandboxed user-space components. (GRITOS-REQ-ARC-PRINCIPLE-USERSPACESVC-00001)
5.  **Component-Based Design:** User-space systems as isolated components with explicit interfaces. (GRITOS-REQ-ARC-PRINCIPLE-COMPONENTDESIGN-00001)
6.  **Portability with Clean Abstractions:** Well-defined HAL for easier porting. (GRITOS-REQ-ARC-PRINCIPLE-PORTABILITY-00001)
7.  **Explicit Resource Management:** All system resources explicitly managed and allocated via capabilities. (GRITOS-REQ-ARC-PRINCIPLE-EXPLICITRSCMGMT-00001)

### Key Constraints (Derived from SoP 1.4.3)
1.  **No Dynamic Kernel Memory Allocation:** After initial boot setup. (GRITOS-REQ-CON-KERNEL-NODYNAMICMEM-00001)
2.  **Focus on Resource-Constrained Embedded Systems:** Efficiency and small footprint are key. (GRITOS-REQ-CON-TARGET-RESOURCECONSTRAINED-00001)
3.  **Designed for Certifiability:** Architectural choices and processes must support certification. (GRITOS-REQ-CON-PROCESS-CERTIFIABILITY-00001)
4.  **Bounded Execution Times:** All kernel operations must have provably bounded WCET. (GRITOS-REQ-CON-PERF-BOUNDEDEXEC-00001)
5.  **Formal Verification Target for Kernel Core:** Key security and safety properties of the microkernel are targets for formal verification. (GRITOS-REQ-CON-KERNEL-FORMALVERIFYTARGET-00001)

## 3. Core Modules & Development Sub-Plans

This section details the core modules of gRiTOS and the high-level development tasks associated with each. Each task should be traceable to specific requirements in the SoP and ReqFull documents.

### 3.1. Foundation & Bootloader
-   **Role:** Establishes the initial trusted hardware state, verifies and loads the kernel, and hands over control.
-   **Development Tasks:**
    *   Task: Design and implement ROM Bootloader (RBL) / Primary Bootloader (PBL) verification logic. (SoP 2.3.1, GRITOS-REQ-SEC-BOOT-SECURECHAIN-00001)
    *   Task: Develop Secondary Bootloader (SBL) with hardware initialization capabilities. (SoP 2.3.1, GRITOS-REQ-SEC-BOOT-SECURECHAIN-00001)
    *   Task: Implement cryptographic verification (hashing, signature check) in SBL for the kernel image. (SoP 2.3.1, GRITOS-REQ-SEC-BOOT-SECURECHAIN-00001)
    *   Task: Implement A/B partition update logic in SBL, including rollback capabilities. (SoP 2.3.6, GRITOS-REQ-SEC-UPDATE-SECUREMECHANISM-00001)
    *   Task: Define and implement boot information structure passed from SBL to kernel (memory map, DTB location, etc.). (SoP 2.3.1)
    *   Task: Implement anti-rollback protection for SBL and Kernel. (SoP 2.3.1, GRITOS-REQ-SEC-BOOT-SECURECHAIN-00001)
    *   Task: Integrate Hardware Root of Trust (HRoT) for secure key storage for bootloader verification. (SoP 2.3.2, GRITOS-REQ-SEC-HROT-INTEGRATION-00001)

### 3.2. Kernel Core
-   **Role:** Provides fundamental OS primitives: scheduling, IPC, basic memory management, and capability management. Manages kernel objects and enforces security policies.
-   **Development Tasks:**
    *   Task: Define and implement the minimal System Call Interface (target < 15 syscalls: `gRiTOS_IPC_Call`, `gRiTOS_IPC_Send`, `gRiTOS_IPC_Recv`, `gRiTOS_IPC_Reply`, `gRiTOS_KernelObject_Invoke`, `gRiTOS_Yield`, `gRiTOS_DebugPutchar`). (SoP 4.1.1, GRITOS-REQ-ARC-KERNEL-SYSCALLMIN-00001)
    *   Task: Implement core kernel objects: Capability, CNode, TCB, Endpoint, Notification, AddressSpace, UntypedMemory, IRQ Handler Object, Memory Object. (SoP 4.1.2, GRITOS-REQ-ARC-KERNEL-OBJECTS-00001)
    *   Task: Implement `gRiTOS_KernelObject_Invoke` methods for each kernel object type. (SoP 2.1.1, 2.1.2, 2.1.3, 2.1.4)
    *   Task: Implement static memory allocation for all kernel data structures; no dynamic kernel memory post-init. (SoP 1.4.3, GRITOS-REQ-CON-KERNEL-NODYNAMICMEM-00001, GRITOS-REQ-FNC-KERNEL-MEM-00003)
    *   Task: Implement preemptive, priority-based scheduler (Fixed-Priority Preemptive) with configurable priority levels, FIFO and Round-Robin for same-priority threads. (SoP 2.1.1, 4.1.3, GRITOS-REQ-FNC-KERNEL-SCHED-00001, GRITOS-REQ-ARC-KERNEL-SCHEDALGO-00001)
    *   Task: Implement support for time partitioning (scheduler budgets) for Mixed-Criticality Systems. (SoP 2.4.5, GRITOS-REQ-SAF-MCS-SUPPORT-00001)
    *   Task: Develop deterministic context switching mechanisms. (SoP 2.1.1, GRITOS-REQ-FNC-KERNEL-PROCESS-00002)
    *   Task: Implement TCB management: creation, configuration (associating VSpace, CSpace, fault handler EP), priority setting, state transitions (`TS_INACTIVE`, `TS_READY`, `TS_RUNNING`, etc.). (SoP 2.1.1, GRITOS-REQ-FNC-KERNEL-PROCESS-00001)
    *   Task: Implement process management: creation from UntypedMemory, configuration (associating root CNode, VSpace), starting initial thread. Process states (`PS_INACTIVE`, `PS_ACTIVE`, `PS_TERMINATING`). (SoP 2.1.1, GRITOS-REQ-FNC-KERNEL-PROCESS-00001)
    *   Task: Implement kernel interrupt handling: minimal initial handling, IRQ object management, delivery of interrupt events to user-space handlers via IPC/Notifications. (SoP 2.1.4, 4.1.4, GRITOS-REQ-FNC-KERNEL-IRQ-00001, GRITOS-REQ-FNC-KERNEL-IRQ-00002, GRITOS-REQ-ARC-KERNEL-IRQ-TWO-STAGE-00001)
    *   Task: Design and implement capability system: unforgeable capabilities, rights mask, type information, capability derivation. (SoP 2.3.3, GRITOS-REQ-SEC-POLP-CAPABILITY-00001, GRITOS-REQ-ARC-PRINCIPLE-CAPABILITYSEC-00001)
    *   Task: Formally verify key kernel properties (e.g., isolation, IPC correctness, scheduler integrity) if practical. (SoP 1.4.1, GRITOS-REQ-ARC-PHILOSOPHY-PROVABLECORR-00001)
    *   Task: Ensure all kernel operations have bounded WCET. (SoP 1.4.3, GRITOS-REQ-CON-PERF-BOUNDEDEXEC-00001)

### 3.3. IPC Subsystem
-   **Role:** Manages secure communication between isolated components (kernel-user, user-user).
-   **Development Tasks:**
    *   Task: Implement synchronous IPC (`gRiTOS_Call`, `gRiTOS_Reply`) including direct data transfer and blocking mechanisms. (SoP 2.1.3, GRITOS-REQ-FNC-KERNEL-IPC-00001)
    *   Task: Implement asynchronous IPC (`gRiTOS_SendAsync`, `gRiTOS_NotificationObject` signals/waits). (SoP 2.1.3, GRITOS-REQ-FNC-KERNEL-IPC-00002)
    *   Task: Define and implement IPC message structure (`msg_info`: tag, direct payload, capability slots). (SoP 2.1.3, 4.2, GRITOS-REQ-FNC-KERNEL-IPC-00003, GRITOS-REQ-ARC-IPC-MSGSTRUCTURE-00001)
    *   Task: Implement secure capability transfer via IPC, including kernel validation and CSpace management. (SoP 2.1.3, GRITOS-REQ-FNC-KERNEL-IPC-00003)
    *   Task: Optimize IPC pathways for low latency and bounded WCET. (SoP 2.1.3, GRITOS-REQ-NFR-PERF-IPC-00001)
    *   Task: Implement Endpoint kernel object logic for message queuing and thread synchronization. (SoP 4.1.2)
    *   Task: Implement Notification kernel object logic for lightweight signaling. (SoP 4.1.2)

### 3.4. Memory Management Subsystem
-   **Role:** Manages physical and virtual memory, enforces memory isolation using hardware (MMU/MPU), and provides mechanisms for memory sharing.
-   **Development Tasks:**
    *   Task: Implement MMU/MPU configuration and management for memory isolation (kernel/user, inter-component). (SoP 2.1.2, GRITOS-REQ-FNC-KERNEL-MEM-00001)
    *   Task: Implement `UntypedMemory` object and `Untyped_Retype` method for creating kernel objects and memory objects. (SoP 2.1.2, 4.1.2, 4.3, GRITOS-REQ-FNC-KERNEL-MEM-00002, GRITOS-REQ-ARC-MEM-CAPABILITYBASEDMODEL-00001)
    *   Task: Implement `gRiTOS_MemoryObject` (VMO-like) for representing physical memory regions. (SoP 2.1.2, 4.1.2, 4.3, GRITOS-REQ-FNC-KERNEL-MEM-00002)
    *   Task: Implement `AddressSpace` object and methods for mapping/unmapping `gRiTOS_MemoryObject` instances (`ADDRSPACE_METHOD_MAP_OBJECT`). (SoP 2.1.2, 4.1.2, 4.3, GRITOS-REQ-FNC-KERNEL-MEM-00002)
    *   Task: Implement shared memory by mapping the same `gRiTOS_MemoryObject` into multiple `AddressSpace` objects, controlled by capabilities. (SoP 2.1.2, GRITOS-REQ-FNC-KERNEL-MEM-00004)
    *   Task: Implement memory fault handling: trap identification, delivery of exception IPC to user-space handlers. (SoP 2.1.2, GRITOS-REQ-FNC-KERNEL-MEM-00005)
    *   Task: Ensure secure construction of exception IPC messages to avoid info leaks. (SoP 2.1.2, GRITOS-REQ-SEC-IPC-EXCEPTIONMSG-00001)

### 3.5. Component Framework (User-Space)
-   **Role:** Defines the structure, lifecycle, and management of user-space applications and services.
-   **Development Tasks:**
    *   Task: Define component manifest format (executable, required capabilities, provided services, resources, config). (SoP 4.4, GRITOS-REQ-ARC-USERSPACE-COMPONENTFW-00001)
    *   Task: Design and implement a Component Manager service for instantiating and managing components based on manifests. (SoP 4.4)
    *   Task: Define component lifecycle: instantiation, startup, running, termination, cleanup, resource reclamation. (SoP 4.4, GRITOS-REQ-ARC-USERSPACE-COMPONENTFW-00001)
    *   Task: Develop an Interface Definition Language (IDL) for specifying component interfaces and generating stubs/skeletons. (SoP 2.2, 4.4.5, GRITOS-REQ-NFR-MAINT-MODULARITY-00001)
    *   Task: Implement mechanisms for service discovery between components (e.g., via a naming service or capability passing). (SoP 4.4)

### 3.6. HAL & BSP (Hardware Abstraction Layer & Board Support Package)
-   **Role:** Abstracts hardware specifics, enabling portability. BSP provides drivers and configurations for specific boards.
-   **Development Tasks:**
    *   Task: Define HAL API for CPU architecture specifics (context switching, MMU/MPU control, atomics, clocks), core peripherals (timer, interrupt controller, console), and boot interface. (SoP 3.7.2, 4.6, GRITOS-REQ-NFR-PORT-HALBSP-00001, GRITOS-REQ-ARC-PORT-HALBSPSTRATEGY-00001)
    *   Task: Implement HAL for the primary reference architecture (e.g., ARMv8-A or RISC-V 64-bit). (SoP 3.7.1)
    *   Task: Define BSP structure: device drivers, hardware configuration (Devicetree or similar), clock/pinmux config, memory layout. (SoP 3.7.2, 4.6, GRITOS-REQ-NFR-PORT-HALBSP-00001)
    *   Task: Develop Devicetree parsing and utilization mechanisms for driver configuration. (SoP 4.6)
    *   Task: Create a BSP for the initial reference hardware platform, including essential device drivers. (SoP 4.6)

### 3.7. User-Space System Services
These are foundational services running as isolated components.

#### 3.7.1. Root Resource Manager
-   **Role:** Initial user-space process responsible for system composition, resource allocation, and capability delegation.
-   **Development Tasks:**
    *   Task: Implement Root Resource Manager component logic. (SoP 2.2.5, GRITOS-REQ-FNC-SVC-RSCMGMT-00001)
    *   Task: Implement IPC interfaces for creating components, service discovery (getting capabilities), allocating untyped memory, and creating IRQ handlers. (SoP 2.2.5, GRITOS-REQ-FNC-SVC-RSCMGMT-00001)

#### 3.7.2. Device Drivers (User-Space)
-   **Role:** Manage hardware peripherals, running as isolated components.
-   **Development Tasks:**
    *   Task: Develop a generic device driver model/framework for user-space drivers. (SoP 2.2.1, GRITOS-REQ-FNC-SVC-DRIVER-00001)
    *   Task: Implement specific user-space drivers for core peripherals (e.g., UART, GPIO, SPI, I2C) based on BSP needs, exposing functionality via standardized IPC. (SoP 2.2.1)
    *   Task: Implement user-space Ethernet driver and storage controller drivers. (SoP 2.2.1)

#### 3.7.3. File System Service
-   **Role:** Provides file management capabilities.
-   **Development Tasks:**
    *   Task: Design and implement a user-space file system server. (SoP 2.2.2, GRITOS-REQ-FNC-SVC-FS-00001)
    *   Task: Implement IPC interfaces for file operations (open, close, read, write, stat, mkdir, rmdir, unlink), using capabilities for paths, buffers, and file descriptors. (SoP 2.2.2, GRITOS-REQ-FNC-SVC-FS-00001)
    *   Task: Integrate with storage device drivers. (SoP 2.2.2)
    *   Task: Ensure reliability and data integrity, potentially crash-consistency. (SoP 2.2.2)

#### 3.7.4. Networking Stack Service
-   **Role:** Provides network communication capabilities (TCP/IP, UDP).
-   **Development Tasks:**
    *   Task: Design and implement a user-space networking stack component. (SoP 2.2.3, GRITOS-REQ-FNC-SVC-NET-00001)
    *   Task: Implement IPC interfaces for socket-like operations (socket, bind, listen, accept, connect, send, recv, close), using capabilities for data buffers. (SoP 2.2.3, GRITOS-REQ-FNC-SVC-NET-00001)
    *   Task: Integrate with user-space Ethernet driver. (SoP 2.2.3)
    *   Task: Consider integration of a dedicated firewall component support. (SoP 2.2.3, GRITOS-REQ-SEC-NET-FIREWALLSUPPORT-00001)

#### 3.7.5. Time Services
-   **Role:** Provides system time, monotonic clocks, and timer functionalities.
-   **Development Tasks:**
    *   Task: Implement a user-space time management service. (SoP 2.2.4, GRITOS-REQ-FNC-SVC-TIME-00001)
    *   Task: Implement IPC interfaces for getting current time, setting one-shot/periodic timers (using Notification objects), and canceling timers. (SoP 2.2.4, GRITOS-REQ-FNC-SVC-TIME-00001)
    *   Task: Integrate with hardware timers via user-space drivers or HAL. (SoP 2.2.4)

#### 3.7.6. System Monitoring & Logging Service
-   **Role:** Monitors system health, resource usage, and provides secure logging.
-   **Development Tasks:**
    *   Task: Implement a user-space logging service with IPC interface for submitting log messages (with severity, buffer capability). (SoP 2.2.6, GRITOS-REQ-FNC-SVC-LOG-00001)
    *   Task: Implement a user-space monitoring service with IPC interfaces for querying CPU/memory usage and subscribing to events. (SoP 2.2.6, GRITOS-REQ-FNC-SVC-LOG-00001)
    *   Task: Ensure secure and reliable audit trail. (SoP 2.2.6)

### 3.8. Security Services
-   **Role:** Provides core security functionalities beyond the kernel's capability system.
-   **Development Tasks:**
    *   Task: Implement Secure Over-the-Air (OTA) Update Manager component, including image verification, A/B partition management, and communication with SBL (status block). (SoP 2.3.6, GRITOS-REQ-SEC-UPDATE-SECUREMECHANISM-00001, GRITOS-REQ-SEC-UPDATE-OTA-00001)
    *   Task: Develop user-space driver and IPC interface for Hardware Root of Trust (HRoT) integration (secure key storage, crypto operations, TRNG). (SoP 2.3.2, GRITOS-REQ-SEC-HROT-INTEGRATION-00001)
    *   Task: Implement user-space Secure Storage service for confidential and integrity-protected data, potentially using HRoT. (SoP 2.2.7, GRITOS-REQ-FNC-SVC-SSTORE-00001)
    *   Task: Develop and maintain threat models for critical components and the system. (SoP 2.3.7, GRITOS-REQ-SEC-PROCESS-THREATMODEL-00001)
    *   Task: Implement cryptographic algorithm compliance (FIPS 140-2/3 or equivalent) for all security services. (SoP 3.4.2, GRITOS-REQ-NFR-SEC-CRYPTOCOMPLIANCE-00001)

### 3.9. Functional Safety Services
-   **Role:** Implements mechanisms crucial for achieving functional safety objectives.
-   **Development Tasks:**
    *   Task: Implement kernel mechanisms for fault containment and isolation (already covered in Kernel Core). (SoP 2.4.1, GRITOS-REQ-SAF-FAULT-CONTAINMENT-00001)
    *   Task: Design and implement user-space Health Monitoring components that utilize kernel error reporting and heartbeat mechanisms. (SoP 2.4.3, GRITOS-REQ-SAF-ERROR-DETECTION-00001)
    *   Task: Implement stack overflow detection support (e.g., guard pages if feasible, compiler options). (SoP 2.4.3)
    *   Task: Implement user-space watchdog management service interacting with hardware watchdogs. (SoP 2.4.3)
    *   Task: Develop fault recovery supervisor components capable of restarting failed user-space services and managing graceful degradation strategies. (SoP 2.4.4, GRITOS-REQ-SAF-FAULT-RECOVERY-00001)
    *   Task: Ensure support for Mixed-Criticality Systems (MCS) through strict resource partitioning (CPU, memory, I/O) configurable by a privileged component. (SoP 2.4.5, GRITOS-REQ-SAF-MCS-SUPPORT-00001)
    *   Task: Perform WCET analysis for all safety-critical kernel and user-space paths. (SoP 3.5.3, GRITOS-REQ-NFR-SAF-WCET-00001)

## 4. High-Level Development Phases

This section outlines the overall development trajectory, drawing inspiration from `concept/inputs/grit_dev_plan.md` but adapted for gRiTOS's specific nature.

### Phase 0: Foundation & Planning (Current Phase)
-   **Goal:** Define product vision, scope, requirements, and detailed development plan. Establish core architectural principles and constraints. Setup initial development environment and QMS outline.
-   **Key Modules/Tasks:**
    *   Product Specification (SoP) finalization.
    *   Product Requirements (ReqFull) extraction and review.
    *   This Development Plan creation.
    *   QMS and compliance strategy outline.
    *   Toolchain selection and initial setup.

### Phase 1: Core Kernel Boot and Minimal IPC
-   **Goal:** Boot a minimal kernel on the reference hardware/emulator. Implement basic kernel objects, memory management (static allocation, UntypedMemory retyping for kernel structures), scheduler, and a functional IPC mechanism for kernel-level communication.
-   **Key Modules/Tasks:**
    *   Foundation & Bootloader: SBL development for kernel loading (no verification yet).
    *   Kernel Core: `_start`, minimal console, panic handler, static memory allocators, basic TCB, CNode, Endpoint, AddressSpace, UntypedMemory objects. Basic scheduler (FIFO).
    *   IPC Subsystem: Implement `gRiTOS_IPC_Call` and `gRiTOS_KernelObject_Invoke` for initial kernel object interactions.
    *   HAL & BSP: Basic HAL for CPU init, timer, interrupt controller, console for reference platform.
    *   First kernel thread/task running.

### Phase 2: User-Space Transition & Basic Component Framework
-   **Goal:** Enable the kernel to start initial user-space components. Implement basic user-space memory management, capability passing to user-space, and the initial Component Manager.
-   **Key Modules/Tasks:**
    *   Kernel Core: Implement full TCB/Process creation for user-space. Implement capability delegation to user-space.
    *   Memory Management: Implement `gRiTOS_MemoryObject` creation from `UntypedMemory` by a user-space manager; implement mapping these objects into user-space `AddressSpace`.
    *   IPC Subsystem: Full IPC functionality (`SendAsync`, `NotificationObject`, capability transfer to/from user-space).
    *   Component Framework: Develop initial Component Manager (itself a user-space component). Define component manifest.
    *   User-Space System Services: Root Resource Manager started by kernel, takes over some boot tasks.
    *   First user-space component ("init" or test component) running.

### Phase 3: Essential System Services & Drivers
-   **Goal:** Develop core user-space system services: device drivers for essential peripherals, time service, basic logging/monitoring.
-   **Key Modules/Tasks:**
    *   HAL & BSP: Expand drivers for reference platform (e.g., GPIO, SPI/I2C).
    *   User-Space System Services:
        *   Device Drivers: Implement user-space drivers for selected peripherals.
        *   Time Services: Implement time management service.
        *   System Monitoring & Logging: Basic logging service.
    *   Security Services: Basic HRoT integration (e.g., TRNG access).
    *   Functional Safety Services: Basic health monitoring hooks.

### Phase 4: Advanced Services & Security Hardening
-   **Goal:** Implement more complex services like file systems and networking. Harden security features.
-   **Key Modules/Tasks:**
    *   User-Space System Services:
        *   File System Service: Implement a simple file system (e.g., RAMFS, basic FAT).
        *   Networking Stack Service: Implement basic networking (e.g., UDP/IP or TCP/IP subset).
    *   Security Services: Full Secure Boot Chain implementation and testing. OTA Update Manager development. Secure Storage service.
    *   Kernel Core: Formal verification efforts for selected critical parts (if pursued).
    *   Attack surface minimization review and hardening.

### Phase 5: Functional Safety Features & Mixed Criticality
-   **Goal:** Implement and test key functional safety mechanisms and robust Mixed-Criticality System support.
-   **Key Modules/Tasks:**
    *   Functional Safety Services: Implement fault recovery mechanisms (service restarts), watchdog management.
    *   Kernel Core & Scheduler: Implement and rigorously test time partitioning and resource segregation for MCS.
    *   Memory Management: Ensure robust spatial isolation for MCS.
    *   Testing: Extensive fault injection testing. WCET analysis for critical paths.

### Phase 6: Broader Hardware Support & Ecosystem Development
-   **Goal:** Port gRiTOS to additional architectures/boards. Develop APIs and documentation for application developers.
-   **Key Modules/Tasks:**
    *   HAL & BSP: Develop HAL for a second architecture. Develop new BSPs.
    *   API Development: Refine and document APIs for application development (native and POSIX-like if pursued).
    *   Ecosystem: Develop sample applications, tutorials.

### Phase 7: Certification & Long-Term Maintenance
-   **Goal:** Achieve target safety/security certifications. Establish LTS and maintenance processes.
-   **Key Modules/Tasks:**
    *   Compliance & Certification: Finalize Safety Case, Security Case. Engage with certification bodies.
    *   Maintenance: Establish LTS branches, patching processes, EOL policies.
    *   Performance optimization and stabilization.

## 5. Multi-Architecture Strategy (SoP 3.7.1, 4.6)

gRiTOS is designed for portability across multiple CPU architectures.
-   **HAL Design:** A minimal, well-defined Hardware Abstraction Layer (HAL) will isolate architecture-specific code (CPU operations, context switching, interrupt control, MMU/MPU management, core timers) from the generic kernel logic. The kernel will interact with the HAL via a C API.
-   **BSP Structure:** Board Support Packages (BSPs) will provide drivers for board-specific peripherals and hardware configurations. BSPs will utilize a hardware description mechanism (e.g., Devicetree) to configure drivers and resources dynamically. This allows the same driver code to be used across different boards if the peripheral is compatible.
-   **Conditional Compilation:** Rust's conditional compilation features (`#[cfg(target_arch = "...")` etc.) will be used extensively in the HAL and potentially in some drivers or low-level kernel code to manage architecture-specific implementations.
-   **Directory Structure:** Architecture-specific code will reside in `src/hal/<architecture_family>/<architecture_variant>/` (e.g., `src/hal/arm/armv8a/`) and `src/bsp/<board_name>/`.
-   **Initial Focus:** Development will initially target a primary reference architecture (e.g., ARMv8-A 64-bit or RISC-V 64-bit). Subsequent ports will leverage the clean HAL/kernel separation.

## 6. Build, Test, and Quality Assurance Strategy (SoP 5 & 7)

### Toolchain & Build System (SoP 5.2)
-   **Compilers:** Specific versions of GCC/Clang (for C parts of HAL/SBL) and Rust toolchain (for kernel and user-space components). Strict compiler flags for safety, security, and MISRA compliance (where applicable) will be enforced.
-   **Build System:** CMake/Meson for C components, Cargo for Rust. Kconfig-like system for feature configuration.
-   **Static Analysis:** Tools like `clang-tidy`, `clippy`, SonarQube, Frama-C (for C code), and potentially specialized Rust static analysis tools.
-   **Formal Verification:** Tools like Isabelle/HOL or TLA+ may be used for verifying critical kernel properties.
-   **Real-Time Analysis:** WCET analysis tools (e.g., AbsInt aiT, Rapita RVS) and schedulability analysis tools.

### Testing Strategy (SoP 5.3)
-   **Unit Testing:** Target 100% code coverage (statement/branch, MC/DC for critical parts) for kernel components and critical services. Frameworks like CMocka (for C) and Rust's built-in test framework.
-   **Integration Testing:** Verify inter-component communication (IPC) and service interactions using test harnesses and simulated environments.
-   **System Testing:** End-to-end functional, performance, safety, and security testing on target hardware or high-fidelity emulators (QEMU).
-   **Regression Testing:** Automated test suites integrated into CI to prevent regressions.
-   **Fault Injection Testing:** SWIFI and potentially HWIFI to test fault tolerance and recovery mechanisms.
-   **Security Testing:** Fuzzing of IPC interfaces and parsers, penetration testing, vulnerability scanning.
-   **Real-Time Performance Testing:** Stress tests, latency/jitter measurements under various loads.

### Continuous Integration/Continuous Deployment (CI/CD) (SoP 5.4)
-   A CI/CD pipeline (e.g., Jenkins, GitLab CI) will automate builds, static analysis, unit/integration tests for every commit.
-   On-target testing will be integrated for key reference platforms.
-   Automated documentation generation and release packaging.

### Quality Management System (QMS) (SoP 7)
-   Development will follow a QMS compliant with relevant standards (e.g., ISO 9001, aspects of IEC 61508/ISO 26262).
-   **Processes:** Documented processes for requirements management, design control, configuration management, change control, defect management, and audits.
-   **Traceability:** Rigorous traceability from requirements to design, code, tests, and verification results using appropriate tools.
-   **Metrics:** Key quality metrics (code coverage, defect density, test pass rates) will be tracked and reported.

## 7. Information Exchange Protocol

This protocol defines the structure for commit messages to ensure clarity, traceability, and effective information exchange between developers, particularly when handing off tasks or for asynchronous collaboration.

### Purpose
To provide a standardized format for Git commit messages that captures essential information about the changes, their context, and impact, facilitating better understanding, review, and project management.

### Format
Each commit message will consist of several fields, followed by a colon and the relevant information.

```
Task-ID: <JIRA_ID, GitHub_Issue_ID, or N/A>
Sub-Task-ID: <Internal sub-task identifier or N/A>
Status: <Not-Started | In-Progress | Blocked | In-Review | Completed | Tested>
Depends-On: <Task-ID or Commit_SHA if dependent on other unmerged work>
Files-Modified:
  - path/to/modified_file_1.ext: Brief description of change
  - path/to/another_file.ext: Brief description of change
  - ...
Summary: <A concise summary of the commit's purpose (max 72 chars)>

Notes-To-Next-Jules:
<Detailed explanation of the changes, design decisions, rationale,
assumptions made, and any specific instructions or points of attention
for the next developer or reviewer. This section can be multi-line.
Include information about:
- What was implemented and why.
- How it was implemented (briefly).
- Any challenges encountered.
- Parts that are incomplete or need further work.
- Specific testing done or further testing required.
- Links to relevant documentation or specifications.
- Potential impacts on other modules or functionalities.
- Any questions or areas needing clarification from the next person.
>
```

### Field Descriptions and Examples

-   **`Task-ID:`**
    -   **Description:** The primary identifier for the task or issue this commit relates to. This could be an ID from a project management tool like JIRA, a GitHub Issue number, or "N/A" if not directly tied to a tracked task (e.g., minor refactoring, documentation typo).
    -   **Example:** `Task-ID: GRITOS-DEV-123`

-   **`Sub-Task-ID:`**
    -   **Description:** An internal identifier for a more granular sub-task, if applicable. Useful for breaking down larger tasks. "N/A" if not used.
    -   **Example:** `Sub-Task-ID: KERNEL-IPC-IMPL-001`

-   **`Status:`**
    -   **Description:** The current status of the *task* or *sub-task* this commit contributes to. This helps in understanding if the commit completes the task or is an intermediate step.
        -   `Not-Started`: Task definition, planning commit.
        -   `In-Progress`: Partial implementation.
        -   `Blocked`: Work cannot proceed due to external factors.
        -   `In-Review`: Implementation complete, awaiting review.
        -   `Completed`: Implementation complete and reviewed (for this part of the task).
        -   `Tested`: Implementation complete, reviewed, and unit/integration tested.
    -   **Example:** `Status: In-Progress`

-   **`Depends-On:`**
    -   **Description:** If this commit depends on another task or a specific previous commit that might not yet be merged into the main development branch. Use Task-ID or the full Commit SHA. "N/A" if no such dependencies.
    -   **Example:** `Depends-On: GRITOS-DEV-120` or `Depends-On: a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0`

-   **`Files-Modified:`**
    -   **Description:** A list of key files modified in this commit, each followed by a very brief note on what changed in that file. This provides a quick overview of the commit's scope at a file level. For a large number of files, list the most significant ones or use directory paths with wildcards.
    -   **Example:**
        ```
        Files-Modified:
          - src/kernel/ipc.rs: Implemented synchronous send logic.
          - include/kernel/ipc.h: Added new IPC message type.
          - plan/gritos_dev_plan.md: Updated IPC development tasks.
        ```

-   **`Summary:`**
    -   **Description:** A short, imperative mood summary of the commit's main purpose. Think of it as the subject line of an email. It should be concise (ideally under 50 characters, max 72) and clearly state what the commit does.
    -   **Example:** `Summary: Implement basic IPC send for kernel messages`

-   **`Notes-To-Next-Jules:`**
    -   **Description:** This is the body of the commit message. It provides a detailed explanation of the changes. It should be comprehensive enough for another developer to understand the context, rationale, and implications of the commit. Use this section to elaborate on design choices, trade-offs, unresolved issues, or specific areas that need attention from reviewers or future developers.
    -   **Example:**
        ```
        Notes-To-Next-Jules:
        This commit introduces the initial implementation for synchronous IPC send
        operations within the kernel (`gRiTOS_IPC_Send_KernelMode`). It currently
        handles message queueing and basic TCB state changes.

        Key decisions:
        - Used a singly linked list for the endpoint send queue for now due to
          simplicity. May need to revisit for performance with many blocked senders.
        - Capability transfer is NOT yet implemented in this commit; only data payload.

        Further work needed:
        - Integration with the scheduler to wake up receiver tasks.
        - Full implementation of capability transfer logic.
        - Error handling for full queues or invalid endpoint capabilities.

        Testing done:
        - Basic unit tests for queueing and TCB state transition.
        - No integration tests yet.

        Please review the queue management logic in `src/kernel/ipc/endpoint.rs`
        carefully for potential race conditions, although spinlocks are used.
        ```

## 8. Future Considerations / Evolution (SoP 8 & 10)

-   **Long-Term Vision:** Evolve gRiTOS into a highly secure, reliable, and performant platform for a wide range of embedded and safety-critical applications. This includes exploring advanced security features, broader hardware support (including more complex SoCs), and potentially features like virtualization/hypervisor support (SoP 10.3) if market demands dictate.
-   **Ecosystem Growth:** Foster a developer community around gRiTOS, potentially by open-sourcing core components (SoP 10.5). Encourage third-party development of drivers, services, and applications.
-   **Standards Compliance:** Continuously monitor and adapt to new versions of relevant safety (IEC 61508, ISO 26262) and security (Common Criteria, FIPS) standards.
-   **Advanced Features:**
    *   Full POSIX compatibility layer (SoP 10.1).
    *   Advanced networking features (e.g., more protocols, TSN support).
    *   More sophisticated file systems.
    *   Dynamic loading of components/drivers.
-   **Performance Enhancements:** Ongoing optimization of kernel paths, IPC, and resource management.
-   **Tooling:** Continuous improvement of debugging, tracing, and analysis tools.
```

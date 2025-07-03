# grx Product Specification

# 1. High-Level Product Vision and Scope

## 1.1. Product Statement
grx is a modern, real-time microkernel-design operating system optimized for functional safety, cybersecurity, and quality-managed configurations in embedded and safety-critical domains.

This product is developed according to the [grx Development Plan](./grx_development_plan.md).

## 1.2. Target Markets and Applications
grx is targeted towards the following industries and use cases:
- **Automotive:** Advanced Driver-Assistance Systems (ADAS), infotainment systems, powertrain control.
- **Industrial Automation:** Robotic control, Programmable Logic Controllers (PLCs), critical infrastructure monitoring.
- **Medical Devices:** Patient monitoring systems, infusion pumps, diagnostic equipment.
- **Aerospace and Defense:** Flight control systems, unmanned aerial vehicles (UAVs), secure communication systems.
- **Secure IoT Gateways:** Edge computing devices requiring real-time processing and strong security.

## 1.3. Key Differentiators
grx offers a unique combination of features that differentiate it from other operating systems:
- **Microkernel Architecture for Enhanced Security and Safety:** By minimizing the Trusted Computing Base (TCB), grx significantly reduces the attack surface and simplifies verification efforts. Faults in user-space components are isolated and cannot compromise the kernel or other critical parts of the system. This approach, inspired by seL4's rigorous TCB minimization and Redox OS's robust user-space driver model, localizes faults and simplifies verification.
        - **Cybersecurity Metric:** TCB size (target < 100 KB ROM, 10k-15k LoC for core kernel) is a primary cybersecurity metric, directly impacting the attack surface.
        - **Cybersecurity Metric:** Number of formally verified security properties in the kernel (e.g., data integrity, control-flow integrity, isolation) will be tracked.
        - **Safety Metric:** Number of safety-critical kernel components/properties covered by formal verification (Target: Define scope based on safety goals, e.g., isolation, scheduler integrity).
        - **Safety Note:** The fault isolation provided by the microkernel is a foundational element in arguing for freedom from interference between components of different SIL/ASILs, crucial for a composable safety case.
- **Hard Real-Time Guarantees:** grx is designed from the ground up to provide deterministic scheduling and bounded latencies for all critical operations, ensuring that deadlines are met even under peak load. This is achieved through a design that emphasizes predictable performance, drawing lessons from RTOS like FreeRTOS and Zephyr, and the deterministic nature of microkernels like seL4.
        - **Safety Note:** Deterministic performance and bounded latencies are prerequisites for predictable system response and are essential for safety functions that have strict timing requirements (e.g., to meet Fault Tolerant Time Intervals - FTTI).
- **Focus on Certifiability:** The architecture and development process are geared towards achieving compliance with stringent functional safety (e.g., IEC 61508, ISO 26262) and cybersecurity (e.g., Common Criteria) standards.
        - **Safety Metric:** Maintain a list of target SIL/ASIL levels for core grx components (e.g., Kernel: ASIL D target, specific drivers: ASIL B/C based on system context). (Reinforces content in 3.5.1)
        - **Safety Note:** Certifiability necessitates adherence to rigorous safety standards (e.g., IEC 61508, ISO 26262), encompassing comprehensive hazard analysis, safety requirements engineering, robust V&V, and a meticulously constructed safety case.
- **Quality Managed Configurations:** grx development adheres to rigorous quality management processes, ensuring traceability, comprehensive documentation, and robust verification and validation.
- **Secure Over-the-Air (OTA) Updates:** Built-in mechanisms for atomic, cryptographically verified OTA updates with rollback capabilities ensure long-term maintainability and security in the field.
        - **Cybersecurity Metric:** Time to complete an OTA update cycle (download, verification, install, reboot) - target to be defined based on availability and security exposure window requirements.
        - **Cybersecurity Metric:** OTA rollback success rate for verified failures of a new image (Target: >=99.9%).
        - **Cybersecurity Metric:** Size of update packages (evaluate for efficiency and minimizing attack surface during transit/storage).
        - **Cybersecurity Note:** The security of the entire OTA infrastructure (update server, package signing, key management) is paramount.
        - **Cybersecurity Warning:** The OTA mechanism itself (e.g., Update Manager component, SBL update logic) must be hardened against vulnerabilities.
        - **Cybersecurity Consideration:** Implement a defined process for revoking and replacing compromised OTA signing keys.
- **Component-Based Design:** User-space services and applications are built as isolated components, facilitating modularity, independent updates, and fault containment. This modular structure, similar to the principles found in frameworks like Genode and Zircon's component model, facilitates independent updates and enhances fault containment.
- **Capability-Based Security:** Fine-grained access control through unforgeable capabilities enforces the principle of least privilege throughout the system. This model, drawing from the robust security architectures of seL4 and Zircon, provides unforgeable, fine-grained control over all system resources, enforcing the Principle of Least Privilege (PoLP) by default.
        - **Cybersecurity Metric:** Assess granularity of capability rights (distinct operations per object type).
        - **Cybersecurity Metric:** Percentage of kernel objects exclusively managed by capabilities (Target: 100%).
        - **Cybersecurity Warning:** Misconfiguration or overly broad capabilities granted to components can undermine security. Rigorous capability derivation and management by trusted user-space components (e.g., root resource manager, component manager) is critical.
        - **Cybersecurity Consideration:** Develop and utilize tools for auditing and visualizing capability distribution in a running system to detect potential over-privilege and assist in security analysis.

## 1.4. Architectural Foundations and Constraints

The design and development of grx are guided by a set of core philosophies, principles, and constraints that ensure its suitability for high-assurance, real-time embedded systems. These are derived from the detailed analysis in the '[Operating System Kernel Analysis for grx Architecture](../research/operating_system_kernel_analysis_for_grx_architecture.md)' document.

These foundations are derived from detailed analyses found in [Operating System Kernel Analysis for grx Architecture](../research/operating_system_kernel_analysis_for_grx_architecture.md) and [Kernel Boot Sequence Comparison](../research/kernel_boot_sequence_comparison.md).

### 1.4.1. Core Philosophy
-   **Security by Design:** Security is a fundamental aspect of the architecture, not an afterthought. The system aims to minimize attack surfaces and enforce the principle of least privilege rigorously.
-   **Safety-First Approach:** Functional safety is paramount. The OS must provide mechanisms to build safety-critical systems, ensuring predictability, fault tolerance, and verifiable correctness where feasible.
        - **Safety Note:** This philosophy must drive all development activities, from requirements and design to implementation, verification, and validation. Safety considerations are integral, not supplemental.
-   **Provable Correctness (where practical):** For the most critical components, particularly the microkernel itself, the design should facilitate and aim for formal verification of key properties (e.g., isolation, core scheduler behavior), drawing inspiration from systems like seL4.
        - **Safety Note:** Applying formal verification to key kernel safety properties (e.g., spatial and temporal isolation, integrity of scheduling, IPC correctness) provides the highest level of assurance against systematic faults in the verified code, forming a strong basis for the safety argument.
-   **Deterministic Real-Time Performance:** The system must provide bounded, predictable execution times for critical operations, suitable for hard real-time applications.

### 1.4.2. Guiding Principles
1.  **Microkernel Architecture:** grx is fundamentally a microkernel. Only essential services (IPC, scheduling, basic memory management, capability management) reside in privileged kernel mode. All other services (drivers, file systems, network stacks) operate as isolated user-space components.
2.  **Capability-Based Security:** Access to all kernel objects and resources (memory, IPC channels, hardware) is mediated by unforgeable capabilities. This enforces fine-grained access control and the Principle of Least Privilege.
3.  **Minimal Trusted Computing Base (TCB):** The kernel code and its associated privileged components will be kept as small and simple as possible to reduce attack surface and facilitate verification (including formal verification).
4.  **User-Space Services:** The majority of OS functionality is implemented as sandboxed user-space components communicating via secure IPC. This enhances modularity, fault isolation, and maintainability.
5.  **Component-Based Design:** User-space systems are built as a collection of clearly defined, isolated components with explicit interfaces, promoting modularity and structured composition.
6.  **Portability with Clean Abstractions:** A well-defined Hardware Abstraction Layer (HAL) will abstract hardware specifics, enabling easier porting to different architectures while maintaining a clean separation from the core kernel logic.
7.  **Explicit Resource Management:** All system resources (CPU time, memory, I/O) are explicitly managed and allocated, typically via capability delegation from parent components or dedicated resource managers.

### 1.4.3. Key Constraints
1.  **No Dynamic Kernel Memory Allocation:** The grx kernel must not use dynamic memory allocation (e.g., `malloc`/`free` via a kernel heap) after its initial boot setup. All kernel memory for objects and data structures must be statically pre-allocated or managed via fixed-size pools defined at compile-time/boot-time. This is critical for determinism and reducing kernel complexity/vulnerabilities.
        - **Cybersecurity Note:** This constraint directly contributes to cybersecurity by eliminating entire classes of kernel heap-related vulnerabilities (e.g., overflows, use-after-free).
        - **Safety Note:** This architectural constraint is critical for functional safety as it enhances predictability, simplifies Worst-Case Execution Time (WCET) analysis, and eliminates risks associated with dynamic memory allocation failures or heap corruption in the kernel.
2.  **Focus on Resource-Constrained Embedded Systems:** While scalable, the primary target is embedded systems where resources (CPU, RAM, ROM) are limited. Efficiency and a small footprint are key.
3.  **Designed for Certifiability:** Architectural choices and development processes must support and simplify eventual certification against relevant functional safety (e.g., IEC 61508, ISO 26262) and cybersecurity (e.g., Common Criteria) standards.
4.  **Bounded Execution Times:** All kernel operations (system calls, interrupt handling) must have provably bounded worst-case execution times (WCET).
        - **Safety Note:** Demonstrating bounded WCET for all kernel operations and safety-relevant user-space functions is essential for schedulability analysis and ensuring that safety-critical tasks meet their deadlines, even under worst-case load conditions.
5.  **Formal Verification Target for Kernel Core:** Key security and safety properties of the microkernel core are intended targets for formal verification to achieve the highest levels of assurance.
        - **Cybersecurity Note:** Formal verification is intended to prove the absence of specified classes of vulnerabilities in the verified kernel code, providing a high degree of assurance for core security properties.

# 2. Functional Requirements
## 2.1. Kernel Core Services
### 2.1.1. Process/Thread Management
- Support for isolated processes (protection domains) with their own virtual address spaces. Process creation involves creating and configuring the necessary kernel objects. This is typically managed by a parent component or a dedicated process server. Key operations would include:
        - `grx_KernelObject_Invoke(untyped_memory_cap, KOBJ_METHOD_RETYPE, object_type=Process, ...)`: To create a Process object.
        **Algorithm/Steps:**
        1. Kernel validates `untyped_memory_cap` and requested size for Process object, root CNode, and initial AddressSpace structures.
        2. Allocates memory from the `UntypedMemory` region for these kernel structures.
        3. Initializes the Process object, including a unique Process ID.
        4. Initializes the root CNode for the process (e.g., with a fixed initial size).
        5. Initializes the AddressSpace object (e.g., creates an empty top-level page table).
        6. Creates a capability to the new Process object and returns it to the caller (e.g., in a specified CNode slot).
        - `grx_KernelObject_Invoke(process_cap, PROC_METHOD_CONFIGURE, root_cnode_cap, vspace_cap, ...)`: To associate the fundamental capabilities with the new process object.
        - `grx_KernelObject_Invoke(process_cap, PROC_METHOD_START_THREAD, entry_point, stack_pointer, priority, initial_args_cap, ...)`: To create and start the initial thread within the process. (This combines thread creation and start for the initial thread).
        Process states will be precisely defined as: `PS_INACTIVE` (created but no threads), `PS_ACTIVE` (one or more threads exist), `PS_TERMINATING` (in the process of being shut down). Threads will transition through the following precise states:
        - `TS_INACTIVE`: Thread object exists but is not yet runnable (e.g., TCB created but not configured/started).
        - `TS_READY`: Thread is ready to run and waiting for CPU time.
        - `TS_RUNNING`: Thread is currently executing on a CPU.
        - `TS_BLOCKED_IPC_SEND`: Thread is blocked waiting for an IPC send operation to complete (e.g., endpoint full or waiting for receiver in rendezvous).
        - `TS_BLOCKED_IPC_RECV`: Thread is blocked waiting for an IPC receive operation (e.g., `grx_IPC_Call` waiting for reply, or `grx_IPC_Recv` on an empty endpoint).
        - `TS_BLOCKED_NOTIFICATION`: Thread is blocked waiting on a `grx_NotificationObject`.
        - `TS_BLOCKED_SLEEP`: Thread is voluntarily sleeping for a specified duration.
        - `TS_TERMINATED`: Thread has finished execution or has been aborted.
- Preemptive, priority-based scheduling for threads, with configurable priority levels (e.g., 256 levels). Threads will have fixed priorities within a defined range (e.g., 0-255, where a lower number indicates a higher priority). The scheduler will ensure that the highest priority ready thread is always running. Threads at the same priority level will be scheduled using FIFO and Round-Robin (with time slice duration configurable per thread via `TCB_METHOD_SET_TIMESLICE`) policies.
        **Ready Queue Data Structure:** The scheduler will maintain an array of ready queues, one for each priority level. For example: `ready_queues[NUM_PRIORITIES]`. Each element `ready_queues[i]` will be a doubly linked list of TCBs that are in the `TS_READY` state at priority `i`.
        - **FIFO:** When a thread becomes `TS_READY`, it's added to the tail of the linked list for its priority. The scheduler selects the thread at the head of the highest non-empty priority list.
        - **Round-Robin:** Similar to FIFO, but when a running thread's time slice expires, if it's still `TS_READY`, it's moved to the tail of its priority's linked list.
- Support for real-time scheduling policies (e.g., Rate-Monotonic, Earliest Deadline First) for critical threads. The specific APIs for configuring these advanced scheduling policies, such as `grx_ThreadSetSchedulerPolicy()`, will be clearly defined, taking cues from Zephyr's `k_thread_priority_set()` and advanced scheduling options.
- Deterministic context switching and interrupt handling. Context switching times will be minimized and bounded. Thread control operations will be invoked via `grx_KernelObject_Invoke()` on a TCB capability. Key methods include:
        - `TCB_METHOD_CONFIGURE(vspace_cap, cspace_cap, fault_handler_ep_cap, ...)`: To configure a newly created TCB.
        - `TCB_METHOD_SET_PRIORITY(priority)`: To set the thread's priority.
        - `TCB_METHOD_SET_TIMESLICE(timeslice_ticks)`: To configure its Round-Robin time slice duration.
        - `TCB_METHOD_START(entry_point, stack_pointer, arg1, arg2)`: To start execution of a configured thread.
        - `TCB_METHOD_ABORT()`: To terminate the thread.
        - `TCB_METHOD_YIELD()`: (This would be `grx_Yield()` syscall directly).
        - `TCB_METHOD_SLEEP(duration_ticks)`: To put the thread to sleep.
        The `grx_ThreadExit()` functionality is implicit when a thread's main function returns or explicitly calls a user-space exit routine which then invokes appropriate kernel cleanup via its parent/process server.
### 2.1.2. Memory Management
- MMU/MPU-based memory isolation between kernel and user space, and between user-space components. Standard page sizes (e.g., 4KB) will be supported, with potential for larger page sizes (e.g., 2MB, 1GB) if the hardware architecture permits and it's beneficial for specific use cases. The kernel will be responsible for managing page tables and ensuring strict isolation.
        - **Cybersecurity Metric:** Number of successful inter-component/kernel-userspace isolation penetration tests (Target: 100% pass against defined threats).
        - **Cybersecurity Warning:** Incorrect MMU/MPU configuration by the kernel is a critical vulnerability. The code responsible for page table management and context switching must be rigorously verified.
- Capability-based memory management (e.g., seL4\_Untyped\_Retype, zx\_vmo\_create) for fine-grained control and explicit resource allocation. grx will adopt a memory management model inspired by Zircon's Virtual Memory Objects (VMOs) and seL4's untyped memory. User-space components will request memory resources from a system MemoryManager. This manager will utilize kernel primitives such as `grx_Untyped_Retype()` (conceptually similar to `seL4_Untyped_Retype`) to allocate memory from pre-defined regions of untyped memory, or create VMO-like objects via calls like `grx_VMO_Create()`. A `grx_MemoryObject` (VMO) would be characterized by:
        - **Size:** The total size of the memory region in bytes, typically a multiple of the page size.
        - **Physical Backing:** Information about the physical pages backing this VMO, if any. VMOs could be initially unbacked (representing a reservation of virtual address space) and populated on demand, or directly backed by physical pages from `UntypedMemory`.
        - **Reference Count:** For managing the lifecycle of the VMO, especially when shared.
        - **Caching Policy:** Attributes defining cacheability (e.g., write-back, write-through, uncached) for architectures that support this, configurable via a method on the VMO capability.
These `grx_MemoryObject` instances (or portions thereof) can be mapped into a process's virtual address space. This would be achieved by invoking `grx_KernelObject_Invoke` on an `AddressSpace` capability with a method like `ADDRSPACE_METHOD_MAP_OBJECT`. The parameters for this method would include:
        - `object_capability`: The capability to the `grx_MemoryObject` to be mapped.
        - `object_offset`: The offset within the `grx_MemoryObject` from where mapping should begin (must be page-aligned).
        - `map_length`: The length of the region within the `grx_MemoryObject` to map (must be a multiple of page size).
        - `target_vaddr`: The desired starting virtual address in the target `AddressSpace`. The kernel can also be allowed to choose an available address within a specified range.
        - `permissions`: A bitmask specifying access rights for the mapping (e.g., `PERM_READ`, `PERM_WRITE`, `PERM_EXECUTE`).
        - `caching_policy_override` (optional): Allow overriding the VMO's default caching policy for this specific mapping, if hardware supports and it's a privileged operation.
        **Algorithm/Steps for `ADDRSPACE_METHOD_MAP_OBJECT`:**
        1. Kernel validates the `AddressSpace` capability and the `object_capability` (ensuring it's a mappable `grx_MemoryObject` and the caller has sufficient rights, e.g., `RIGHT_MAP`).
        2. Validates parameters: `object_offset` and `map_length` must be page-aligned and within the bounds of the `grx_MemoryObject`. `permissions` must be valid.
        3. If `target_vaddr` is specified by the user, checks if the virtual address range is free within the target `AddressSpace` and meets alignment requirements. If not specified, the kernel selects a suitable free range (e.g., using a VMAR-like region tracking).
        4. For each page in the `map_length`:
            a. Determines the corresponding physical page address from the `grx_MemoryObject` and `object_offset`.
            b. Updates the page table entries within the target `AddressSpace` to map the virtual page to the physical page with the specified `permissions` and `caching_policy_override`. This includes setting present, permissions, and cache-related bits in the page table entry.
        5. Performs necessary TLB (Translation Lookaside Buffer) invalidation/shootdown operations for the affected virtual address range to ensure consistency across cores if applicable (though initial focus is single-core, design should consider SMP).
        6. Returns success or an error code.
        All such operations are gated by capability checks: the caller must have the appropriate rights on both the `AddressSpace` capability and the `grx_MemoryObject` capability.
- **No dynamic kernel memory allocation:** All kernel memory must be statically allocated at boot time. This means all internal kernel data structures, including thread control blocks, capability storage, scheduling contexts, and page tables (or structures managing them), must be sized and allocated from a pre-reserved static memory region defined at compile-time or initialized during the early boot phase. This eliminates runtime heap operations within the kernel, contributing to determinism and reducing potential vulnerabilities.
- Support for shared memory regions between authorized components via explicit IPC/capabilities. Shared memory will be implemented by mapping the same memory object (e.g., a `grx_VMO` or a memory region derived from untyped memory) into the virtual address spaces of multiple cooperating processes. The capability to map and the access rights (read-only, read-write) for this shared region will be explicitly granted and managed through the capability system, ensuring controlled and secure sharing.
**Memory Fault Handling:**
        If a thread attempts an invalid memory access (e.g., accessing an unmapped region, a write to a read-only page, or a privilege violation detected by MPU/MMU), the hardware will trap into the kernel. The kernel will:
        1. Identify the faulting thread and the fault details (faulting address, access type).
        2. Determine if a fault handler (an IPC endpoint capability) is registered for the faulting thread's process (e.g., via `TCB_METHOD_SET_FAULT_HANDLER_EP`).
        3. If a handler is registered, the kernel will construct an 'exception IPC message' and send it to the registered fault handler endpoint. The `message_info` payload of this IPC message will be structured as follows:
        - `tag`: `SYS_EXCEPTION_TAG` (a predefined system tag).
        - `data_words[0]`: `EXCEPTION_TYPE_MEMORY_FAULT` (a specific code for memory faults).
        - `data_words[1]`: Faulting virtual address.
        - `data_words[2]`: Program Counter (PC) at the time of the fault.
        - `data_words[3]`: Stack Pointer (SP) at the time of the fault.
        - `data_words[4]`: Fault status register value (architecture-specific, indicating cause like page not present, permission error).
        - `data_words[5]`: Access type flags (e.g., `FAULT_ACCESS_READ`, `FAULT_ACCESS_WRITE`, `FAULT_ACCESS_EXECUTE`).
        - `cap_slots`: May contain a capability to the faulting TCB to allow the handler to inspect/manage it.
        The faulting thread will be suspended pending the outcome of the fault handling.
        4. The user-space fault handler (typically part of the process itself or a designated debugger/monitor component) can then inspect the fault, potentially resolve it (e.g., copy-on-write, lazy page allocation by mapping a VMO page), terminate the thread/process, or perform other corrective actions.
        5. If no handler is registered, or if the handler fails to resolve the fault and the thread is resumed, the kernel will typically terminate the faulting process to maintain system integrity.
        - **Cybersecurity Note:** Exception IPC messages must be carefully constructed to avoid leaking sensitive information (e.g., kernel addresses, precise fault reasons that could aid an attacker).
        - **Cybersecurity Consideration:** Implement rate limiting or other mitigations for fault handlers to prevent denial-of-service if a component generates excessive faults. Faults should be securely logged.
### 2.1.3. Inter-Process Communication (IPC)
- Synchronous and asynchronous message passing mechanisms (e.g., seL4\_Call, zx\_channel\_write). The primary IPC mechanism in grx will be a synchronous, blocking send-receive operation, invoked via a kernel call like `grx_Call(endpoint_capability, message_info_send, &message_info_recv)`. The `endpoint_capability` must be a capability to an `Endpoint` object. The kernel uses this capability directly to identify the communication channel and the listening thread(s). `message_info_send` is the payload sent by the client, and `message_info_recv` will be populated with the server's reply upon successful completion. This is conceptually similar to `seL4_Call` or sending a message on a Zircon channel and waiting for a reply. This call will allow a client thread to send a message to a server thread and block until the server thread replies.
        **Algorithm/Steps for `grx_Call` (Simplified):**
        **Sending Phase:**
        1. Kernel validates `endpoint_capability` (must be an Endpoint cap with `RIGHT_SEND`).
        2. If the endpoint has a waiting receiver thread (in `TS_BLOCKED_IPC_RECV` state on this endpoint):
            a. Directly transfer `message_info_send` (data words and capabilities) from sender's registers/TCB to receiver's registers/TCB. This involves kernel copying data words and performing capability transfer for any capabilities in `cap_slots`.
            b. Make receiver thread `TS_READY`.
            c. Place sender thread in `TS_BLOCKED_IPC_RECV` state, now waiting for a reply on a temporary reply capability/object implicitly created by the kernel for this call.
            d. Context switch if the receiver is now the highest priority runnable thread.
        3. Else (no receiver waiting):
            a. Enqueue the sender thread and its `message_info_send` on the endpoint's queue of senders.
            b. Place sender thread in `TS_BLOCKED_IPC_SEND` state.
            c. Context switch to another ready thread.
        **Receiving Phase (when a server calls `grx_IPC_Recv` or is already waiting):**
        1. Kernel validates `endpoint_capability` (must be an Endpoint cap with `RIGHT_RECV`).
        2. If the endpoint has a waiting sender thread:
            a. Dequeue the first sender.
            b. Transfer sender's `message_info_send` to current (receiver) thread.
            c. Place sender thread in `TS_BLOCKED_IPC_RECV` (waiting for reply on its implicit reply object).
            d. Current (receiver) thread continues execution (now has the message).
        3. Else (no sender waiting):
            a. Place current (receiver) thread in `TS_BLOCKED_IPC_RECV` state on this endpoint.
            b. Context switch.
        **Replying Phase (when server calls `grx_IPC_Reply`):**
        1. Kernel validates `reply_cap` (must be a temporary reply capability associated with the original `grx_Call`).
        2. Transfer `message_info_send` (from server, now acting as sender for reply) to the original caller's `message_info_recv` space.
        3. Make original caller thread `TS_READY`.
        4. Destroy/invalidate the `reply_cap`.
        5. Context switch if original caller is now highest priority.
        This simplified overview omits details like CNode operations for capability transfers for brevity but captures the essence.
  Asynchronous communication will be supported primarily through:
  i. `grx_SendAsync(target_capability, message_payload)`: A non-blocking send operation, similar to `seL4_Send`, allowing a thread to send a message without waiting for immediate receipt or reply. This is suitable for notifications or when the sender cannot block.
  ii. `grx_NotificationObject`: This kernel object provides a lightweight, non-blocking signaling mechanism.
        - **Structure:** A `grx_NotificationObject` primarily consists of a word-sized internal state (e.g., a bitmask or a counter) that can be atomically updated and waited upon.
        - **Operations (via `grx_KernelObject_Invoke` on a Notification capability):**
            - `NOTIF_METHOD_SIGNAL(bits_to_set)`: Atomically sets specified bits in the notification object's state. This operation is non-blocking.
            - `NOTIF_METHOD_CLEAR(bits_to_clear)`: Atomically clears specified bits.
            - `NOTIF_METHOD_WAIT(wait_mask, options, timeout)`: Blocks the calling thread until specified bits (defined by `wait_mask`) are set in the notification object's state. `options` could specify 'wait for all bits' vs 'wait for any bit'. A `timeout` can be specified. This is often used in conjunction with `grx_IPC_Recv` on a separate endpoint for more complex synchronization, or as a standalone primitive.
            - `NOTIF_METHOD_POLL(wait_mask)`: Non-blocking check if specified bits are set.
        This allows for efficient event notification where a full message exchange via an Endpoint is not required.
- Support for passing capabilities/handles over IPC. IPC messages will have a designated mechanism for securely transferring capabilities. The `message_info` structure (used for both send and receive parts of `grx_Call`, and for `grx_SendAsync`/`grx_IPC_Recv`) would be defined to include:
        - `uintptr_t tag;`: User-defined message label/ID.
        - `uintptr_t data_words[N];`: Array for N direct payload registers (e.g., N=4 or 8).
        - `grx_CapSlot_t cap_slots[M];`: Array for M capability transfer slots (e.g., M=1 or 4).
        `grx_CapSlot_t` is defined as:
        ```c
        typedef struct {
            uintptr_t cap_ CNode_ptr; // Pointer/index to the CNode holding the cap to be sent
            uintptr_t cap_ CNode_index; // Index within that CNode
            uint32_t  cap_rights_mask; // Rights to be associated if deriving a new cap, or 0 for full rights
            uint32_t  cap_badge;       // Badge for new capability (e.g., for endpoints)
        } grx_CapSlot_t;
        ```
        When sending, the user specifies the source capability via these fields. The kernel uses this to locate the capability in the sender's CSpace. On receive, these slots in the receiver's `message_info` would be populated by the kernel with information about the received capabilities (e.g., the new local capability pointer/ID, and its badge if it's an endpoint).
        The exact number of data words (N) and capability slots (M) will be fixed for the architecture to ensure predictable kernel processing and stack/register usage during IPC. For instance, a portion of the message payload structure will be reserved for capability slots. When a message containing capabilities is sent via `grx_Call` or `grx_SendAsync`, the kernel will validate these capabilities and transfer them from the sender's capability space to the receiver's, potentially into a specific capability slot or as part of the message buffer itself. This ensures that capability transfers are explicit and mediated by the kernel.
        - **Cybersecurity Warning:** While the kernel mediates IPC transfer, user-space services are responsible for validating the *content* of messages. Robust input validation in all services receiving IPC messages is crucial to prevent exploits based on malformed, unexpected, or malicious payloads.
        - **Cybersecurity Warning:** Untrusted components could attempt to cause denial-of-service by flooding an endpoint with IPC messages. Critical services should consider implementing service-level rate limiting or the kernel may provide options for endpoint queuing policies.
        - **Cybersecurity Consideration:** Establish a clear, auditable security policy for capability transfer via IPC, defining which components are authorized to mint, transfer, and receive specific types of capabilities.
- Guaranteed bounded IPC latency for real-time communication. The kernel's IPC path will be highly optimized to ensure that the time taken for a `grx_Call` (including send, context switch, receive, reply, and return context switch) is minimal and has a strictly bounded worst-case execution time (WCET). This is critical for schedulability analysis in hard real-time systems.
- Support for event notification mechanisms. Event notification will be primarily handled via the asynchronous IPC mechanisms described above, particularly `grx_NotificationObject` and `grx_SendAsync`, which allow for efficient, non-blocking signaling between components.
### 2.1.4. Interrupt Management
- Deterministic interrupt dispatch and handling. Physical hardware interrupts will initially be caught by a minimal, highly optimized kernel interrupt handling routine. This routine will perform essential state saving and acknowledge the interrupt at the hardware level. The kernel's role is to then swiftly and deterministically deliver an interrupt event to the appropriate user-space handler.
- Ability for user-space components to register for and receive interrupts via IPC (e.g., zx\_interrupt\_bind). User-space driver components register their interest in specific hardware interrupts by first obtaining or creating an `IRQ Handler Object` capability. This is typically done via `grx_KernelObject_Invoke` on a CNode capability with a method like `CNODE_METHOD_CREATE_IRQ_HANDLER_CAP(target_cnode_slot, irq_number, notification_endpoint_cap, notification_badge)`.
        Parameters for this creation/binding would include:
        - `target_cnode_slot`: The slot in the component's CNode where the new IRQ Handler capability will be placed.
        - `irq_number`: The physical interrupt number to handle.
        - `notification_endpoint_cap`: A capability to an `Endpoint` object to which interrupt notifications will be sent.
        - `notification_badge` (optional): A badge to be associated with the notification, allowing the handler thread to distinguish between different IRQs if using the same endpoint.
        **Algorithm/Steps for `CNODE_METHOD_CREATE_IRQ_HANDLER_CAP` (invoked via `grx_KernelObject_Invoke` on a CNode cap):**
        1. Kernel validates the calling thread's CNode capability and its right to create IRQ Handler capabilities.
        2. Kernel validates the `irq_number`: checks if it's a valid system IRQ and not already exclusively claimed by a non-shareable handler.
        3. Kernel validates the `notification_endpoint_cap`: ensures it's a valid Endpoint capability with `RIGHT_SEND` (for the kernel to send notifications to it).
        4. Kernel allocates an internal `IRQHandler` kernel object. This object stores the `irq_number`, the `notification_endpoint_cap`, and the `notification_badge`.
        5. Kernel creates a new IRQ Handler capability in the CNode slot specified by `target_cnode_slot`. This capability points to the internal `IRQHandler` object and grants rights like `RIGHT_HANDLE_IRQ` (allowing it to be waited upon for interrupts) and potentially `RIGHT_ACK_IRQ` (if explicit acknowledgement from user-space is required for some IRQ types).
        6. Kernel registers this `IRQHandler` object with its internal interrupt dispatch mechanism for the specified `irq_number`.
        7. Returns success or an error code.
        Alternatively, a pre-existing `IRQ Handler Object` for a given `irq_number` could be acquired from a privileged resource manager. A user-space driver thread will then bind this object to a physical IRQ line and can perform an IPC wait operation (e.g., `grx_InterruptWait()` or a general `grx_Call()` on the object) to block until an interrupt occurs. Upon interrupt occurrence for a registered IRQ, the kernel, after its minimal initial handling, will send an IPC message to the `notification_endpoint_cap` associated with the `IRQ Handler Object`. The `message_info` payload of this IPC notification would typically contain:
        - `tag`: Will be the `notification_badge` specified when the `IRQ Handler Object` was created and associated with the endpoint. This allows a single endpoint to receive notifications for multiple IRQs or other events, with the badge distinguishing the source.
        - `data_words[0]`: The specific `irq_number` that triggered the interrupt.
        - `data_words[1]`: Lower 32 bits of a 64-bit kernel-provided monotonic timestamp (e.g., nanoseconds since boot) of the interrupt event.
        - `data_words[2]`: Upper 32 bits of the 64-bit kernel-provided monotonic timestamp.
        - `data_words[3]`: Flags related to the interrupt event (e.g., `IRQ_FLAG_SPURIOUS` if detected, `IRQ_FLAG_OVERRUN` if applicable from hardware). (Max 1-2 more data words for future use if needed).
        - `cap_slots`: Generally, no capabilities are transferred with a basic interrupt notification. The handler already possesses the IRQ Handler capability.
        The user-space driver thread, upon receiving this message via `grx_IPC_Recv` on its endpoint, can then perform the detailed processing for the specified `irq_number`.
- Prioritization of interrupts. Hardware interrupt priorities, if supported by the platform, will be respected. The kernel may also allow mapping interrupt importance to the scheduling priority of the user-space handler threads to ensure that high-priority device interrupts are processed promptly. The specific mechanism for this mapping (e.g., handler thread inherits priority from IRQ configuration) will be defined.
## 2.2. System Services (User-Space Components)
All system services in grx will operate as isolated user-space components, interacting with each other and client applications via the kernel's IPC mechanisms. This component-based architecture, drawing inspiration from frameworks like Genode, ensures modularity and fault containment. Clear, well-defined interfaces, potentially specified using an Interface Definition Language (IDL), will govern these interactions.
- **Cybersecurity Note:** Each user-space service, despite its isolation from the kernel, contributes to the overall system attack surface. Vulnerabilities within a service can lead to compromise of that service's domain, and potentially broader impact depending on the capabilities it holds.
- **Cybersecurity Metric:** Track the number and severity of vulnerabilities discovered per user-space service through ongoing fuzzing, static analysis, and penetration testing (Target: Strive for zero exploitable critical/high vulnerabilities in released versions).
- **Cybersecurity Recommendation:** Strongly consider the use of memory-safe programming languages (e.g., Rust) for complex user-space services, especially those handling untrusted inputs (e.g., Network Stack, File System, IPC interfaces of drivers).
### 2.2.1. Device Drivers
- All device drivers (e.g., GPIO, UART, SPI, I2C, Ethernet, storage controllers) must run as isolated user-space processes. This model is similar to Redox OS and Zircon, where driver isolation enhances system stability and security. Each driver will expose its functionality through a standardized IPC-based interface.
    For example, a generic serial port driver might expose:
    - `METHOD_WRITE(buffer_cap, length)`: Client provides capability to a buffer. Driver reads `length` bytes.
        - **Request `message_info`:**
            - `tag`: `SERIAL_METHOD_WRITE` (predefined constant)
            - `data_words[0]`: `length` (number of bytes to write)
            - `cap_slots[0]`: `buffer_cap` (capability to a memory object with data to write)
        - **Response `message_info`:**
            - `tag`: `SERIAL_METHOD_WRITE_RESP`
            - `data_words[0]`: `bytes_written` (actual number of bytes written)
            - `data_words[1]`: `error_code` (0 for success)
    - `METHOD_READ(buffer_cap, length)`: Client provides capability to a buffer. Driver writes up to `length` bytes. Response indicates bytes read or error.
    - `METHOD_IOCTL(request_code, arg_cap)`: For device-specific control operations.
    A generic block device driver might expose:
    - `METHOD_READ_BLOCKS(block_start, count, buffer_cap)`
        - **Request `message_info`:**
            - `tag`: `BLKDEV_METHOD_READ_BLOCKS`
            - `data_words[0]`: `block_start` (starting block number)
            - `data_words[1]`: `count` (number of blocks to read)
            - `cap_slots[0]`: `buffer_cap` (capability to a memory object to store read data)
        - **Response `message_info`:**
            - `tag`: `BLKDEV_METHOD_READ_BLOCKS_RESP`
            - `data_words[0]`: `blocks_read`
            - `data_words[1]`: `error_code`
    - `METHOD_WRITE_BLOCKS(block_start, count, buffer_cap)`
    - `METHOD_GET_INFO()`: Returns block size, total blocks, etc.
    Requests would typically pass parameters in direct payload registers, and buffer capabilities in capability slots. Responses would use direct registers for status/counts.
### 2.2.2. File System
- Provide a robust, possibly crash-consistent, user-space file system service. The file system server will manage file operations, translating client requests received over IPC into block-level operations for storage device drivers. Its design will prioritize reliability and data integrity, potentially drawing from designs seen in microkernel-based systems that feature user-space file servers.
    Key IPC interfaces for a file system service would include:
    - `FS_METHOD_OPEN(path_string_cap, path_length, flags)`: `path_string_cap` points to shared memory with the path. Returns a file descriptor capability (`fd_cap`) or error.
        - **Request `message_info`:**
            - `tag`: `FS_METHOD_OPEN`
            - `data_words[0]`: `flags`
            - `data_words[1]`: `path_length`
            - `cap_slots[0]`: `path_string_cap` (capability to memory object containing path string)
        - **Response `message_info`:**
            - `tag`: `FS_METHOD_OPEN_RESP`
            - `data_words[0]`: `error_code`
            - `cap_slots[0]`: `fd_cap` (capability to the open file object/context, if successful)
    - `FS_METHOD_CLOSE(fd_cap)`: Closes the file.
    - `FS_METHOD_READ(fd_cap, offset, length, buffer_cap)`: Client provides `buffer_cap` for driver to write into. Returns bytes read.
        - **Request `message_info`:**
            - `tag`: `FS_METHOD_READ`
            - `data_words[0]`: `offset` (lower 32 bits)
            - `data_words[1]`: `offset` (upper 32 bits, if 64-bit offset needed)
            - `data_words[2]`: `length`
            - `cap_slots[0]`: `fd_cap`
            - `cap_slots[1]`: `buffer_cap`
        - **Response `message_info`:**
            - `tag`: `FS_METHOD_READ_RESP`
            - `data_words[0]`: `bytes_read`
            - `data_words[1]`: `error_code`
    - `FS_METHOD_WRITE(fd_cap, offset, length, buffer_cap)`: Client provides `buffer_cap` with data to write. Returns bytes written.
    - `FS_METHOD_STAT(fd_or_path_cap, path_length_if_path, stat_buffer_cap)`: Fills client-provided buffer with file status.
    - `FS_METHOD_MKDIR(path_string_cap, path_length, mode)`
    - `FS_METHOD_RMDIR(path_string_cap, path_length)`
    - `FS_METHOD_UNLINK(path_string_cap, path_length)`
    File descriptor capabilities would be specific types of capabilities managed by the FS server, granting rights to an open file resource.
### 2.2.3. Networking Stack
- Implement a user-space networking stack (e.g., TCP/IP, UDP) with configurable protocols. The networking stack will be a user-space component, similar to approaches in some microkernel systems, processing network packets and providing standard socket APIs (or equivalent) via IPC to applications. This isolation helps protect the kernel from network-based attacks.
    A socket-like IPC interface would be provided:
    - `NET_METHOD_SOCKET(domain, type, protocol)`: Returns a socket capability (`socket_cap`).
        - **Request `message_info`:**
            - `tag`: `NET_METHOD_SOCKET`
            - `data_words[0]`: `domain` (e.g., AF_INET)
            - `data_words[1]`: `type` (e.g., SOCK_STREAM)
            - `data_words[2]`: `protocol` (e.g., IPPROTO_TCP)
        - **Response `message_info`:**
            - `tag`: `NET_METHOD_SOCKET_RESP`
            - `data_words[0]`: `error_code`
            - `cap_slots[0]`: `socket_cap` (if successful)
    - `NET_METHOD_BIND(socket_cap, address_cap, address_len)`
    - `NET_METHOD_LISTEN(socket_cap, backlog)`
    - `NET_METHOD_ACCEPT(socket_cap)`: Returns a new `socket_cap` for the client connection.
    - `NET_METHOD_CONNECT(socket_cap, address_cap, address_len)`
    - `NET_METHOD_SEND(socket_cap, buffer_cap, length, flags)`
        - **Request `message_info`:**
            - `tag`: `NET_METHOD_SEND`
            - `data_words[0]`: `length`
            - `data_words[1]`: `flags`
            - `cap_slots[0]`: `socket_cap`
            - `cap_slots[1]`: `buffer_cap` (containing data to send)
        - **Response `message_info`:**
            - `tag`: `NET_METHOD_SEND_RESP`
            - `data_words[0]`: `bytes_sent`
            - `data_words[1]`: `error_code`
    - `NET_METHOD_RECV(socket_cap, buffer_cap, length, flags)`
    - `NET_METHOD_CLOSE(socket_cap)`
    Address and data buffers would be passed via capabilities to shared memory regions.
        - **Cybersecurity Metric:** Quantifiable compliance level with relevant network security protocol standards and best practices (e.g., RFCs, security hardening guides for protocols).
        - **Cybersecurity Metric:** Number of known, unpatched vulnerabilities in any chosen third-party network stack components (Target: 0).
        - **Cybersecurity Warning:** A network stack is an exceptionally large attack surface. If included, it must be rigorously hardened. Consider default-deny firewalling policies implemented by a dedicated firewall component, configured according to the principle of least functionality for network services.
### 2.2.4. Time Services
- Provide accurate system time, monotonic clocks, and timer services. A dedicated time management service will be responsible for this, interacting with hardware timers via user-space drivers and providing time information to other components through a secure IPC interface.
    Interfaces would include:
    - `TIME_METHOD_GET_CURRENT_TIME()`: Returns current system time.
        - **Request `message_info`:**
            - `tag`: `TIME_METHOD_GET_CURRENT_TIME`
        - **Response `message_info`:**
            - `tag`: `TIME_METHOD_GET_CURRENT_TIME_RESP`
            - `data_words[0]`: `error_code`
            - `data_words[1]`: `seconds_low` (lower 32 bits of seconds)
            - `data_words[2]`: `seconds_high` (upper 32 bits of seconds)
            - `data_words[3]`: `nanoseconds`
    - `TIME_METHOD_SET_TIMER(duration_ns, notification_cap, event_id)`: Sets a one-shot timer.
        - **Request `message_info`:**
            - `tag`: `TIME_METHOD_SET_TIMER`
            - `data_words[0]`: `duration_ns_low`
            - `data_words[1]`: `duration_ns_high`
            - `data_words[2]`: `event_id` (to be sent with notification)
            - `cap_slots[0]`: `notification_cap` (capability to a `grx_NotificationObject`)
        - **Response `message_info`:**
            - `tag`: `TIME_METHOD_SET_TIMER_RESP`
            - `data_words[0]`: `error_code`
            - `cap_slots[0]`: `timer_id_cap` (capability to the created timer object, for cancellation)
    - `TIME_METHOD_CREATE_PERIODIC_TIMER(interval_ns, notification_cap, event_id)`: Creates a periodic timer.
    - `TIME_METHOD_CANCEL_TIMER(timer_id_cap)`: Cancels an active timer.
    Timer IDs could be capabilities to timer objects managed by the time service.
### 2.2.5. Resource Management
- A root resource manager (initial user-space process) responsible for initial system composition, resource allocation, and capability delegation. This root server, conceptually similar to seL4's root task or Genode's 'core' component, plays a critical role in establishing the initial system state and enforcing resource allocation policies based on system configuration.
    The root resource manager would expose interfaces for:
    - `RM_METHOD_CREATE_COMPONENT(manifest_cap, &component_id_cap, &initial_tcb_cap)`: To start new components.
        - **Request `message_info`:**
            - `tag`: `RM_METHOD_CREATE_COMPONENT`
            - `cap_slots[0]`: `manifest_cap` (capability to memory object containing component manifest)
        - **Response `message_info`:**
            - `tag`: `RM_METHOD_CREATE_COMPONENT_RESP`
            - `data_words[0]`: `error_code`
            - `cap_slots[0]`: `component_id_cap` (optional, capability representing the component instance)
            - `cap_slots[1]`: `initial_tcb_cap` (capability to the main thread of the new component)
    - `RM_METHOD_GET_CAPABILITY(service_name_cap, &service_endpoint_cap)`: A basic service discovery mechanism.
    - `RM_METHOD_ALLOCATE_UNTYPED_MEMORY(size, alignment, &untyped_cap)`: For delegating large chunks of untyped memory to sub-managers or trusted components.
    - `RM_METHOD_CREATE_IRQ_HANDLER(irq_num, endpoint_cap, &irq_handler_cap)`: For managing access to interrupt handling.
### 2.2.6. System Monitoring & Logging
- Components for real-time monitoring of system health, resource usage, and secure logging. These components will gather data via IPC from other services and the kernel (where permissible and controlled by capabilities), providing a secure and reliable audit trail and system health overview.
    Key interfaces:
    - Logging Service:
        - `LOG_METHOD_SUBMIT_LOG(log_buffer_cap, length, severity_level)`
        - **Request `message_info`:**
            - `tag`: `LOG_METHOD_SUBMIT_LOG`
            - `data_words[0]`: `length`
            - `data_words[1]`: `severity_level`
            - `cap_slots[0]`: `log_buffer_cap`
        - **Response `message_info`:** (Likely asynchronous, or simple ack)
            - `tag`: `LOG_METHOD_SUBMIT_LOG_ACK`
            - `data_words[0]`: `error_code` (e.g., if log queue is full)
    - Monitoring Service:
        - `MONITOR_METHOD_GET_CPU_USAGE(component_id_cap_or_all)`: Returns CPU usage statistics.
        - `MONITOR_METHOD_GET_MEMORY_STATS(component_id_cap_or_all)`: Returns memory usage.
        - `MONITOR_METHOD_SUBSCRIBE_EVENT(event_type, notification_cap)`: For real-time event notifications.
### 2.2.7. Secure Storage
- User-space services for secure data storage and key management. This service will interact with underlying storage drivers and potentially hardware-backed secure elements (via HRoT integration) to provide confidential and integrity-protected storage facilities to authorized applications through defined IPC interfaces.
    Interfaces might include:
    - `SSTORE_METHOD_OPEN_OBJECT(object_id_cap, flags, &object_handle_cap)`
    - `SSTORE_METHOD_CREATE_OBJECT(size, flags, &object_id_cap, &object_handle_cap)`
        - **Request `message_info`:**
            - `tag`: `SSTORE_METHOD_CREATE_OBJECT`
            - `data_words[0]`: `size_low`
            - `data_words[1]`: `size_high`
            - `data_words[2]`: `flags` (e.g., access permissions, encryption attributes)
        - **Response `message_info`:**
            - `tag`: `SSTORE_METHOD_CREATE_OBJECT_RESP`
            - `data_words[0]`: `error_code`
            - `cap_slots[0]`: `object_id_cap` (persistent identifier capability)
            - `cap_slots[1]`: `object_handle_cap` (session handle capability for I/O)
    - `SSTORE_METHOD_READ(object_handle_cap, offset, length, buffer_cap)`
    - `SSTORE_METHOD_WRITE(object_handle_cap, offset, length, buffer_cap)`
    - `SSTORE_METHOD_CLOSE_OBJECT(object_handle_cap)`
    - `SSTORE_METHOD_DELETE_OBJECT(object_id_cap)`
    Object IDs and handles could be capabilities ensuring access control.
        - **Cybersecurity Metric:** Documented cryptographic strength of algorithms used (key sizes, modes of operation, compliance with NIST recommendations).
        - **Cybersecurity Warning:** Secure key management for this service is paramount. If keys are mishandled, all stored data can be compromised. Strongly consider integration with HRoT for key protection and cryptographic operations.
        - **Cybersecurity Consideration:** Implement anti-tamper mechanisms for stored data where feasible, and ensure data-at-rest protection policies are clearly defined.
## 2.3. Security Features
### 2.3.1. Secure Boot Chain
- Multi-stage bootloader (RBL \-\> SBL \-\> Kernel) with cryptographic verification at each stage (e.g., ECDSA signatures). The RBL, ideally immutable in hardware, will verify the SBL. The SBL will then verify the grx kernel image before execution. This chain of trust, similar to the principles employed by Zephyr's MCUBoot, ensures that only authenticated and integral code is loaded from power-on.
    The SBL will pass critical information to the kernel upon handoff, potentially via a boot information structure (e.g., pointed to by a register). This structure should include:
    - **Memory Map:** A list of available physical memory regions, their sizes, and types (e.g., RAM, reserved, device memory).
    - **Kernel Command Line (Optional):** Any parameters intended for kernel or early user-space configuration.
    - **Device Tree Blob (DTB) Location:** Physical address of the DTB, if used (see Section 4.6).
    - **Initial Ramdisk Location (Optional):** If an initial ramdisk containing essential early user-space components is used.
    - **Boot Source Information:** Indication of which partition was booted from (e.g., for A/B updates).
    The integrity of this boot information structure itself should be implicitly covered by the SBL's and kernel's own integrity checks.
    A potential C-style structure for this boot information (`grx_BootInfo_t`) passed from SBL to kernel could be:
    ```c
    #define MAX_MEM_REGIONS 16
    // Cybersecurity Metric: Documented cryptographic strength of signing algorithms and hash functions used (e.g., ECDSA P-384, SHA-384).
    // Cybersecurity Metric: Measured time for each cryptographic verification stage in the boot process (to detect performance regressions that might indicate bypass attempts).
    // Cybersecurity Warning: Physical attacks (e.g., glitching, side-channel analysis, direct memory access if exposed) on the RBL, SBL, or non-volatile storage could potentially bypass secure boot. System integrators must consider platform-specific hardware security measures.
    // Cybersecurity Consideration: Implement anti-rollback protection for all firmware stages (RBL if possible, SBL, Kernel) to prevent downgrades to versions with known vulnerabilities. This should be coordinated with the OTA update mechanism.
    typedef struct {
        uint64_t base_addr;
        uint64_t length;
        uint32_t type; // e.g., MEM_TYPE_RAM, MEM_TYPE_RESERVED, MEM_TYPE_DEVICE
        uint32_t reserved;
    } grx_MemoryRegion_t;

    typedef struct {
        uint32_t version; // Version of this bootinfo structure
        char kernel_cmd_line[128]; // Optional null-terminated command line
        uint64_t dtb_phys_addr;    // Physical address of Device Tree Blob
        uint64_t initrd_phys_addr; // Physical address of Initial Ramdisk (0 if none)
        uint32_t boot_partition_id; // Identifier for the partition booted from
        uint32_t num_mem_regions;
        grx_MemoryRegion_t mem_regions[MAX_MEM_REGIONS];
        // Other platform specific info may follow
    } grx_BootInfo_t;
    ```
    (Note: This C struct is illustrative of the data to be passed; the exact representation in memory would be platform-agreed.)
### 2.3.2. Hardware Root of Trust (HRoT) Integration
- Leverage hardware features (e.g., eFuses, TPM/HSM, Arm TrustZone) for secure key storage and cryptographic operations. This includes utilizing secure elements for storing cryptographic keys for boot image verification and runtime cryptographic operations, ensuring keys are protected from software compromise.
    Specific cryptographic operations that grx aims to offload to an HRoT (like a TPM or Secure Element) include:
    - **Secure Key Generation:** Generating cryptographic keys within the HRoT.
    - **Secure Key Storage:** Storing private keys or sensitive symmetric keys within the HRoT's protected storage.
    - **Digital Signatures:** Performing signing operations using keys held within the HRoT.
    - **Decryption/Encryption:** Offloading bulk or sensitive data decryption/encryption using HRoT-managed keys.
    - **True Random Number Generation (TRNG):** Accessing a hardware TRNG via the HRoT.
    - **Secure Attestation:** Allowing the HRoT to attest to the system's boot state and software configuration.
    User-space components will interact with the HRoT via a dedicated, secure driver and IPC mechanism, mediated by the kernel where necessary to enforce access control to HRoT functions.
    For example, accessing a True Random Number Generator (TRNG) via an HRoT driver could have the following IPC interface:
    - **Request to HRoT TRNG service (e.g., `HROT_TRNG_METHOD_GET_RANDOM_BYTES`):**
        - `message_info_send`:
            - `tag`: `HROT_TRNG_GET_RANDOM_BYTES_REQ`
            - `data_words[0]`: `num_bytes_requested` (e.g., up to a certain max per call)
            - `cap_slots[0]`: `buffer_cap` (capability to a client-owned memory object to store the random bytes)
    - **Response from HRoT TRNG service:**
        - `message_info_recv`:
            - `tag`: `HROT_TRNG_GET_RANDOM_BYTES_RESP`
            - `data_words[0]`: `error_code` (0 for success)
            - `data_words[1]`: `bytes_written` (actual number of random bytes written to `buffer_cap`)
        - **Cybersecurity Metric:** Percentage of critical cryptographic operations (key generation, signing, decryption of secrets) successfully offloaded to and performed by the HRoT.
        - **Cybersecurity Warning:** The software driver and IPC interface to the HRoT are security-critical components. Vulnerabilities here could undermine or bypass the HRoT's protections. This interface must be minimal and rigorously tested.
        - **Cybersecurity Consideration:** Ensure the HRoT provides a certified True Random Number Generator (TRNG) and that it is the exclusive source of entropy for all cryptographic key generation and other security-sensitive operations within grx.
### 2.3.3. Principle of Least Privilege (PoLP)
- Enforced by capability-based security model. This will be rigorously enforced by the kernel's capability-based access control system, inspired by seL4 and Zircon. Components will only be granted the minimum set of capabilities necessary for their function, and these capabilities cannot be forged or easily escalated.
    A grx capability will conceptually consist of:
    - **Object Pointer/ID:** A reference to the kernel object it grants access to.
    - **Rights Mask:** A bitmask defining the specific operations permitted on that object (e.g., read, write, execute, grant, map, invoke method X).
    - **Type Information:** Information about the type of object the capability refers to, for kernel validation.
    Basic rights applicable across various object types would include:
    - `RIGHT_READ`: Permission to read from the object (e.g., memory object, message from endpoint).
    - `RIGHT_WRITE`: Permission to write to the object (e.g., memory object, send to endpoint).
    - `RIGHT_EXECUTE`: Permission to execute code within a memory object (if applicable).
    - `RIGHT_GRANT`: Permission to delegate this capability (or a subset of its rights) to another component.
    - `RIGHT_MANAGE`: Permission to perform control operations on the object (e.g., destroy, configure).
    Specific objects will have more granular rights defined (e.g., `TCB_RIGHT_SET_PRIORITY`). The kernel rigorously validates these rights on every operation.
    The `Rights Mask` on a capability could be a 32-bit or 64-bit integer, with bits defined as follows (example):
    ```c
    // Generic Rights (applicable to many object types)
    #define CAP_RIGHT_READ      (1U << 0)  // General read access
    #define CAP_RIGHT_WRITE     (1U << 1)  // General write access
    #define CAP_RIGHT_GRANT     (1U << 2)  // Permission to delegate/copy the capability
    #define CAP_RIGHT_MANAGE    (1U << 3)  // Permission for control operations (e.g., delete, configure)
    #define CAP_RIGHT_MAP       (1U << 4)  // Permission to map a memory object

    // Endpoint specific rights
    #define CAP_RIGHT_EP_SEND   (1U << 8)  // Permission to send to an endpoint
    #define CAP_RIGHT_EP_RECV   (1U << 9)  // Permission to receive from an endpoint (and get reply cap)
    #define CAP_RIGHT_EP_CALL   (CAP_RIGHT_EP_SEND | CAP_RIGHT_EP_RECV) // For convenience

    // TCB specific rights
    #define CAP_RIGHT_TCB_CONTROL (1U << 12) // Configure, start, stop, set priority etc.

    // Add more specific rights for other object types as needed...
    ```
    When deriving a new capability from an existing one, the new capability's rights mask must be a subset of the original capability's rights mask.
### 2.3.4. Attack Surface Minimization
- Achieved through microkernel design and user-space services. The extremely small TCB of the microkernel (aiming for seL4-like minimality) and the confinement of complex services (drivers, filesystems, network stacks) to isolated user-space components are the primary strategies here.
### 2.3.5. Memory Safety
- Prevention of common memory vulnerabilities (e.g., buffer overflows) through architectural design and potentially language choice for user-space. Architectural design will contribute to memory safety through strict MMU/MPU-enforced isolation. Furthermore, for user-space components, the use of memory-safe programming languages (e.g., Rust, as suggested in the OS Kernel Analysis for grx Section 3.6) will be strongly encouraged and supported to prevent common vulnerabilities like buffer overflows and use-after-free errors at the application and service level.
### 2.3.6. Secure Updates
- Atomic, cryptographically verified Over-the-Air (OTA) update mechanisms with rollback capabilities (A/B partitioning). These will primarily be achieved using an A/B dual-partition scheme for the system image. The update manager will write the new image to the inactive partition, verify its integrity and authenticity, and then trigger a boot into the new version. Successful boot confirms the update; failure triggers an automatic rollback to the previous known-good version. This process will draw from established practices seen in systems like Fuchsia (Zircon) and Zephyr.
    A dedicated user-space 'Update Manager' component will be responsible for the OTA update process. Its roles include:
    - Receiving the new system image package.
    - Authenticating and verifying the package using its embedded signatures and metadata.
    - Writing the verified image to the inactive partition.
    - Communicating with the SBL (e.g., by writing to a shared, non-volatile status block) to:
        - Indicate that a new valid image is ready in the inactive partition.
        - Request a 'trial boot' into the new image.
    - After a successful trial boot, confirming the update to the SBL (again, via the status block) to make the new partition the permanent active one.
    - If the trial boot fails (e.g., system doesn't come up correctly or a health check fails), the SBL will automatically roll back to the previous partition. The Update Manager may then report the failure.
    The status block shared between the Update Manager (user-space) and the SBL (bootloader) could be a simple structure in a dedicated, non-volatile memory region (e.g., raw flash partition, RPMB). Example structure (`grx_UpdateStatusBlock_t`):
    ```c
    #define UPDATE_MAGIC 0xGR1T05UPDT
    #define PARTITION_A  0
    #define PARTITION_B  1

    typedef enum {
        BOOT_STATUS_NORMAL       = 0, // Boot normally from current_partition
        BOOT_STATUS_TRY_UPDATE   = 1, // SBL should try booting update_partition
        BOOT_STATUS_CONFIRM_OK   = 2, // Update was successful, make update_partition current
        BOOT_STATUS_REVERT       = 3  // Update failed, revert to previous partition
    } grx_BootAttemptStatus_t;

    typedef struct {
        uint32_t magic_number;         // To validate structure integrity
        uint8_t  current_boot_partition; // e.g., PARTITION_A or PARTITION_B
        uint8_t  update_target_partition;// Partition being updated / to try next
        grx_BootAttemptStatus_t attempt_status; // Instruction to SBL, or status from SBL
        uint32_t boot_attempts_left;   // For trial boots before reverting
        uint32_t image_version_current;
        uint32_t image_version_target;
        uint8_t  checksum[32];         // Checksum of this status block (excluding checksum field)
    } grx_UpdateStatusBlock_t;
    ```
    The Update Manager writes to `update_target_partition` and sets `attempt_status` to `BOOT_STATUS_TRY_UPDATE`. The SBL reads this, attempts boot, and updates `current_boot_partition` and `attempt_status` based on outcome or confirmation from a running system.
        - **Cybersecurity Metric:** Time to complete an OTA update cycle (download, verification, install, reboot) - target to be defined based on availability and security exposure window requirements. (This is also mentioned in 1.3, ensuring comprehensive coverage)
        - **Cybersecurity Metric:** OTA rollback success rate for verified failures of a new image (Target: >=99.9%). (Also in 1.3)
        - **Cybersecurity Metric:** Size of update packages (evaluate for efficiency and minimizing attack surface during transit/storage). (Also in 1.3)
        - **Cybersecurity Note:** The security of the entire OTA infrastructure (update server, package signing, key management) is paramount. (Also in 1.3)
        - **Cybersecurity Warning:** The OTA mechanism itself (e.g., Update Manager component, SBL update logic) must be hardened against vulnerabilities. (Also in 1.3)
        - **Cybersecurity Consideration:** Implement a defined process for revoking and replacing compromised OTA signing keys. (Also in 1.3)
### 2.3.7. Threat Modeling
- **Cybersecurity Process:** Continuous threat modeling and security analysis (e.g., STRIDE, Attack Trees) will be performed throughout the grx lifecycle, from design through maintenance.
        - **Cybersecurity Metric:** Percentage of architectural components, interfaces, and trust boundaries covered by up-to-date threat models (Target: 100% for security-critical elements).
        - **Cybersecurity Metric:** Number of identified threats and their mitigation status (e.g., mitigated by design, by operational control, by assumption, risk accepted). Track residual risk.
        - **Cybersecurity Note:** Threat models must be reviewed and updated in response to design changes, new features, discovered vulnerabilities, or emerging attack vectors. Outputs from threat modeling will directly inform security requirements, testing strategies, and risk assessments.
## 2.4. Functional Safety Features
### 2.4.1. Fault Containment
- Strict isolation of user-space components to prevent fault propagation (e.g., a driver crash does not affect the kernel). This is a direct benefit of the microkernel architecture, where, similar to seL4 and Redox OS, the kernel mediates all interactions. A fault in a user-space driver or service is confined to that component's protection domain, preventing corruption of the kernel or other isolated components. The kernel will ensure that a faulting component cannot escalate its privileges or gain unauthorized access to resources.
    Specifically, when a component attempts an invalid operation (e.g., accessing memory outside its mapped regions, using a capability with insufficient rights, or executing a privileged instruction), the kernel will:
    1. Trap the fault.
    2. Not propagate the fault to other components or the kernel itself, maintaining their integrity.
    3. Identify the faulting component and the nature of the fault.
    4. Deliver an exception IPC (as described in Section 2.1.2 Memory Fault Handling) to a registered fault handler for that component. If no handler is registered, or if the handler cannot resolve the issue, the kernel will ensure the component is safely terminated and its resources (capabilities it holds, memory it exclusively owns) are reclaimed or marked invalid, preventing further impact on the system.
        - **Safety Metric:** Effectiveness of fault containment barriers (e.g., as measured by targeted fault injection testing – percentage of injected faults successfully contained within the originating component's protection domain without propagation of hazardous effects). Target: Define based on SIL/ASIL (e.g., >99.9% for specific fault classes affecting high ASIL functions).
        - **Safety Warning:** While the microkernel provides hardware-enforced isolation, developers must meticulously analyze shared resources (even if access-controlled via capabilities) and logical dependencies between components for potential pathways of fault propagation or common cause failures (CCFs). Perform thorough inter-component safety analysis (e.g., FMEA at component interfaces). The grx Safety Manual will provide further details on assumed failure modes and integrator responsibilities regarding fault containment for HARA.
### 2.4.2. Predictable Error Behavior
- Deterministic handling and recovery from faults. The deterministic nature of the grx microkernel, stemming from its minimal TCB, static kernel memory allocation (as per seL4's principles), and bounded IPC execution times, is fundamental to this. Error handling paths within the kernel will be designed for predictability. For user-space components, their isolated nature allows for more predictable system-level responses to their individual failures. The predictability of these error behaviors is fundamental for system integrators to determine and meet their application-specific Fault Detection Time Interval (FDTI) and Fault Tolerant Time Interval (FTTI) requirements, ensuring faults are handled before they can lead to hazardous states.
### 2.4.3. Error Detection Mechanisms
- Stack overflow detection, memory protection faults, watchdog timers. The timeliness and coverage of these mechanisms are critical for achieving the required Fault Detection Time Interval (FDTI) for various fault types. The kernel will provide mechanisms for user-space components to report critical errors. Health monitoring components will utilize these mechanisms to detect and act upon system anomalies.
    In addition to hardware mechanisms like watchdog timers and memory protection faults (MPU/MMU), the kernel will provide a basic 'heartbeat' or 'health check' primitive. For example, a critical user-space component (like a Health Monitor) could be granted a special capability to:
    - `grx_KernelObject_Invoke(target_tcb_cap, TCB_METHOD_GET_HEALTH_STATUS, &status_word)`: Allows a privileged monitor to query a basic health status from a TCB (e.g., has it run recently, is it blocked indefinitely without a valid reason).
    The `status_word` returned by `TCB_METHOD_GET_HEALTH_STATUS` could be a bitmask with flags such as:
    ```c
    #define TCB_HEALTH_NORMAL             (0U)      // No issues detected by kernel
    #define TCB_HEALTH_SUSPECTED_HUNG     (1U << 0) // Kernel heuristics suggest thread might be hung (e.g., long time in runnable without running, or in uninterruptible wait)
    #define TCB_HEALTH_STACK_OVERFLOW_POTENTIAL (1U << 1) // If stack guard pages are used and a fault occurred near stack limit
    #define TCB_HEALTH_IPC_XRUN           (1U << 2) // Exceeded expected IPC time budget or deadline (if tracked)
    // Add other relevant kernel-detectable health indicators
    ```
Effective use of such health status information by a Health Monitor is crucial for timely fault detection, contributing to the overall FDTI budget.
    This status is primarily for privileged health monitoring components.
    - `grx_KernelObject_Invoke(monitored_component_endpoint_cap, MONITOR_MSG_HEARTBEAT_REQUEST)`: A component sends a heartbeat. If the Health Monitor doesn't receive it within a defined window, it can take action.
    Components themselves can also report internal errors to a designated system logger or error handling service via standard IPC, as detailed in Section 2.2.6. All error detection and reporting mechanisms must be designed with consideration for the FDTI constraints of the safety functions they protect.
        - **Safety Metric:** Diagnostic Coverage (DC) achieved by the sum of implemented error detection mechanisms for critical hardware and software elements. Targets for DC will be derived from system-level safety goals and standards (e.g., ISO 26262 single-point fault metric, latent fault metric).
        - **Safety Warning:** Undetected latent faults can compromise safety mechanisms or lead to violations of safety goals when a second fault occurs. Strive for high diagnostic coverage for all safety-critical elements. The reliability and effectiveness of the detection mechanisms themselves must be ensured and validated.
### 2.4.4. Fault Recovery Strategies
- Ability to restart individual user-space services (Redox OS's "restartless design"), graceful degradation. grx will support the restarting of failed user-space services independently, a concept similar to the 'restartless' design aspects of Redox OS. For example, a failed device driver could be reinitialized by a supervisory component without requiring a full system reboot. For more critical failures, the system may enter a pre-defined safe state or trigger a controlled shutdown. The specific recovery strategy will be configurable based on the component's criticality and system policy. All fault recovery actions, including service restarts or graceful degradation, must be designed to complete within the application-defined Fault Tolerant Time Interval (FTTI) to prevent hazardous events. The time taken to detect the fault (FDTI), notify the supervisor, reclaim resources, and re-instantiate the service contributes to this FTTI.
    The protocol for a supervisor (parent component or a dedicated System Supervisor component) to restart a faulting child component would generally involve:
    1. **Fault Notification:** (This notification must occur within the allocated FDTI). The supervisor is notified of the child's fault (e.g., via an exception IPC forwarded from the child's fault handler, or if the child process terminates unexpectedly).
    The exception IPC message format for general component faults (distinct from low-level memory faults if handled differently by the kernel, though often they are the same initial trap) would be similar to the memory fault message (Section 2.1.2), but potentially with a different `EXCEPTION_TYPE_*` code in `data_words[0]`. For example:
    - `data_words[0]`: `EXCEPTION_TYPE_UNDEFINED_INSTRUCTION`, `EXCEPTION_TYPE_PRIVILEGED_INSTRUCTION`, `EXCEPTION_TYPE_SYSCALL_ERROR`, `EXCEPTION_TYPE_UNHANDLED_IRQ`.
    The remaining fields (`faulting_address`, `PC`, `SP`, `fault_status_register`, `access_type_flags`) would be populated as applicable to the specific fault type. The key is providing enough context to the fault handler.
    2. **Resource Revocation/Reclamation:** The supervisor, using its capabilities, directs the kernel (or cooperates with the Memory Manager) to revoke capabilities held by the faulting component and reclaim its exclusive resources (e.g., memory objects, endpoints). This ensures a clean state.
    3. **State Analysis (Optional):** The supervisor might analyze any persistent state or logs from the failed component to decide if a restart is appropriate.
    4. **Re-instantiation:** The supervisor re-creates the component based on its manifest (see Section 4.4), establishing new capabilities and resources.
    5. **Service Re-establishment:** If the component provided services, clients might need to re-bind to the new instance, possibly managed via a service discovery mechanism.
        - **Safety Metric:** Measured success rate of automated fault recovery actions (e.g., service restart, transition to a safe state, graceful degradation) for specific, credible fault scenarios. Targets to be defined based on availability and safety requirements.
        - **Safety Note:** All fault recovery actions must be designed and verified to transition the system to a known safe state and must not themselves introduce new hazards. The behavior of recovery mechanisms under various fault conditions (including faults in the recovery mechanism itself, if feasible to analyze) should be analyzed.
### 2.4.5. Mixed-Criticality System (MCS) Support
- Mechanisms for strict resource partitioning (CPU time, memory, I/O) to ensure critical tasks are not impacted by non-critical ones (seL4 MCS extensions). grx will provide robust support for MCS by implementing mechanisms inspired by seL4's MCS extensions. This includes:
        - **Time Partitioning:** Guaranteed CPU time allocations for critical components, ensuring that lower-criticality components cannot starve critical ones. This will involve configuring the scheduler with strict time budgets.
    Parameters for time partitioning associated with a scheduling context or a group of threads would include:
    - **Period (P):** The time duration over which the budget applies (e.g., 10ms, 100ms).
    - **Budget (Q):** The maximum execution time a component (or group of threads) is allowed within its Period (e.g., 2ms of execution within a 10ms period).
    - **Deadline (D):** (Optional, if supporting deadline-based scheduling) The time within the period by which the budgeted execution must complete.
    The kernel scheduler will enforce these budgets, preempting components that exceed their allocated time and ensuring other components receive their guaranteed share. Configuration of these parameters would be a privileged operation, typically performed by the root resource manager during system setup based on a system-wide scheduling plan.
    This configuration could be managed via a dedicated kernel object, say a `grx_SchedulingContext` object, associated with one or more TCBs. A capability to this `SchedulingContext` object would be required to modify its parameters.
    **`grx_SchedulingContext` Object Attributes (configurable via `grx_KernelObject_Invoke`):**
    - `SCHEDCTX_METHOD_SET_BUDGET(period_ns, budget_ns)`: Sets the period (P) and budget (Q).
    - `SCHEDCTX_METHOD_SET_DEADLINE(deadline_ns)`: Sets the deadline (D), relative to period start.
    - `SCHEDCTX_METHOD_ASSOCIATE_THREAD(tcb_cap)`: Associates a thread with this scheduling context.
    - `SCHEDCTX_METHOD_GET_RUNTIME_STATS()`: Retrieves current budget consumed, overrun status, etc.
    The kernel scheduler would then use these `SchedulingContext` parameters to enforce time partitioning for all associated threads.
        - **Resource Segregation:** Strict separation of memory, I/O, and other resources using the capability system, ensuring that components can only access their allocated resources.
        - **Freedom From Interference (FFI):** The architecture will be designed and verifiable to prevent interference (both temporal and spatial) between components of different criticality levels.
                - **Safety Metric:** Level of FFI demonstrated (e.g., through formal analysis of partitioning mechanisms, results of extensive resource contention testing showing no violations of timing or memory protection for critical components by less critical ones).
                - **Safety Warning:** Ensuring robust FFI in MCS environments is exceptionally complex. All potential interference channels (timing, memory, I/O, shared kernel resources, IPC effects) must be identified and mitigated. The kernel's partitioning mechanisms are fundamental but require correct configuration and complementary system-level design.

# 3. Non-Functional Requirements
## 3.1. Performance
### 3.1.1. Boot Time
- Target: < 500 ms (or specific application-defined deadline). This target is achievable due to the minimal nature of the microkernel, a streamlined, verifiable bootloader (inspired by systems like Zephyr's MCUBoot), and the absence of complex service initialization within the kernel itself, similar to the rapid startup characteristics of RTOSs like FreeRTOS and microkernels like seL4.
        **Measurement:** From power-on until the initial set of critical user-space components (e.g., resource manager, essential drivers defined in the system configuration) are initialized and operational. The exact point of 'operational' will be defined per product configuration.
### 3.1.2. Interrupt Latency
- Target: < 10 μs (worst-case). Achievable through a highly optimized kernel interrupt path that performs minimal processing before swiftly delivering interrupt events to user-space handlers via efficient IPC. This design avoids lengthy in-kernel ISRs common in monolithic systems.
        **Measurement:** Time from hardware interrupt signal assertion to the execution of the first instruction in the registered user-space handler thread. This includes kernel processing time and the context switch to the handler thread. Measurements should be taken under various system loads, excluding other higher-priority interrupt processing.
### 3.1.3. Context Switch Time
- Target: < 1 μs. The microkernel's minimal state per thread and a highly optimized, deterministic scheduler allow for rapid context switches, comparable to high-performance RTOS and microkernels.
        **Measurement:** Time taken to switch from one user-space thread to another user-space thread of the same or lower priority, including saving the context of the outgoing thread and loading the context of the incoming thread. This does not include scheduler decision time, which should also be bounded and minimal.
### 3.1.4. Inter-Process Communication (IPC) Latency
- Target: < 5 μs (worst-case for small messages). This relies on a carefully designed IPC mechanism, optimized for speed and predictability, drawing from the performance focus of IPC in systems like seL4 and Zircon, where IPC is a fundamental operation.
        **Measurement:** For `grx_IPC_Call`, this is the round-trip time from a client thread invoking the call to a server thread (in a separate AddressSpace on the same core), the server processing a minimal reply, and the client thread receiving the reply and resuming execution. This assumes the server is already waiting on the endpoint. Target applies to messages with minimal direct payload (e.g., a few registers) and no capability transfer. Latency for messages with capability transfers or larger indirect payloads will be characterized separately.
### 3.1.5. Jitter
- Minimal and bounded jitter for real-time tasks. This will be achieved by ensuring bounded execution times for all kernel operations, static kernel memory allocation, and prioritized, preemptive scheduling, minimizing sources of non-determinism.
        **Measurement:** Variation in execution time for periodic tasks and variation in response time for event-driven tasks. Specific jitter tolerance will be defined per critical task in a given system configuration.
### 3.1.6. Throughput
- Define throughput requirements for critical data paths (e.g., network, storage). For example, for a user-space networking stack, this might be measured in packets-per-second (PPS) or Mbps for specific packet sizes and protocols, between two grx components on the same node, or to an external interface via a user-space driver. For a user-space file system, this might be measured in IOPS or MB/s for sequential and random read/write operations of various block sizes.
### 3.1.7. Resource Footprint
- **Kernel RAM:** Target: < 50 KB. This is aligned with the goal of a minimal TCB. This target accommodates essential kernel data structures (e.g., for a reasonable number of TCBs, CNodes, Endpoints, AddressSpaces, MemoryObjects, IRQ Handlers, all statically allocated from a pre-defined budget). The exact amount will depend on the compile-time configuration of maximum kernel objects supported.
- **Kernel Flash/ROM:** Target: < 100 KB. A small code size is a direct result of the microkernel design, containing only essential primitives (syscall handlers, scheduler, capability management, IPC logic, minimal HAL). This is comparable to seL4's verified kernel code size and is crucial for verifiability and reducing attack surface.
- **Minimum System RAM:** Define a target for a basic functional system (e.g., < 4 MB). While the kernel itself is tiny, this accounts for essential user-space services (root server, core drivers, IPC). The component-based architecture will allow scaling down by excluding non-essential services.
- **Minimum System Flash/ROM:** Define a target for a basic functional system (e.g., < 10 MB). This includes the kernel and a minimal set of user-space components required for operation. The modular design allows for tailoring the system image to specific device constraints.
### 3.1.8. Energy Efficiency
- Define power consumption targets for various operational states (active, idle, sleep). The kernel will provide primitives for power management (e.g., clock gating, idle states) that user-space power management services can leverage, similar to Zephyr's energy-conscious design. The minimal nature of the kernel itself contributes to lower background CPU usage.
## 3.2. Reliability & Availability
### 3.2.1. Mean Time Between Failures (MTBF)
- Define target (e.g., > 100,000 hours). This high MTBF target is supported by the microkernel architecture's inherent fault isolation (minimizing the impact of component failures, similar to seL4/Redox OS principles) and the potential for formally verified kernel components. This is further supported by the design goal of using memory-safe languages (e.g., Rust) for critical user-space components where feasible, reducing common sources of software faults. The minimal kernel TCB, especially if formally verified, significantly reduces the likelihood of kernel-induced system failures.
        **Derivation:** MTBF will be estimated using a combination of:
        1. Predictive analysis based on component failure rates (e.g., from hardware datasheets, software defect density models like Rome Laboratory TR-92-52 for user-space components).
        2. Results from extended duration system stability tests under stress conditions.
        3. Fault injection testing results to assess resilience to transient and permanent hardware faults.
        For the formally verified microkernel, a qualitative argument for extremely high reliability (approaching infinite MTBF for software faults covered by proofs) will be made, with overall system MTBF then dominated by hardware and non-verified software components.
### 3.2.2. Mean Time To Recovery (MTTR)
- Define target for critical services (e.g., < 100 ms for driver restart). Achievable due to the ability to restart individual user-space services (e.g., drivers) independently and quickly, a key benefit of the microkernel design (Redox OS 'restartless' concept). This MTTR is a critical factor in satisfying the overall Fault Tolerant Time Interval (FTTI) for safety-critical functions; recovery must be completed well within the FTTI. The component manifest (Section 4.4) will define restart policies (e.g., max restart count, backoff delays) for services, managed by a System Supervisor component to prevent flapping or resource exhaustion from continuous restarts.
        **Derivation:** MTTR will be measured empirically for defined critical component failure scenarios (e.g., driver crash, service becoming unresponsive). Measurements will capture detection time (by health monitor), restart protocol execution time (resource reclamation, component re-instantiation by supervisor), and service restoration time (component initialization to operational state). The sum of these times, particularly the detection time (contributing to FDTI) and the active recovery time, must be evaluated against the FTTI. The target applies to automated recovery without manual intervention.
### 3.2.3. Uptime
- Target (e.g., 99.999% for critical systems).
### 3.2.4. Fault Tolerance
- Specify ability to continue operation or degrade gracefully upon single/multiple point failures.
## 3.3. Maintainability & Updatability
### 3.3.1. Update Frequency
- Define expected frequency of security patches and feature updates.
### 3.3.2. Rollback Capability
- Guarantee ability to revert to previous stable version during updates. The SBL (Section 4.5) is responsible for detecting a failed boot attempt after an update (e.g., via a failed health check from a critical service or lack of explicit confirmation from the Update Manager) and automatically booting from the previous known-good partition.
### 3.3.3. Restartless Updates
- Maximize ability to update/restart individual components without system reboot. This is a direct outcome of the user-space component architecture, where services can be updated and restarted without kernel interruption, inspired by systems like Redox OS and the component models of Zircon/Genode.
### 3.3.4. Modularity
- Clearly defined interfaces and independent build/test for components. The component-based design, akin to Genode's framework, ensures that components can be developed, tested, and updated independently. The use of an IDL (Interface Definition Language) as proposed in Section 4.4.5 will be key to maintaining stable interfaces between components, allowing them to evolve independently as long as the interface contract is respected.
## 3.4. Security (Verifiable)
### 3.4.1. Common Criteria (CC) Evaluation Assurance Level (EAL)
- Target EAL (e.g., EAL4+ or higher if formal verification is used). Achieving a high EAL is predicated on the microkernel's minimal TCB, the potential for formal verification of the kernel (following seL4's example), and rigorous development processes. The evaluation will target a specific protection profile relevant to RTOS and safety/security critical systems. Evidence for EAL will include semi-formal design descriptions, rigorous testing, vulnerability analysis, and potentially formal proofs for selected security properties of the microkernel.
        - **Cybersecurity Note:** The EAL target will be supported by a Security Target (ST) document defining the scope of evaluation, security objectives, threats, and the security functional and assurance requirements to be met by grx.
        **Target Protection Profile:** While a specific off-the-shelf Protection Profile (PP) may not perfectly align, grx will aim to satisfy assurance requirements comparable to a general-purpose OS PP (e.g., OSPP) augmented with requirements from PPs for virtualization, separation kernels, or high robustness, tailored to its microkernel nature and target domains. Key security functions for evaluation will include: System Access Control, Data Protection, Security Audit, Identification & Authentication (for management interfaces), and Security Management.
### 3.4.2. Cryptographic Algorithm Compliance
- FIPS 140-2/3 compliance for cryptographic modules. All cryptographic algorithms, protocols, and random number generators used by grx components (kernel, user-space, bootloaders) must adhere to NIST recommendations or other relevant industry/government standards for algorithm selection, key sizes, and modes of operation (e.g., AES-256, SHA-384, ECDSA P-384, SP 800-56A/B for key agreement/derivation).
        - **Cybersecurity Metric:** Percentage of cryptographic modules/services that are FIPS 140-2/3 validated or formally proven compliant with specified standards.
### 3.4.3. Vulnerability Management
- A defined and documented vulnerability management process will be implemented, covering:
            - Vulnerability identification (internal testing, external reports, supply chain monitoring).
            - Tracking and prioritization (e.g., using CVSS - Common Vulnerability Scoring System).
            - Patch development and testing.
            - Secure disclosure and dissemination of patches.
        - **Cybersecurity Metric:** Mean Time To Patch (MTTP) for reported vulnerabilities, categorized by severity (e.g., critical vulnerabilities patched within X days).
        - **Cybersecurity Metric:** Number of open known high/critical vulnerabilities in any released codebase (Target: 0).
### 3.4.4. Attack Surface Metrics
- Quantify TCB size, number of syscalls. The target for a very low number of system calls (e.g., <15 similar to seL4, or <100 like Redox OS, as detailed in Section 4.1.1) is a key metric here.
## 3.5. Safety (Verifiable)
### 3.5.1. Safety Integrity Level (SIL)
- Target SIL (e.g., IEC 61508 SIL3/4, ISO 26262 ASIL D, DO-178C Level A). High SIL targets are supported by the formally verifiable aspects of the microkernel (inspired by seL4), strong fault isolation, predictable real-time behavior, and rigorous adherence to safety-critical development processes. Key inputs to this process include defining and meeting stringent Fault Tolerant Time Intervals (FTTI) and Fault Detection Time Intervals (FDTI) for identified safety goals. Achieving this will require a development process compliant with relevant standards (e.g., IEC 61508-3), including hazard analysis, safety requirements specification, and verification/validation activities with full traceability. Formal methods applied to the kernel would provide strong evidence for freedom from common software errors.
        - **Safety Note:** The specific target SIL/ASIL for grx or its components will dictate the required rigor of all development processes, verification and validation (V&V) activities, documentation, and the evidence compiled within the safety case.
        **Key Safety Functions for SIL Assessment (Examples):**
        1.  **Kernel Temporal and Spatial Partitioning:** Ensuring that components cannot exhaust CPU time or corrupt memory/state of other components beyond their allocated resources.
        2.  **Secure IPC:** Guaranteeing that IPC messages cannot be forged, tampered with, or misdirected, and that capability transfers are valid.
        3.  **Reliable Fault Detection and Reporting:** Ensuring the kernel reliably detects critical internal faults and can report them to designated handlers.
        4.  **Deterministic Scheduling:** Providing verifiable guarantees that high-priority critical tasks meet their deadlines.
        The specific safety functions and their required integrity levels will be derived from the hazard analysis of target applications built on grx.
### 3.5.2. Freedom From Interference (FFI)
- Mathematically provable FFI between critical and non-critical components. This will be demonstrated through a combination of the kernel's capability-based isolation, memory protection mechanisms, and potentially formal analysis of inter-component interactions, particularly in MCS configurations (drawing from seL4 MCS).
        - **Safety Note:** Demonstrating FFI is a cornerstone of the safety argument for systems utilizing MCS. This requires not only robust kernel-enforced partitioning mechanisms but also careful analysis and testing of all shared resources and potential inter-component dependencies.
### 3.5.3. Worst-Case Execution Time (WCET)
- Bounded WCET for all critical kernel paths and real-time tasks. Static WCET analysis tools will be employed for all kernel code and critical real-time user-space components. The design will avoid constructs that make WCET analysis difficult or impossible (e.g., unbounded loops, dynamic memory allocation in critical paths).
        - **Safety Metric:** Percentage of safety-critical software components and execution paths for which WCET has been determined and verified through analysis and on-target testing. Target: 100%.
        - **Safety Warning:** Inaccurate or unvalidated WCET analyses can lead to missed deadlines for safety-critical functions, potentially resulting in hazardous events. The WCET analysis process must be rigorous, account for all relevant hardware effects (CPU pipeline, caches, memory access times, interrupts), and its results must be confirmed with on-target measurements under defined conditions.
        **Methodology:** A hybrid approach will be used:
        1. Static analysis of source code and compiled binaries using tools like AbsInt aiT, Rapita Systems RVS, or similar, to determine execution paths and estimate instruction/cycle counts.
        2. Measurement-based analysis on target hardware to calibrate the static analysis model and account for hardware-specific effects (caches, pipelines). This involves using high-precision tracing or cycle counters.
        3. For formally verified kernel components, WCET may be derived or bounded as part of the formal proof process where feasible.
### 3.5.4. Failure Rate Targets
- Define acceptable failure rates for critical functions.
        - **Safety Metric:** Documented hardware component failure rates (e.g., FIT rates from manufacturer datasheets or reliability handbooks) and allocated software failure rate targets for safety-related software components.
        - **Safety Note:** These failure rates are critical inputs for quantitative safety analyses such as Fault Tree Analysis (FTA) and Reliability Block Diagrams. These analyses help to demonstrate that the overall system architecture meets its probabilistic safety goals (e.g., Probabilistic Metric for random Hardware Failures - PMHF - for ISO 26262).
## 3.6. Usability (Developer)
### 3.6.1. API Clarity
- Well-documented, intuitive APIs for kernel primitives and user-space services. APIs for kernel primitives will be minimal and precise, similar to seL4's approach. User-space service APIs will be defined using clear contracts, potentially through an IDL as mentioned in Section 2.2. Documentation for each kernel object method and user-space service interface will include pre-conditions, post-conditions, error codes, and example usage. The IDL (Section 4.4.5) will be the source of truth for user-space service API definitions.
        **Process:** All public APIs (kernel syscalls, kernel object methods, IDL-defined service interfaces) will undergo a formal internal review process before stabilization. This process will check against a defined API style guide covering naming conventions, parameter ordering, error handling patterns, and documentation completeness. The style guide will draw from established best practices (e.g., AUTOSAR C++14 coding guidelines for safety aspects, Linux kernel coding style for general readability where applicable).
### 3.6.2. Debugging Support
- Comprehensive debugging tools (on-target, remote, tracing, logging). This includes kernel-level support for debug attachment and user-space tracing facilities that can operate across isolated components, potentially leveraging hardware debug features.
### 3.6.3. Toolchain Integration
- Seamless integration with standard development toolchains (GCC/Clang, potentially Rust). This includes providing build system support (e.g., CMake/Meson scripts as per Section 5.2.6) for common development environments (IDEs) and integration with debugging tools (e.g., GDB stubs, OpenOCD configurations).
## 3.7. Portability
### 3.7.1. Supported Architectures
- List target CPU architectures (e.g., ARM Cortex-A/R/M, RISC-V, x86). Initial development will focus on a primary reference architecture (e.g., ARMv8-A 64-bit or RISC-V 64-bit with MMU and virtualization extensions). Ports to other architectures will follow based on project needs and HAL/BSP development effort.
### 3.7.2. Hardware Abstraction Layer (HAL) / Board Support Package (BSP) Strategy
- Define a clear HAL for easy porting to new hardware. The HAL will provide a minimal, stable interface for the kernel to interact with CPU-specific functions, timers, and interrupt controllers. The BSP structure will be inspired by modular systems like Zephyr, facilitating straightforward adaptation to new hardware by separating generic drivers from board-specific configurations (e.g., via Devicetree-like descriptions). The HAL API will be versioned to allow for evolution while maintaining compatibility for existing BSPs where possible. BSPs will include comprehensive documentation on hardware setup, peripheral mapping, and driver functionalities.

# 4. Architectural Design Details
## 4.1. Kernel Core Specification
### 4.1.1. System Call Interface
The grx System Call Interface is designed to be minimal, adhering to the principle of a small Trusted Computing Base (TCB). The target is to have a very restricted set of entry points into the kernel, similar in philosophy to seL4's highly optimized and minimal interface. We aim for fewer than 15 core system calls. User-space libraries will build upon these primitives to provide richer functionalities. The preliminary list of essential system calls is as follows:

    1.  `grx_IPC_Call(target_cap, msg_info, &reply_info)`: Synchronous IPC. Sends a message and waits for a reply. This is the primary mechanism for service invocation and inter-component communication. (Inspired by `seL4_Call`).
    2.  `grx_IPC_Send(target_cap, msg_info)`: Asynchronous (non-blocking) IPC send. Used for notifications or when the sender cannot block. (Inspired by `seL4_Send`).
    3.  `grx_IPC_Recv(endpoint_cap, &msg_info, &sender_info)`: Blocking receive. Waits for a message on an endpoint. Used by servers to receive requests. (Inspired by `seL4_Recv`).
    4.  `grx_IPC_Reply(reply_cap, msg_info)`: Used by a server to send a reply to a client that made a `grx_IPC_Call`. (Inspired by `seL4_Reply`).
    5.  `grx_KernelObject_Invoke(capability, method_id, args...)`: Generic method invocation on kernel objects (e.g., TCBs, Endpoints, Notification objects, Untyped Memory). This allows specific operations on objects without needing a unique syscall for each. The `method_id` and `args` would be specific to the object type. This approach helps keep the syscall count low. (Inspired by Zircon's object model and seL4's method invocation on capabilities).
        *   Examples of methods invokable via `grx_KernelObject_Invoke`:
            *   `TCB_Configure` (for a TCB capability)
            *   `TCB_SetPriority` (for a TCB capability)
            *   `TCB_SetState` (e.g., suspend, resume) (for a TCB capability)
            *   `Untyped_Retype` (for an Untyped Memory capability, to create other objects)
            *   `CNode_CopyCap` (for a CNode capability, to copy capabilities)
            *   `CNode_DeleteCap` (for a CNode capability, to delete capabilities)
            *   `IRQ_SetHandler` (for an IRQ capability, to associate with an endpoint)
            *   `Notification_Signal` (for a Notification capability)
            *   `Notification_Wait` (for a Notification capability, often combined with `grx_IPC_Recv` on an endpoint)
    6.  `grx_Yield()`: Relinquishes the CPU to the next ready thread. (Inspired by `seL4_Yield`).
    7.  `grx_DebugPutchar(char c)`: Minimal syscall for extremely low-level debugging output. To be used sparingly and potentially disabled in production builds.

    **Note:** This list is preliminary and subject to rigorous refinement during the detailed design phase. The aim is to achieve a minimal, verifiable interface. Many operations typically associated with syscalls in monolithic kernels (e.g., file I/O, network operations, detailed memory mapping beyond initial setup) will be handled by user-space servers accessed via IPC.

    **Trusted Computing Base (TCB) Size:**
    grx aims for a TCB for the core microkernel in the order of 10,000 to 15,000 lines of C code (LoC) for components intended for formal verification, drawing inspiration from the seL4 microkernel's achievements in minimizing its TCB for verifiability and security.
### 4.1.2. Kernel Objects
The grx kernel manages a small set of fundamental object types. Capabilities to these objects are held by user-space components, and operations on these objects are primarily invoked using the `grx_KernelObject_Invoke()` system call. The core kernel objects include:

    1.  **Capability:** The fundamental unit of access control in grx. A capability grants the possessing component specific rights to a kernel object (or another capability). Capabilities are unforgeable and managed by the kernel.
    2.  **CNode (Capability Space Node):** An object that stores capabilities. CNodes form part of a component's capability space. Operations include managing capabilities within the CNode (e.g., copy, move, delete - invoked via `grx_KernelObject_Invoke` on the CNode capability). (Inspired by seL4 CNodes).
    3.  **TCB (Thread Control Block):** Represents a thread of execution. Holds the thread's state, priority, scheduling parameters, and references to its address space and capability space (root CNode). Operations include configuring, starting, stopping, and setting priority. (Inspired by seL4 TCBs).
    4.  **Endpoint:** A kernel object used for receiving IPC messages. Threads with a capability to an endpoint can listen for messages sent to it. Endpoints are typically used in client-server interactions where the server listens on an endpoint. (Inspired by seL4 Endpoints).
    5.  **Notification:** A lightweight kernel object for signaling events between threads, without the overhead of full message passing. Threads can signal a notification object or wait for a signal. (Inspired by seL4 Notifications).
    6.  **AddressSpace (Virtual Address Space Object):** Represents a virtual address space. TCBs are associated with an AddressSpace object. Operations include mapping and unmapping memory regions (which themselves are represented by capabilities to memory objects like VMOs or retyped UntypedMemory).
    7.  **UntypedMemory:** Represents regions of raw physical memory that have not yet been assigned a specific type or function by the kernel. Capabilities to UntypedMemory allow privileged user-space components (like a root memory manager) to 'retype' this memory into other kernel objects (e.g., TCBs, Endpoints, CNodes, or other memory objects of specific sizes). This is the foundation of resource allocation. (Inspired by seL4 Untyped Memory).
    8.  **IRQ Handler Object:** A special capability granting a user-space component the right to handle a specific hardware interrupt. The kernel delivers interrupt notifications to an IPC endpoint associated with this IRQ Handler capability. (Inspired by seL4 IRQ Handlers).
    9.  **Memory Object (e.g., VMO - Virtual Memory Object like):** Represents a contiguous region of memory that can be mapped into an AddressSpace. These objects would be created from UntypedMemory by a memory manager component. (Inspired by Zircon VMOs, created from seL4-style UntypedMemory).

    The specific methods available for each object type are invoked through `grx_KernelObject_Invoke()` by passing the object's capability and a method identifier along with necessary arguments, as outlined in Section 4.1.1.
### 4.1.3. Scheduler Algorithm
grx will implement a deterministic, preemptive, priority-based scheduler designed for hard real-time systems.
    - **Primary Algorithm:** Fixed-Priority Preemptive (FPP) scheduling. The scheduler will always run the highest-priority runnable thread.
    - **Priority Levels:** A configurable range of priority levels will be supported (e.g., 256 levels, with 0 being the highest).
    - **Same-Priority Scheduling:** For threads at the same priority level, the following policies will be supported:
        - **FIFO (First-In, First-Out):** Threads run to completion (or until they block or are preempted by a higher priority thread) in the order they become ready.
        - **Round-Robin:** Threads run for a configurable time slice before yielding to the next same-priority thread in the ready queue.
    - **Advanced Scheduling Policies:** While FPP is the core, the kernel will provide primitives (e.g., precise priority control, time slice management) that enable user-space components or libraries to implement more advanced scheduling strategies like Earliest Deadline First (EDF) or Rate-Monotonic Scheduling (RMS) by dynamically adjusting thread priorities and parameters based on application requirements. Direct kernel support for these complex policies will be avoided to maintain kernel minimality, but hooks for user-space schedulers will be considered.
    - **Mixed-Criticality Systems (MCS) Support:** For MCS, the scheduler will support time partitioning by assigning strict CPU time budgets to groups of threads or components, ensuring critical tasks receive their allocated execution time regardless of the behavior of less critical tasks. This will be configured via kernel object properties on TCBs or dedicated scheduling context objects.
### 4.1.4. Interrupt Handling
Interrupt handling in grx is designed to be efficient, deterministic, and to delegate processing to user-space drivers as quickly as possible. The process is as follows:
    1.  **Initial Kernel Handling:** When a hardware interrupt occurs, a minimal, architecture-specific kernel routine is invoked. This routine:
        *   Saves minimal essential processor state.
        *   Identifies the source of the interrupt (IRQ number).
        *   Acknowledges the interrupt at the interrupt controller level to allow further interrupts (if appropriate for the IRQ type).
        *   Masks the specific IRQ line at the controller if it's a level-triggered interrupt or if the handler is not immediately ready to prevent interrupt storms.
    2.  **Event Delivery:** The kernel then checks if a user-space handler is registered for this IRQ via an `IRQ Handler Object` (see Section 4.1.2).
        *   If a handler is registered and waiting (e.g., on an Endpoint associated with the IRQ Handler object), the kernel constructs a notification message and signals the associated Endpoint or Notification object. This unblocks the waiting user-space driver thread.
        *   The notification may include information like the IRQ number and a timestamp.
    3.  **User-Space Processing:** The awakened user-space driver thread performs the actual interrupt processing (e.g., reading data from a device, updating device state).
    4.  **End of Interrupt:** After processing, the user-space driver may need to inform the kernel to re-enable the IRQ at the controller (especially for level-triggered interrupts or if it was masked by the kernel). This could be done via an invocation on the `IRQ Handler Object`.
    This two-stage process ensures that the time spent in privileged kernel mode for interrupt handling is minimal and bounded. Interrupt masking and unmasking at the hardware level will be managed by the kernel, coordinated with user-space driver requests via the IRQ Handler Object.
## 4.2. Inter-Process Communication (IPC) Mechanism Specification
Inter-Process Communication (IPC) is fundamental to grx, as most system services operate in user space and interact via IPC. The design prioritizes security, performance (low latency, high throughput for small messages), and determinism.

    **Core IPC Operations:**
    As defined in Section 4.1.1, the primary IPC operations are:
    -   `grx_IPC_Call(target_cap, msg_info, &reply_info)`: Synchronous send and receive.
    -   `grx_IPC_Send(target_cap, msg_info)`: Asynchronous/non-blocking send.
    -   `grx_IPC_Recv(endpoint_cap, &msg_info, &sender_info)`: Blocking receive on an endpoint.
    -   `grx_IPC_Reply(reply_cap, msg_info)`: Reply to a synchronous call.

    **Message Structure (`msg_info`):**
    IPC messages in grx are designed to be efficient and flexible, supporting both small data transfers directly and larger transfers indirectly, along with secure capability passing. A `msg_info` structure, passed via registers or the stack, will define the message content:

    1.  **Message Tag/Label:** A word-sized field that can be used by applications to identify the type of message or method being invoked. The kernel treats this opaquely for synchronous calls but may inspect it for kernel-defined protocols.
    2.  **Direct Payload Registers (MRs - Message Registers):** A small number of machine word-sized registers (e.g., 4-8 words) for passing small data items directly. This is extremely fast as it avoids memory access. (Inspired by seL4 Message Registers).
    3.  **Capability Slots:** A fixed number of slots (e.g., 1-4) within the message structure dedicated to transferring capabilities. When a capability is placed in a slot and an IPC operation is performed:
        *   The kernel validates the source capability.
        *   If valid, the kernel transfers this capability to the receiving thread's CSpace, placing it either in a predefined receive slot or as specified by the receiver.
        *   The message register corresponding to the slot will indicate the type or presence of the transferred capability.
    4.  **Extended Data Transfer (Indirect):** For larger data transfers, capabilities to shared memory regions (e.g., `grx_MemoryObject` instances, see Section 4.1.2) are passed via the capability slots. Cooperating components then read/write directly to this shared memory. The message itself might contain offsets and lengths pertaining to the data within the shared region. This approach avoids large data copies through the kernel.

    **Endpoints:**
    -   `Endpoint` objects (see Section 4.1.2) are the kernel-managed entities to which messages are sent and from which messages are received when using `grx_IPC_Recv`.
    -   Threads require a capability to an Endpoint to send to it or receive from it.
    -   Endpoints can be configured with badges or properties to allow servers to manage multiple client interactions over a single endpoint.

    **IPC Performance Considerations:**
    -   The IPC path, particularly `grx_IPC_Call`, will be aggressively optimized for minimal latency, targeting worst-case execution times suitable for hard real-time systems.
    -   The kernel will avoid dynamic memory allocation during IPC operations.
    -   Direct register usage for small payloads and capability-based shared memory for larger data ensure efficient data transfer.

    This IPC design provides a robust foundation for building a secure, component-based operating system where services are isolated yet can communicate efficiently.
## 4.3. Memory Management Specification
grx employs a capability-based memory management system designed for security, flexibility, and determinism, drawing inspiration from seL4's untyped memory model and Zircon's VMOs. All kernel memory is statically allocated at boot time (see Section 2.1.2); this section details user-space memory management.

    **Core Concepts:**

    1.  **`UntypedMemory` Objects:**
        *   At boot, the system's available physical memory is primarily represented by `UntypedMemory` kernel objects. Capabilities to these objects are initially granted to a privileged user-space Memory Manager component.
        *   `UntypedMemory` represents raw, unallocated physical memory. It cannot be directly mapped or used by processes.

    2.  **Retyping `UntypedMemory`:**
        *   The Memory Manager component is responsible for carving out memory for specific uses. It does this by invoking the `Untyped_Retype` method (via `grx_KernelObject_Invoke`) on an `UntypedMemory` capability.
        *   This operation allows the Memory Manager to create other kernel objects from the raw memory, such as:
            *   Other kernel objects like TCBs, Endpoints, CNodes, etc.
            *   Fixed-size `grx_MemoryObject` instances (see below).
        *   This retyping mechanism is the foundation of all dynamic resource allocation in user space, ensuring that all memory is accounted for and its type is explicitly defined by a privileged component.

    3.  **`grx_MemoryObject` (VMO-like Objects):**
        *   A `grx_MemoryObject` represents a contiguous region of physical memory of a specific size (e.g., a collection of pages). These are created by retyping `UntypedMemory`.
        *   These objects are page-aligned and represent the actual frames of memory that can be mapped into a process's address space.
        *   They are analogous to Zircon's Virtual Memory Objects (VMOs) or physical memory frames in other systems.
        *   Capabilities to `grx_MemoryObject` instances can be transferred between processes via IPC, allowing for shared memory.

    4.  **`AddressSpace` Objects (Virtual Address Management):**
        *   Each process is associated with an `AddressSpace` kernel object, which represents its virtual memory map (e.g., page tables).
        *   To make memory usable by a process, a `grx_MemoryObject` must be mapped into its `AddressSpace`.
        *   This is achieved by invoking a method like `AddressSpace_MapObject` (via `grx_KernelObject_Invoke`) on the `AddressSpace` capability. This call would specify:
            *   The capability to the `grx_MemoryObject` to map.
            *   The desired virtual address (or allow the kernel to choose).
            *   The access permissions (e.g., Read, Write, Execute) for the mapping.
            *   Offset and length within the `MemoryObject`.
        *   The kernel is responsible for updating the MMU/MPU configuration based on these mapping operations.
        *   Unmapping is performed similarly via an `AddressSpace_UnmapObject` method.

    **Memory Protection:**
    -   The MMU/MPU hardware enforces memory protection based on the mappings in each process's `AddressSpace`.
    -   Attempting to access memory outside of explicitly mapped regions, or with incorrect permissions, will result in a fault handled by the kernel (typically by sending an exception message to a designated fault handler for the process).

    **Benefits of this Approach:**
    -   **Explicit Resource Management:** All memory is explicitly allocated and typed by a Memory Manager.
    -   **Security:** Capabilities control who can create, map, and share memory.
    -   **Flexibility:** Supports various memory sharing patterns and fine-grained permission control.
    -   **Determinism:** Kernel operations for mapping/unmapping are designed to be bounded, and the absence of dynamic kernel allocation for these structures contributes to predictability.
## 4.4. User-Space Component Framework Specification
grx will feature a robust user-space component framework that defines how applications and system services are structured, isolated, and managed. This framework is crucial for modularity, security, and maintainability, drawing inspiration from established systems like Genode's recursive component model and Zircon's Component Framework.

    **Core Concepts:**

    1.  **Component as Unit of Isolation:**
        *   A component is the fundamental unit of execution and isolation in user space. Each component runs in its own isolated protection domain (i.e., its own `AddressSpace` and CSpace).
        *   Components can range from simple device drivers and system services to complex applications.

    2.  **Component Declaration (Manifest):**
        *   Each component will be defined by a manifest file (e.g., using a simple, well-defined XML or JSON format). This manifest will declare:
            *   **Executable:** The path to the component's binary executable.
            *   **Required Capabilities:** A list of capabilities the component needs to function, specifying their type (e.g., service endpoint, memory region, IRQ handler) and how they are to be acquired (e.g., from parent, from a specific system service by name/ID).
            *   **Provided Services/Interfaces:** A declaration of services or interfaces the component offers to others, typically identified by an interface type or a named `Endpoint` capability it will create and publish.
            *   **Resource Quotas:** Initial memory allocation requirements, CPU time share or priority group, and other resource limits.
            *   **Configuration Data:** Optional component-specific configuration parameters.

    3.  **Component Lifecycle Management:**
        *   **Instantiation & Startup:** Components are instantiated by a parent component, or by a dedicated 'Component Manager' service (which itself is a component). The instantiating entity reads the child's manifest, creates the necessary kernel objects (AddressSpace, root CNode, initial TCB), allocates initial resources (as per the manifest and parent's budget), and provides the declared capabilities. It then starts the component's main thread.
        *   **Running State:** Once started, a component executes its defined function, interacting with other components and the kernel via IPC and capabilities.
        *   **Termination & Cleanup:** Components can terminate voluntarily or be terminated by a privileged parent/manager. Upon termination, their resources (memory, capabilities they exclusively hold) must be reclaimed by the system. The kernel will assist in this by invalidating capabilities rooted in the terminated component's CSpace.

    4.  **Inter-Component Communication:**
        *   Communication between components will primarily occur via the grx IPC mechanisms detailed in Section 4.2.
        *   Components discover each other's services through capabilities to Endpoints, which might be obtained from a naming service, passed by a mutual parent, or through a structured discovery mechanism.

    5.  **Interface Definition:**
        *   To ensure clear contracts between components, an Interface Definition Language (IDL) is strongly recommended. An IDL would allow for the formal specification of services, methods, and data types used in IPC, enabling automated generation of client stubs and server skeletons. This promotes modularity and simplifies integration. (Inspired by practices in systems like Fuchsia/Zircon and Genode).

    **Benefits:**
    -   **Modularity:** Components can be developed, tested, and updated independently.
    -   **Security:** Isolation and explicit capability requirements enforce the Principle of Least Privilege.
    -   **Fault Containment:** Failures in one component are less likely to affect others.
    -   **Resource Control:** Manifests and hierarchical instantiation allow for explicit resource allocation and budgeting.
## 4.5. Bootloader Design Specification
The grx bootloader is a critical component for establishing a secure and trusted system startup. It will implement a multi-stage process with cryptographic verification at each step, drawing inspiration from robust secure bootloaders like MCUBoot (used by Zephyr and others).

    **Boot Stages:**

    1.  **Stage 0: ROM Bootloader (RBL) / Primary Bootloader (PBL)**
        *   **Location:** Typically on-chip, immutable Read-Only Memory (ROM) or One-Time Programmable (OTP) memory, provided by the SoC vendor. This is the hardware root of trust.
        *   **Responsibilities:**
            *   Minimal hardware initialization (e.g., basic CPU setup, security peripherals).
            *   Locate the Secondary Bootloader (SBL) in non-volatile storage (e.g., flash memory).
            *   Verify the integrity and authenticity of the SBL using a public key securely stored in eFuses or OTP memory.
            *   If verification succeeds, load the SBL into RAM and transfer execution.
            *   If verification fails, enter a defined error state (e.g., halt, recovery mode if supported).
        *   **Security:** Its immutability is key. Its public key(s) for SBL verification must be securely provisioned during manufacturing.

    2.  **Stage 1: Secondary Bootloader (SBL)**
        *   **Location:** Non-volatile flash memory, verified by the RBL.
        *   **Responsibilities:**
            *   Initialize essential hardware required for kernel boot (e.g., RAM controller, clock system, console for early diagnostics).
            *   Locate the grx kernel image (and potentially an initial ramdisk or component configuration data if used) in flash.
            *   **Image Verification:** Cryptographically verify the integrity and authenticity of the kernel image. This involves:
                *   Hashing the image (e.g., using SHA-256 or SHA-384).
                *   Verifying a digital signature (e.g., ECDSA P-256/P-384 or EdDSA - Ed25519) against a public key embedded within the SBL (or loaded from a secure partition).
            *   **Update Handling (A/B Partition Support):** If an A/B update mechanism is in place (see Section 2.3.6), the SBL will:
                *   Determine the active partition to boot from based on status flags (e.g., update successful, trial boot).
                *   Perform image verification for the selected partition.
                *   Potentially roll back to the previously known-good partition if the selected one fails verification or if a trial boot was unsuccessful.
            *   If verification is successful, load the kernel image into RAM.
            *   Prepare boot parameters or a device tree blob (DTB) to pass to the kernel.
            *   Transfer execution to the grx kernel entry point.
            *   If verification fails, enter a defined error state (e.g., recovery mode, signal error to user).
        *   **Security:** The SBL must be minimal to reduce its own attack surface. It should be updatable via a secure process if necessary, although this is less frequent than kernel/application updates.

    3.  **Stage 2: Kernel Initialization**
        *   The grx kernel takes over, as described in Section 2.2.3 of the OS Kernel Analysis document and subsequent sections here. It relies on the SBL having set up a valid and secure environment.

    **Cryptographic Algorithms:**
    -   **Hashing:** SHA-256 or SHA-384 will be used for image integrity checks.
    -   **Signatures:** ECDSA with NIST P-256 or P-384 curves, or EdDSA (e.g., Ed25519) for image authenticity. The choice will depend on hardware acceleration availability and security requirements.

    **Key Management:**
    -   **RBL Public Key(s) for SBL:** Stored in immutable hardware (e.g., eFuses, OTP).
    -   **SBL Public Key(s) for Kernel:** Embedded within the SBL or stored in a secure, authenticated storage region accessible by the SBL.
    -   Private keys used for signing images must be strictly protected in the build/signing infrastructure.
    -   Key revocation mechanisms should be considered as part of the overall security lifecycle.

    **Boot Time Budget:**
    -   The RBL phase should be extremely fast (e.g., tens of milliseconds).
    -   The SBL phase (including hardware init and image verification) should target completion within a few hundred milliseconds (e.g., < 200-300ms) to contribute to the overall system boot time target of < 500ms. Optimization of cryptographic operations (e.g., using hardware acceleration if available) is crucial.

    This structured boot process ensures a chain of trust from hardware power-on to kernel execution, which is essential for meeting grx's security and safety goals.
## 4.6. Hardware Abstraction Layer (HAL) / Board Support Package (BSP) Strategy
To ensure portability across different hardware platforms while maintaining a clean architecture, grx will define a clear Hardware Abstraction Layer (HAL) and a strategy for Board Support Packages (BSPs). This approach is inspired by the layered driver models seen in systems like Zephyr.

    **Hardware Abstraction Layer (HAL):**

    *   **Purpose:** The HAL provides a standardized, thin software layer that abstracts the core CPU architecture and essential on-chip peripheral functionalities for the grx kernel and low-level user-space drivers. It isolates the core OS from hardware specifics.
    *   **Scope:**
        *   **CPU Architecture Specifics:** Context switching primitives, interrupt/exception entry/exit routines, MMU/MPU control (e.g., page table manipulation, cache operations), atomic operations, CPU clock control.
        *   **Core Peripherals:** System timer, interrupt controller(s), basic serial console (for early boot/debug).
        *   **Boot Interface:** Functions required by the kernel during early boot (e.g., querying memory map from bootloader).
    *   **Characteristics:**
        *   The HAL is architecture-specific (e.g., a HAL for ARMv8-A, another for RISC-V 64-bit).
        *   It exposes a C API for the kernel and potentially for very low-level drivers that might need direct hardware access (though most drivers will be higher-level user-space components).
        *   It is designed to be minimal and efficient, adding negligible overhead.

    **Board Support Package (BSP):**

    *   **Purpose:** A BSP provides all additional software required to support a specific development board, System-on-Chip (SoC), or custom hardware. It builds upon the HAL.
    *   **Scope:**
        *   **Device Drivers:** Implementations of user-space drivers for board-specific peripherals not covered by the HAL (e.g., specific Ethernet MAC/PHYs, sensors, display controllers, audio codecs, flash memory controllers). These drivers adhere to the grx user-space driver model (Section 2.2.1).
        *   **Hardware Configuration:**
            *   **Device Tree (or similar mechanism):** A hardware description format (e.g., Devicetree, similar to its use in Linux and Zephyr) will be used to describe the hardware layout, peripheral configurations, interrupt routing, and resource assignments on a specific board. This allows drivers to be configured dynamically based on the board's hardware.
            *   **Clock Configuration:** Initialization code for the system's clock tree.
            *   **Pin Multiplexing:** Configuration for SoC pin functions.
            *   **Memory Layout:** Definitions of physical memory regions, RAM size, and any board-specific memory-mapped I/O.
        *   **Bootloader Integration:** May include board-specific elements or configurations needed by the SBL (Secondary Bootloader).
        *   **Example Applications/Tests:** Board-specific examples demonstrating peripheral usage.

    **Relationship between HAL, BSP, and Drivers:**
    -   The grx kernel interfaces with the HAL for core operations.
    -   User-space device drivers (part of a BSP or provided as separate components) interact with hardware. For on-chip peripherals already abstracted by the HAL (like the interrupt controller), drivers might use HAL-provided user-space shims if direct kernel interaction is not feasible. For other peripherals, drivers will directly program device registers.
    -   The Devicetree (or chosen alternative) provides the configuration data that links specific drivers to specific hardware instances and their resources.

    This layered approach promotes code reuse for the core OS (kernel + HAL for a given architecture) while enabling straightforward adaptation and support for new hardware platforms through well-defined BSPs and a descriptive hardware configuration mechanism.

# 5. Development and Quality Assurance Process
## 5.1. Development Methodology
grx will adopt a hybrid development methodology that incorporates the rigor of the V-model for safety-critical aspects, particularly for the microkernel and core safety components, while allowing for Agile/iterative practices for user-space component development and non-critical features. The overarching process will ensure traceability and compliance with standards like IEC 61508 and ISO 26262. (see [Development Model Overview](../../docs/process/development_model_overview.md), [V-Model Process Guide](../../docs/process/v_model_process_guide.md), and [Agile Sub-Efforts Guide](../../docs/process/agile_sub_efforts_guide.md))
        - **Safety Note:** Strict adherence to all phases of the V-model, including comprehensive hazard analysis and risk assessment, derivation of safety requirements, and rigorous, independent verification and validation at each stage, is fundamental for achieving functional safety certifications.

    **Core V-Model Structure for Safety-Critical Components (e.g., Microkernel):**

    This model emphasizes verification and validation at each stage of development:

    1.  **Concept of Operations & System Requirements Definition:**
        *   **Activities:** Define overall system goals, operational scenarios, hazard analysis, and derive top-level safety and security requirements for grx.
        *   **Deliverables:** System Requirements Specification, Hazard and Risk Analysis (HARA) document, Initial Safety Concept.
        *   **Verification:** Peer reviews, stakeholder agreement.
        *   **Validation (against):** User/Stakeholder Needs.

    2.  **Software Requirements Specification (SRS):**
        *   **Activities:** Detail specific functional and non-functional requirements for software components (e.g., kernel modules, critical drivers), including safety and security requirements allocated from the system level. Define interfaces between software components.
        *   **Deliverables:** Software Requirements Specification (for each component), Interface Control Document (ICD).
        *   **Verification:** Peer reviews, traceability to System Requirements.
        *   **Validation (against):** System Requirements.

    3.  **Software Architecture Design:**
        *   **Activities:** Define the high-level structure of the software, major components, their interactions, and how they satisfy requirements. For grx, this includes defining kernel objects, IPC mechanisms, component framework structure, HAL/BSP strategy.
        *   **Deliverables:** Software Architecture Document (SAD), updated ICDs.
        *   **Verification:** Peer reviews, traceability to SRS, static analysis of architectural models (if used).
        *   **Validation (against):** Software Requirements.

    4.  **Detailed Design Specification:**
        *   **Activities:** Define low-level design details for each module/unit within a component: algorithms, data structures, function signatures, state machines.
        *   **Deliverables:** Detailed Design Documents (DDD) for each module/unit.
        *   **Verification:** Peer reviews, traceability to Architecture and SRS, static code analysis (on pseudocode or early code).
        *   **Validation (against):** Software Architecture.

    5.  **Implementation (Coding):**
        *   **Activities:** Write source code according to detailed design, coding standards (e.g., MISRA C/C++), and security best practices. Perform developer unit tests.
        *   **Deliverables:** Source code, unit test plans and reports.
        *   **Verification:** Code reviews, static analysis tool reports, adherence to coding standards.

    6.  **Unit Testing (Right side of V - corresponds to Detailed Design):**
        *   **Activities:** Test individual software units/modules in isolation to verify they meet their detailed design specifications.
        *   **Deliverables:** Unit Test Specification, Unit Test Results, Code Coverage reports (aiming for 100% statement/branch, and MC/DC for critical units).
        *   **Validation:** Verifies detailed design.

    7.  **Integration Testing (Right side of V - corresponds to Software Architecture):**
        *   **Activities:** Test integrated software components to verify their interactions and interfaces as defined in the software architecture.
        *   **Deliverables:** Integration Test Specification, Integration Test Results.
        *   **Validation:** Verifies software architecture and inter-component interfaces.

    8.  **Software System Testing (Right side of V - corresponds to Software Requirements):**
        *   **Activities:** Test the complete grx software system on target hardware (or close emulation) to verify it meets the specified software requirements (functional, performance, safety, security).
        *   **Deliverables:** System Test Specification, System Test Results.
        *   **Validation:** Verifies software requirements.

    9.  **System Acceptance & Safety Validation (Right side of V - corresponds to System Requirements/Concept):**
        *   **Activities:** Validate that the fully integrated grx (as part of a target system) meets the overall system requirements and safety goals in its operational environment. This often involves specific safety validation tests derived from the HARA and safety concept.
        *   **Deliverables:** Acceptance Test Plan/Results, Safety Validation Report.
        *   **Validation:** Verifies system requirements and safety concept.

    **Agile Adaptation for Non-Critical User-Space Components:**
    For less critical user-space components or during rapid prototyping phases, Agile methodologies (e.g., Scrum, Kanban) may be employed. However, these must be adapted:
    -   **Safety Backlog:** Safety requirements and hazard mitigations are explicitly included in the product backlog and sprint planning.
    -   **Definition of Done:** Must include necessary safety and security analyses, reviews, and testing appropriate for the component's criticality.
    -   **Iterative V&V:** Verification and validation activities are performed iteratively within or across sprints, ensuring continuous feedback.
    -   **Traceability:** Mechanisms to maintain traceability to higher-level requirements must be upheld even in Agile sprints.

    A tailored combination ensures both rigor for safety/security and flexibility for rapid development where appropriate.
## 5.2. Toolchain Specification
### 5.2.1. Compilers
- Specify exact versions of GCC/Clang (and Rust toolchain if applicable).
        Key compiler flags will be enforced across all C/C++ code for grx:
        - **General Safety & Quality:** `-Wall -Wextra -Wpedantic -Werror -std=c11` (for C) or `-std=c++17` (or later, for C++ if used for non-kernel components).
        - **Security:** `-fstack-protector-all` (or specific variants), `-D_FORTIFY_SOURCE=2`, Position Independent Executable (`-fPIE`, `-pie`) for user-space.
        - **MISRA Compliance:** Specific flags for the chosen MISRA checker/compiler (e.g., `-fmisra-c-2012` if supported, or flags for external checkers).
        - **Debugging:** `-O0 -g3` (full debug information, no optimization).
        - **Release/Optimized:** `-Os` (optimize for size) or `-O2`/`-O3` (optimize for speed, with careful validation of impact on predictability and safety). Specific optimization flags will be evaluated for their impact on WCET.
        - **Target Architecture Flags:** Specific flags for CPU architecture, FPU, and any required ABI.
        For Rust components (if used), `cargo build --release` with appropriate profile settings for optimization and safety checks (e.g., overflow checks enabled during development/testing).
### 5.2.2. Debuggers
- JTAG/SWD debuggers, kernel debuggers, user-space process debuggers.
        Standard debugging practices will include:
        - **OpenOCD Scripts:** Custom OpenOCD (or similar JTAG/SWD interface like SEGGER J-Link) scripts for target board initialization, flashing, and attaching GDB.
        - **GDB Extensions:** Potential development of custom GDB scripts or Python extensions for kernel-aware debugging, allowing inspection of:
            - TCB states and registers.
            - CSpace contents (listing capabilities).
            - Kernel object properties (e.g., Endpoint queues, Memory Object mappings).
            - Untyped memory status.
        - **Core Dump Analysis:** Mechanisms for capturing and analyzing core dumps from user-space components and potentially the kernel (in debug builds).
### 5.2.3. Static Analysis Tools
- Specify tools for code quality, security vulnerabilities (e.g., MISRA C compliance, Coverity, SonarQube). For Frama-C, specific plugins like Value Analysis (to detect runtime errors), E-ACSL (for formal specification and deductive verification via WP plugin), and EVA (Evolutionary Value Analysis) will be utilized. For MISRA C compliance, tools like Polyspace, Klocwork, or Parasoft C/C++test will be considered in addition to compiler-based checks.
### 5.2.4. Formal Verification Tools
- If formal verification is pursued, specify the provers and model checkers. For grx, this would likely involve tools such as Isabelle/HOL (used for seL4) or other interactive theorem provers, targeting the core microkernel. The scope of formal verification will aim to prove:
        - **Isolation Properties:** Non-interference between components (spatial and temporal, given correct configuration).
        - **API Correctness:** Key system calls and kernel object methods behave as specified (e.g., `grx_IPC_Call` correctly transfers data/capabilities, `Untyped_Retype` correctly creates objects).
        - **Security Properties:** Integrity of the capability system (capabilities cannot be forged or arbitrarily modified by unprivileged components).
        - **Safety Properties:** Freedom from runtime errors in the kernel (e.g., null pointer dereferences, buffer overflows, deadlocks for core locking primitives).
        - **Binary Correctness (Selective):** For critical, small functions, proof of correctness of the compiled machine code with respect to source or formal specification.
### 5.2.5. Real-Time Analysis Tools
- WCET analysis tools, schedulability analysis tools.
        Inputs for WCET analysis tools will include:
        - Compiled binary code for the specific task/function.
        - A detailed model of the target CPU core (pipeline, caches, branch prediction).
        - Annotations for loop bounds, recursion depth (recursion generally disallowed in kernel), and infeasible paths.
        Inputs for schedulability analysis (e.g., using tools based on Response Time Analysis or similar formalisms) will include:
        - Task set: Periods, deadlines, WCETs for all critical tasks.
        - Scheduling algorithm parameters (e.g., priorities, time slice for RR).
        - Resource contention models (e.g., blocking times due to shared resources, IPC delays).
### 5.2.6. Build System
- Define the build system (e.g., CMake, custom build scripts) and configuration system (e.g., Kconfig). The build system should integrate Kconfig-like configuration (similar to Zephyr or Linux kernel) for managing compile-time features and modules, and utilize build tools like CMake or Meson for managing the compilation process.
        A typical Kconfig structure might include top-level menus for:
        - `Kernel Type` (e.g., Debug, Release, Verified_subset)
        - `Architecture Options` (e.g., specific CPU, FPU support)
        - `Kernel Features` (e.g., MCS support, number of priority levels, specific syscalls enabled/disabled)
        - `Device Drivers` (selecting which drivers to build for a BSP)
        - `Services` (selecting which user-space system services to include)
        The CMake/Meson project structure will feature a top-level file defining common build options and recursively include build files from kernel, HAL, libraries, services, and application component directories. It will support cross-compilation and different build configurations (Debug, Release).
## 5.3. Testing Strategy
### 5.3.1. Unit Testing
- 100% code coverage for critical kernel components.
        **Framework:** A framework like CMocka or Google Test (potentially adapted for embedded/kernel environments) will be used. For kernel code, unit tests may run in an emulated environment (e.g., QEMU) or on host with HAL/kernel stubs.
        **Metrics:** Beyond 100% statement and branch coverage for critical modules, Modified Condition/Decision Coverage (MC/DC) will be targeted for safety-critical functions as per DO-178C Level A objectives or equivalent safety standards. Static analysis of cyclomatic complexity will aim to keep units testable.
        - **Safety Note:** The choice of code coverage targets (statement, branch, MC/DC) must align with the integrity levels (SIL/ASIL) of the software units being tested, as mandated by relevant safety standards.
### 5.3.2. Integration Testing
- Verify inter-component communication and system services.
        **Approach:** A combination of bottom-up and top-down integration will be used.
        - *Bottom-up:* Lower-level components (e.g., HAL, kernel core primitives, basic drivers) are integrated and tested first, using test harnesses that simulate higher-level callers.
        - *Top-down:* Higher-level services are tested by integrating them with already tested lower-level components, using stubs for peer or external services not yet available.
        **Test Harness:** The test harness will provide capability setup, IPC message injection/capture, event simulation, and result verification. It may run on-target or in a high-fidelity emulator. For IPC-based services, the harness will act as a client sending test messages and verifying responses against expected outcomes, including capability transfers.
### 5.3.3. System Testing
- End-to-end functional, performance, safety, and security testing.
        Key system-level test scenarios will include (but not be limited to):
        - **Boot & Initialization:** Verification of secure boot chain, correct initialization of all kernel objects and system services as per configuration.
        - **Core Service Functionality:** End-to-end tests for process creation, memory allocation, IPC-based service requests (e.g., file I/O, network communication if available).
        - **Resource Management:** Stress tests for resource allocation (memory, capabilities, CPU time) and deallocation, checking for leaks or starvation.
        - **Concurrency & Scheduling:** Tests involving multiple threads with varying priorities, IPC patterns, and resource sharing to verify scheduler correctness and absence of deadlocks/livelocks.
        - **Interrupt Handling:** Verification of correct interrupt delivery to user-space handlers for all supported interrupt sources under various conditions.
        - **Fault Tolerance:** Tests based on fault injection (see 5.3.5) verifying system stability and recovery mechanisms.
        - **Security Policy Enforcement:** Tests to verify that the capability system and other security mechanisms correctly prevent unauthorized access or operations.
        - **Power Management Transitions:** (If applicable) Correct entry/exit from low-power states.
### 5.3.4. Regression Testing
- Automated suite to prevent regressions.
### 5.3.5. Fault Injection Testing
- Deliberately inject faults (e.g., memory errors, timing faults) to test fault tolerance and recovery. This may involve software-implemented fault injection (SWIFI) techniques and potentially hardware-based fault injection for more rigorous testing.
        Specific fault types to be injected include:
        - **Memory Faults:** Bit-flips in RAM (kernel and user-space), invalid memory pointers, buffer overflows (where not prevented by language choice).
        - **CPU Faults:** Simulated register corruption, instruction decoder errors (if possible via specific debug features).
        - **IPC Faults:** Corrupted message tags or data, invalid capabilities in messages, dropped/delayed messages (simulated by test harness).
        - **API Call Failures:** Forcing error return codes from kernel syscalls or critical service APIs to test client error handling.
        - **Timing Faults:** Introducing unexpected delays in task execution or interrupt arrivals.
        Faults will be injected using SWIFI (Software-Implemented Fault Injection) by modifying code or data, or via debug interfaces. Hardware fault injection (e.g., pin-level manipulation, voltage/clock glitching) may be considered for higher safety levels.
        - **Safety Metric:** Percentage of implemented safety mechanisms whose effectiveness has been verified through fault injection testing.
        - **Safety Metric:** Ratio of injected faults correctly detected and handled by safety mechanisms versus the total number of applicable injected faults.
        - **Safety Note:** Fault injection campaigns are essential for empirically verifying the effectiveness of error detection, fault tolerance, and fault recovery mechanisms described in the safety concept. Test cases should simulate realistic fault scenarios.
### 5.3.6. Security Testing
- Fuzzing, penetration testing, vulnerability scanning.
        - **Cybersecurity Metric:** Code coverage achieved by fuzz testing for targeted interfaces, parsers, and syscalls.
        - **Cybersecurity Metric:** Number and severity of security vulnerabilities identified and remediated through each testing phase (static analysis, fuzzing, penetration testing).
        - **Cybersecurity Note:** Security testing shall include tests for known vulnerability classes (e.g., OWASP Top 10 for Embedded Systems, CWE Top 25), specific threats identified in the system's threat model, and robustness against unexpected or malformed inputs.
        - **Cybersecurity Consideration:** Where possible, security testing should be automated and integrated into the CI/CD pipeline.
        **Target Areas for Fuzzing:**
        - Kernel syscall interface: `grx_KernelObject_Invoke` with varied method IDs and arguments.
        - IPC message parsing in all user-space services.
        - Manifest file parsing by the Component Manager.
        - Network protocol parsing (if a network stack is implemented).
        - Filesystem metadata parsing.
        **Tools/Methodologies:** Tools like AFL (American Fuzzy Lop) adapted for the grx environment, or custom fuzzers for specific IPC interfaces. Penetration testing will involve manual and automated attempts to bypass security controls, escalate privileges, and cause denial of service, following methodologies like those in the OSSTMM (Open Source Security Testing Methodology Manual) where applicable.
### 5.3.7. Real-Time Performance Testing
- Stress testing under peak load, latency and jitter measurements.
## 5.4. Continuous Integration/Continuous Deployment (CI/CD)
A robust CI/CD pipeline is essential for maintaining code quality, ensuring regressions are caught early, and enabling reliable, repeatable builds and deployments. The grx CI/CD pipeline, likely implemented using tools like Jenkins, GitLab CI/CD, or GitHub Actions, will include the following stages and gates for every proposed change (e.g., merge request/pull request) and for nightly/release builds:

    1.  **Pre-Commit Checks (Developer Environment):**
        *   Execution of linters (e.g., `clang-format`, `cpplint`, `rustfmt`).
        *   Local execution of relevant unit tests.

    2.  **CI Stage 1: Build & Basic Static Analysis:**
        *   **Gate:** Code successfully compiles for all target architectures and defined build configurations (e.g., Debug, Release, specific Kconfig options).
        *   **Gate:** Basic static analysis (compiler warnings as errors, linters) passes with zero violations.
        *   Execution of fast-running, core unit tests.

    3.  **CI Stage 2: Comprehensive Static Analysis & Unit Testing:**
        *   **Gate:** In-depth static analysis (e.g., using tools like Clang Static Analyzer, SonarQube, Frama-C Value Analysis on selected modules) passes defined quality gates (e.g., no new critical/major issues, complexity metrics within bounds).
        *   **Gate:** Full unit test suite execution (for all modules affected by the change) passes with 100% success.
        *   **Gate:** Code coverage from unit tests meets or exceeds a defined threshold (e.g., 90% statement/branch for most code, striving for 100% for critical kernel code). Coverage reports are generated and archived.

    4.  **CI Stage 3: Integration & System Testing (Emulated/Simulated):**
        *   Build of system images for emulated environments (e.g., QEMU with various hardware models).
        *   **Gate:** Execution of integration tests (IPC, basic service interactions) in the emulated environment passes.
        *   **Gate:** Execution of a core subset of system tests (boot sequence, critical service functionality, basic security checks) in the emulated environment passes.
        *   Performance benchmark execution on emulated targets to detect major regressions.

    5.  **CI Stage 4: On-Target Testing (Hardware-in-the-Loop - HIL):**
        *   (For key reference hardware platforms, triggered on a less frequent basis for typical commits, but mandatory for release candidates)
        *   Deployment of system images to target hardware.
        *   **Gate:** Execution of a comprehensive suite of system tests, including real-time performance tests, stress tests, and peripheral interaction tests on actual hardware, passes.
        *   Fault injection test suite execution on target hardware.

    6.  **CI Stage 5: Documentation & Release Packaging:**
        *   Automated generation of API documentation (e.g., from Doxygen, rustdoc, IDL tools).
        *   Build and packaging of release artifacts (kernel image, user-space components, bootloaders, SDK, documentation).
        *   Generation of traceability matrices and test summary reports.

    7.  **CD Stage (Continuous Deployment/Delivery - for specific environments/releases):**
        *   Automated deployment to development/testing hardware.
        *   For official releases: Secure signing of release artifacts.
        *   Publishing of release notes, artifacts, and documentation.

    **Pipeline Triggers:**
    -   Every commit to a development branch triggers Stages 1-3.
    -   Merge requests/pull requests to main/release branches require successful completion of Stages 1-4 (or a defined subset for expediency, with full suite run post-merge).
    -   Nightly builds trigger the full suite (Stages 1-5).
    -   Release tagging triggers Stages 1-7.

    The CI/CD pipeline will be version-controlled itself and subject to quality assurance.
        - **Safety Metric:** Percentage of safety requirements covered by validated test cases (Target: 100%).
        - **Safety Consideration:** Test environments for safety-critical software should replicate the target hardware and software environment as closely as possible.

# 6. Compliance and Certification Strategy
## 6.1. Target Standards
- Explicitly state the target functional safety and cybersecurity standards and their specific integrity levels (e.g., IEC 61508 SIL3, ISO 26262 ASIL B/C/D, DO-178C Level A, Common Criteria EAL4+).
        **Example: Addressing IEC 61508 SIL3 Clauses with grx:**
        -   **IEC 61508-3, Clause 7.4 - Software design and development:**
            -   *7.4.2.2 (Modularity):* Addressed by grx's microkernel architecture, component-based design (Section 1.4.2, 4.4), and IDL-defined interfaces (Section 4.4.5), promoting independent components with low coupling.
            -   *7.4.2.8 (Error detection and handling):* Addressed by kernel-enforced fault isolation (Section 2.4.1), memory protection (Section 2.1.2), capability-based security (Section 2.3.3), and defined fault recovery strategies (Section 2.4.4).
            -   *7.4.2.10 (Use of suitable programming languages):* Supported by encouraging memory-safe languages like Rust for user-space components (Section 2.3.5) and strict coding standards (e.g., MISRA C) for kernel code.
        -   **IEC 61508-3, Clause 7.5 - Software module testing:**
            -   *7.5.2.2 (Test coverage):* Addressed by targeting 100% statement/branch and MC/DC coverage for critical modules (Section 5.3.1).
            -   *7.5.2.7 (Testing of interfaces):* Addressed through rigorous integration testing (Section 5.3.2) using IPC test harnesses.
        (Similar analysis would be performed for other relevant standards and clauses.)
## 6.2. Safety Case and Security Case
- The structure and content of the safety case and security case documents will demonstrate compliance.
        - **Safety Note:** The Safety Case is a structured argument, supported by a body of evidence, that demonstrates that grx (or a system built upon it) achieves an acceptable level of safety for its intended operational context. It must be actively developed and maintained throughout the system lifecycle.
        - **Safety Note (CCF):** The grx Safety Case will document assumptions regarding internal CCF mitigation within the OS's scope and clearly delineate the system integrator's responsibilities for addressing broader system-level CCFs.
        **Example Table of Contents for a grx Safety Case (aligned with IEC 61508):**
        1.  Introduction (Purpose, Scope, System Overview)
        2.  Quality Management System (Compliance with IEC 61508-1, Cl. 6)
        3.  Safety Lifecycle Management (Compliance with IEC 61508-1, Cl. 7)
        4.  System Definition & Hazard Analysis (Derived from target application)
        5.  Overall Safety Requirements (Derived for grx based on application needs)
        6.  grx Architecture and Design for Safety
            6.1. Architectural Design Specification (Reference to this document, SAD)
            6.2. Hardware Safety Requirements (for underlying hardware)
            6.3. Software Safety Requirements Specification (for grx components)
            6.4. Design Measures for Fault Avoidance (e.g., coding standards, formal methods)
            6.5. Design Measures for Fault Control (e.g., partitioning, error detection, recovery)
        7.  Software Development (Compliance with IEC 61508-3)
            7.1. Detailed Design Documentation
            7.2. Implementation (Source code, build process)
        8.  Verification and Validation
            8.1. Software Module Testing (Unit tests, coverage)
            8.2. Software Integration Testing
            8.3. Software Safety Validation Testing (System tests mapped to safety reqs)
            8.4. Formal Verification Arguments (if applicable)
        9.  Configuration Management (as per Section 7.4)
        10. Documentation
        11. Operational and Maintenance Procedures (Guidance for integrators)
        12. Conclusion (Argument for achieved SIL)

        A similar detailed Table of Contents will be developed for the Security Case, likely aligning with Common Criteria structure (e.g., Introduction, Conformance Claims, Security Problem Definition, Security Objectives, Security Functional Requirements, Security Assurance Requirements, Rationale).
## 6.3. Traceability
- Implement rigorous traceability from high-level requirements down to code, tests, and verification results.
        **Tools/Methods:**
        -   A dedicated requirements management tool (e.g., Jama Connect, Polarion, DOORS) will be preferred for complex projects.
        -   Alternatively, a structured approach using version-controlled documents (e.g., Markdown, AsciiDoc) with unique IDs for each requirement, linked via scripts or a traceability matrix maintained in a spreadsheet/database.
        -   Source code comments (e.g., `//! Implements Req-ID-XYZ`) and version control commit messages will reference requirement IDs.
        -   Test case management tools will link test cases to requirements.
        - **Safety Metric:** Percentage of safety requirements that have complete bidirectional traceability to and from design elements, implementation artifacts, and verification/validation evidence. (Target: 100%).
## 6.4. Evidence Generation
- Define all required evidence (e.g., design documents, code reviews, test reports, analysis reports, formal proofs) for certification.
        **Specific Artifact Examples:**
        -   *Design Documents:* This Product Specification, Software Architecture Document, Detailed Design Documents for each kernel module and critical user-space component, HAL Specification, IPC Protocol Specification, Component Manifest Specification.
        -   *Code Reviews:* Documented checklists, review records per module/commit, static analysis reports.
        -   *Test Reports:* Unit Test Plans/Specifications/Reports (per module), Integration Test Plans/Specifications/Reports (per component interaction), System Test Plans/Specifications/Reports (per feature/requirement), Fault Injection Test Reports, Security Test (Fuzzing/Penetration) Reports.
        -   *Analysis Reports:* WCET Analysis Reports, Schedulability Analysis Reports, FMEA/FTA Reports for key safety functions, Formal Verification Proofs and associated documentation.
        -   *Configuration Management Records:* Version history for all artifacts, CM Plan, baseline definitions.
        -   *QMS Documents:* Process definitions, audit reports, training records.
## 6.5. Certification Body Engagement
- Plan for early and continuous engagement with relevant certification bodies.
## 6.6. Tool Qualification
- Qualify all development and verification tools used for safety/security-critical components. This is particularly critical for compilers, static analysis tools, and verification tools used for components targeting high SIL/ASIL/EAL levels, as per standards like IEC 61508-3, ISO 26262-8, or DO-178C.
        - **Safety Note:** For components targeting high SIL/ASIL levels, failure to properly qualify development and verification tools (or to use pre-certified tools) according to standards like IEC 61508-3 or ISO 26262-8 can significantly jeopardize certification efforts by invalidating the evidence produced by those tools.
## 6.7 Guidance for System Safety Integration
grx is designed as a foundational software element for safety-critical systems. While grx itself will undergo rigorous safety analysis for its defined capabilities and be delivered with a Safety Manual, system integrators are ultimately responsible for performing the overall Hazard Analysis and Risk Assessment (HARA) and Common Cause Failure (CCF) analysis for their specific application. This section outlines how grx features and documentation are intended to support these system-level safety activities.
### 6.7.1 Supporting Hazard Analysis and Risk Assessment (HARA)
- **grx Safety Manual:** A key deliverable for grx will be a Safety Manual. This document will provide crucial information for system integrators, including:
            - Detailed 'assumptions of use' for grx and its safety mechanisms.
            - Configurable parameters relevant to safety.
            - Known limitations and constraints.
            - Guidance on how grx features can be leveraged to meet system safety requirements.
            - High-level failure modes of core grx services and their expected behavior or detection by grx, to inform system FMEAs/FTAs.
- **Defined Safety Mechanisms:** Sections 2.4 (Functional Safety Features) and 3.5 (Safety Verifiable) describe the safety mechanisms within grx. System integrators should use this information to understand the scope, limitations, and configuration assumptions of these mechanisms when taking credit for them in their HARA.
- **Failure Mode Information:** The grx Safety Manual will provide information on the predicted failure modes of grx services (e.g., IPC, memory management, scheduling) and the error detection/reporting capabilities available to the application. This is intended to directly support the integrator's FMEA and FTA activities.
- **Configurability:** grx's configurable nature (e.g., via Kconfig as per 5.2.6, component manifests as per 4.4) allows integrators to tailor the OS to their specific needs, but this also means they must verify that their chosen configuration supports their safety goals.
### 6.7.2 Supporting Common Cause Failure (CCF) Analysis
- **Identification of Potential CCF Sources:** System integrators must analyze their complete system for CCFs. The grx Safety Manual will identify elements within or managed by grx that could potentially contribute to CCFs if not appropriately handled at the system design level. These include:
            - **Shared Hardware Resources:** While grx manages CPU cores, memory, and I/O, redundancy and diversity of these physical resources are system-level concerns for CCF mitigation (e.g., diverse CPU cores for highly critical redundant functions).
            - **Single Kernel Instance:** A fault in the single grx kernel instance could affect all software components. The use of formal verification for the kernel core aims to minimize this systematic risk. For extreme criticality, diverse redundant systems may be necessary.
            - **Core User-Space Services:** System-critical user-space services provided by grx (e.g., Root Resource Manager, Time Service) if designed as singletons, could represent a CCF vulnerability for dependent components. The Safety Manual will discuss the design of these services with respect to CCF.
            - **HAL/BSP:** Faults in common HAL or BSP drivers could affect multiple components relying on those drivers. Careful design and V&V of the HAL/BSP are important.
- **Partitioning as CCF Mitigation:** grx's support for strong spatial and temporal partitioning (Sections 2.4.5, 3.5.2) is a primary means to prevent CCFs between software components of different criticalities or those implementing redundant safety functions. System integrators are responsible for designing their component architecture and resource allocation to leverage these partitioning features effectively.
- **Diversity Considerations:** For the highest safety integrity levels, system integrators may need to implement diverse redundancy (e.g., different software implementing the same safety function, potentially on different hardware). grx aims to be a reliable platform for such architectures but does not inherently provide application-level software diversity.

# 7. Quality Management System (QMS) Requirements
## 7.1. Process Definition
- Document all development, testing, release, and maintenance processes in accordance with a recognized QMS (e.g., ISO 9001, CMMI) (detailed in the [Quality Management Plan](../../docs/plm/quality_management_plan.md)). The QMS will also incorporate specific process requirements from relevant functional safety (e.g., IEC 61508, ISO 26262) and cybersecurity standards.
        **Example: Change Control Process Steps:**
        1.  **Change Request (CR) Submission:** Any stakeholder can submit a CR detailing the proposed change, rationale, and affected components/documents. (Tool: Issue tracker like JIRA, GitLab Issues).
        2.  **Initial Review & Impact Analysis:** A designated lead/team reviews the CR for clarity and completeness. Technical leads perform an initial impact analysis (scope, effort, risk, safety/security implications).
        3.  **Change Control Board (CCB) Review:** The CCB (composed of key stakeholders like project management, technical leads, safety/security officers) reviews the CR and impact analysis.
        4.  **Decision:** CCB approves, rejects, or defers the CR. Approval includes assigning priority and target release/sprint.
        5.  **Implementation:** The assigned team implements the change according to development processes.
        6.  **Verification & Validation:** The change is tested (unit, integration, system tests as appropriate). Safety/security analyses are updated.
        7.  **Documentation Update:** All relevant documentation (requirements, design, test plans) is updated.
        8.  **Final CCB Approval for Merge/Release:** CCB reviews implementation and V&V results before the change is merged into the main branch or included in a release.
        9.  **Closure:** CR is formally closed.
## 7.2. Requirements Management
- Formal process for capturing, analyzing, tracing, and verifying requirements.
        Each requirement (system, software, safety, security) in the requirements management system will have at least the following attributes:
        -   **Unique ID:** A non-ambiguous identifier (e.g., SYS_REQ_001, SW_REQ_KERNEL_023, SAFETY_REQ_005).
        -   **Description:** Clear, concise statement of the requirement.
        -   **Rationale:** Justification for the requirement.
        -   **Source:** Origin of the requirement (e.g., standard clause, HARA, stakeholder request, derived from higher-level req).
        -   **Type:** Functional, Non-Functional, Safety, Security, Performance, Interface, etc.
        -   **Criticality/Priority:** (e.g., High/Medium/Low, or SIL/ASIL level for safety).
        -   **Verification Method:** How compliance will be verified (e.g., Test, Analysis, Inspection, Demonstration).
        -   **Verification Status:** (e.g., Not Verified, In Progress, Verified, Failed).
        -   **Traceability Links:** Links to parent/child requirements, design documents, test cases, and source code implementing it.
        -   **Version/Revision History:** Tracking changes to the requirement.
        -   **Status:** (e.g., Proposed, Approved, Implemented, Obsolete).
## 7.3. Design Control
- Structured design reviews, architectural decision records.
## 7.4. Configuration Management
- Version control for all source code, documentation, and build artifacts.
        **CM Baseline Strategy:**
        -   Baselines will be established at key project milestones, including: End of Software Requirements Phase, End of Architectural Design Phase, prior to each major/minor software release, and for all certification submission packages.
        -   A baseline will consist of a defined set of all versioned configuration items (CIs) – documents, source code, build scripts, tools, test procedures, and test results.
        **Version Control Branching Model (Git):**
        -   `main` or `master`: Represents the latest stable, released version. Highly protected.
        -   `develop`: Integration branch for upcoming releases. Features are merged here after review.
        -   `feature/<feature-name>`: Individual branches for new features or significant changes, branched from `develop`.
        -   `release/<version>`: Branched from `develop` when preparing for a new release. Only bug fixes and release-specific changes allowed.
        -   `hotfix/<issue-id>`: Branched from `main` for critical bug fixes on released versions.
        All merges into `develop` and `main` will require peer review and successful CI pipeline execution.
## 7.5. Change Control
- Formal process for managing changes to requirements, design, and code.
## 7.6. Defect Management
- Process for reporting, tracking, analyzing, and resolving defects.
## 7.7. Metrics and Reporting
- Define key quality metrics (e.g., code coverage, defect density, test pass rates, performance benchmarks) and reporting frequency.
        **Example Key Metrics & Targets:**
        1.  **Static Analysis Defect Density:**
            *   **Formula:** (Number of Critical/Major Static Analysis Warnings) / KLoC (thousands of lines of code).
            *   **Target:** < 0.5 for critical modules after remediation.
        2.  **Unit Test Code Coverage:**
            *   **Formula:** (Covered Statements/Branches/MC_DC_Pairs) / (Total Statements/Branches/MC_DC_Pairs) * 100%.
            *   **Target:** Statement/Branch: >95% for most modules, 100% for critical kernel code. MC/DC: >90% for safety-critical functions (as per SIL/ASIL target).
        3.  **Requirements Traceability:**
            *   **Formula:** (Number of Requirements with Bidirectional Traces to Design & Tests) / (Total Number of Requirements) * 100%.
            *   **Target:** 100% for all approved requirements.
        4.  **Open Critical/Major Bug Count:**
            *   **Formula:** Count of open bugs classified as Critical or Major.
            *   **Target:** 0 for any release candidate.
        Reporting frequency will be weekly for active development sprints/phases and monthly for overall project status.
## 7.8. Audits
- Plan for internal and external audits to ensure QMS compliance.

# 8. Maintenance and Evolution Strategy
## 8.1. Patching and Updates
- Define the process for issuing security patches and bug fixes, leveraging secure OTA mechanisms. This will use the A/B OTA update mechanism with rollback capabilities, as detailed in Section 2.3.6 and Section 4.5, ensuring a reliable update path.
        **Secure Update Package Format:**
        An update package will be a signed archive (e.g., using CMS - Cryptographic Message Syntax, or a custom secure format). It will contain:
        1.  **Manifest File:**
            *   `package_version`: Version of this update package.
            *   `target_grx_version`: The grx version this update applies to or creates.
            *   `compatibility_info`: Hardware revisions or existing software versions this package is compatible with.
            *   `image_list`: A list of images included in the package (e.g., kernel, SBL (if updatable), root filesystem, individual components). For each image:
                *   `image_name_id`: Identifier for the image.
                *   `image_version`: Version of this specific image.
                *   `image_hash`: Cryptographic hash (e.g., SHA-256/384) of the image file.
                *   `target_partition_name_or_id`: Where the image should be written (e.g., "kernel_A", "rootfs_B").
                *   `size`: Size of the image.
            *   `pre_install_script` (optional): Commands for the Update Manager before writing images.
            *   `post_install_script` (optional): Commands for the Update Manager after writing images but before reboot (e.g., data migration).
        2.  **Image Files:** The actual binary images listed in the manifest.
        3.  **Signature:** A digital signature (e.g., ECDSA P-384 or EdDSA) covering the entire manifest file (including image hashes). The public key for verifying this signature will be securely stored on the device, potentially updatable via a more privileged update mechanism.

        **Process to Trigger an Update (Example):**
        1.  **Availability Check:** The Update Manager component periodically checks a designated update server (via a secure connection like HTTPS) or a local source (e.g., USB) for new update packages.
        2.  **Download & Verification:** If a new package is found, the Update Manager downloads it to a temporary location. It then verifies the package's signature against its trusted public key(s) and verifies the integrity of each image file against the hashes in the manifest.
        3.  **Pre-Update Operations:** Executes the `pre_install_script` if present.
        4.  **Image Installation:** Writes each image to the designated inactive partition(s) as per the manifest (e.g., using the A/B scheme).
        5.  **Post-Update Operations:** Executes the `post_install_script` if present.
        6.  **SBL Notification:** The Update Manager updates the `grx_UpdateStatusBlock_t` (see Section 2.3.6) to indicate `update_target_partition` and set `attempt_status` to `BOOT_STATUS_TRY_UPDATE`.
        7.  **System Reboot Request:** The Update Manager requests a system reboot (via a privileged system call or service) to apply the update. The SBL then takes over as described in Section 4.5.
## 8.2. Long-Term Support (LTS)
- Establish LTS versions and their support timelines.
        **LTS Definition:**
        -   **Duration:** An LTS release of grx will be supported for a minimum of 5 years from its initial release date. Extended support (e.g., up to 10-15 years) may be offered for specific product lines or customer agreements.
        -   **Scope of Fixes during LTS:**
            *   **Security Vulnerabilities:** Critical and high-severity security vulnerabilities will be patched.
            *   **Critical Bug Fixes:** Bugs leading to system failure, data corruption, or significant deviation from specified behavior that affect safety or core functionality will be fixed.
            *   **No New Features:** New features or major API changes will generally not be backported to LTS branches to maintain stability. Exception may be made for critical hardware enablement if required.
        -   **Release Cadence:** Patches for LTS versions will be bundled into periodic maintenance releases (e.g., quarterly or semi-annually, with emergency out-of-band patches for severe vulnerabilities if necessary).
## 8.3. Backward Compatibility Policy
- Define policies for API/ABI compatibility across releases.
        **API/ABI Stability:**
        -   **Kernel System Call Interface:** The core syscalls defined in Section 4.1.1 will maintain backward compatibility (source and binary) across minor and patch versions within a major LTS release. Additions are allowed; removals or breaking changes only at major version boundaries with a clear deprecation cycle.
        -   **Kernel Object Methods (via `grx_KernelObject_Invoke`):** Similar to syscalls, existing method IDs and their parameters for core objects will be stable. New methods may be added.
        -   **User-Space Service APIs (IDL-based):** Services defined via IDL (Section 4.4.5) will be versioned. Components providing these services will aim to support older interface versions for a defined period to allow clients to migrate. Breaking changes will trigger a major version increment of the service API.
        -   **HAL API:** The HAL API (Section 4.6) will be versioned. Changes will aim for backward compatibility where possible, but hardware evolution may necessitate breaking changes at major HAL versions.
        -   **Driver Interfaces:** Interfaces between the OS and user-space drivers will strive for stability, but may need to evolve with new hardware capabilities or OS features.
        A clear deprecation policy (e.g., minimum 1-2 LTS release cycles notice before removal of a deprecated API) will be implemented.
## 8.4. Feature Roadmap
- Outline a high-level roadmap for future feature enhancements, new hardware support, and standard compliance updates.
## 8.5. End-of-Life (EOL) Policy
- Define a clear EOL policy for versions and hardware platforms.

# 9. Risk Management
(as defined in the [Risk Management Plan](../../docs/plm/risk_management_plan.md))
## 9.1. Risk Identification
- Identify potential technical, development, security, safety, and operational risks.
  - *Technical:* Performance bottlenecks, complex IPC debugging, hardware dependencies.
        - Performance tuning and ensuring bounded WCET for critical IPC paths.
        - Complexity of designing and debugging a fully capability-based security system.
        - Ensuring spatial and temporal isolation in MCS configurations.
  - *Development:* Lack of specialized expertise, budget/schedule overruns, toolchain maturity.
        - Maintaining the integrity and completeness of formal verification proofs if pursued for the kernel.
        - Steep learning curve for developers new to microkernel architectures and capability systems.
  - *Security:* Undiscovered vulnerabilities, supply chain attacks, key management failures.
  - *Safety:* Unforeseen failure modes, certification delays/failures.
        - Incomplete threat modeling leading to missed safety hazards.
        - Subtle interactions between components leading to emergent unsafe behavior.
## 9.2. Risk Analysis
- Assess the likelihood and impact of each identified risk.
        A standard 5x5 Risk Assessment Matrix will be used:
        **Likelihood Scale (Example):**
        1.  **Very Low (Remote):** Highly unlikely to occur during system lifecycle.
        2.  **Low (Unlikely):** Possible, but not expected to occur.
        3.  **Medium (Occasional):** May occur occasionally during system lifecycle.
        4.  **High (Likely):** Expected to occur multiple times during system lifecycle.
        5.  **Very High (Frequent/Certain):** Almost certain to occur, or occurs continuously.

        **Impact Scale (Example - can be tailored for Safety, Security, Project Cost/Schedule):**
        1.  **Very Low (Negligible):** Minor inconvenience, no impact on safety/security, minimal cost/schedule effect.
        2.  **Low (Minor):** Minor degradation of non-critical functions, limited potential for security breach (no sensitive data), manageable cost/schedule impact.
        3.  **Medium (Moderate):** Loss of non-critical functions, potential for contained security breach (non-sensitive data exposure), significant cost/schedule impact. For safety: minor injuries possible, or system damage not affecting safety.
        4.  **High (Serious):** Loss of critical functions, potential for significant security breach (sensitive data exposure, DoS), major cost/schedule impact. For safety: serious injuries or death to a few people, or significant system damage affecting safety.
        5.  **Very High (Catastrophic):** Complete system failure, major security breach (loss of control, widespread data compromise), unacceptable cost/schedule impact. For safety: multiple fatalities, loss of system.

        Risk Priority Number (RPN) can be derived (e.g., Likelihood x Impact) to prioritize mitigation efforts. Risks above a defined threshold RPN will require mandatory mitigation actions.
## 9.3. Risk Mitigation Strategies
- Define concrete actions to mitigate each risk.
  - *Expertise:* Training, hiring, external consultants.
  - *Complexity:* Phased development, clear architectural boundaries, formal methods for critical parts.
  - *Security:* Continuous threat modeling, security audits, fuzzing, secure coding guidelines.
  - *Safety:* FMEA/FTA, fault injection, early certification body engagement.

        **Detailed Mitigation Examples:**

        1.  **Risk:** *Complexity of designing and debugging a fully capability-based security system (from Section 9.1 Development Risks).*
            *   **Likelihood:** Medium (due to specialized nature).
            *   **Impact:** High (could lead to security flaws or significant development delays).
            *   **Mitigation Steps:**
                1.  **Formal Modeling (Partial):** Formally model key aspects of the capability system (e.g., derivation, revocation) to verify core security properties before implementation.
                2.  **Reference Implementation Study:** Thoroughly study existing successful capability systems (e.g., seL4, Capsicum) for proven patterns and potential pitfalls.
                3.  **Developer Training:** Provide intensive training on capability system concepts and secure coding practices for all developers working on or with the capability system.
                4.  **Specialized Debugging Tools:** Develop or adapt debugging tools to visualize CSpaces, capability derivations, and object permissions to aid in troubleshooting. (e.g., GDB extensions as per Section 5.2.2).
                5.  **Security-Focused Code Reviews:** Mandate that all code interacting with the capability system undergo rigorous reviews by security experts.
                6.  **Extensive Test Cases:** Develop a comprehensive suite of positive and negative test cases specifically targeting capability validation, delegation, and revocation scenarios. Include fuzz testing of capability-related syscalls.

        2.  **Risk:** *Ensuring spatial and temporal isolation in MCS configurations (from Section 9.1 Technical Risks).*
            *   **Likelihood:** Medium (complex interactions).
            *   **Impact:** Very High (failure to isolate can violate core safety and security premises).
            *   **Mitigation Steps:**
                1.  **Formal Verification of Core Mechanisms:** Where feasible (e.g., for the kernel scheduler's time partitioning logic and MMU management for spatial isolation), apply formal verification techniques.
                2.  **Rigorous WCET Analysis:** Perform thorough WCET analysis for all critical components (as per Section 3.5.3) to ensure budget adherence.
                3.  **Hardware Support:** Leverage hardware features for isolation (MMU/MPU, IOMMU if available, distinct interrupt controllers or virtualization extensions).
                4.  **Restricted Communication Channels:** Strictly control IPC pathways between components of different criticality levels. Use gateways or monitors for inter-criticality communication if necessary.
                5.  **Extensive System-Level Testing:** Conduct stress tests with mixed-criticality workloads, monitoring for overruns, deadline misses, and any cross-component interference (e.g., using bus monitoring tools, performance counters).
                6.  **Configuration Validation Tools:** Develop tools to validate the overall system configuration for MCS, ensuring that specified budgets, priorities, and resource allocations are consistent and enforceable.

        3.  **Risk:** *Maintaining the integrity and completeness of formal verification proofs if pursued for the kernel (from Section 9.1 Development Risks).*
            *   **Likelihood:** Medium (proofs are complex and tied to code).
            *   **Impact:** High (compromised proofs undermine assurance claims).
            *   **Mitigation Steps:**
                1.  **Dedicated Verification Team:** Employ or train a dedicated team with expertise in the chosen formal methods and tools (e.g., Isabelle/HOL).
                2.  **Proof Maintenance Strategy:** Integrate proof updates into the CI/CD pipeline. Changes to verified code must trigger re-proof or proof adaptation.
                3.  **Automated Proof Scripting:** Maximize automation of proof execution and checking.
                4.  **Regular Audits of Proofs:** Conduct periodic independent audits of the formal proofs and their assumptions.
                5.  **Version Control for Proofs:** Store all proof scripts and related artifacts in version control, tightly coupled with the source code they verify.
                6.  **Clear Documentation of Assumptions:** All assumptions made during formal verification (e.g., about hardware behavior, compiler correctness) must be explicitly documented and reviewed.
## 9.4. Risk Monitoring
- Continuously monitor risks throughout the project lifecycle.

# 10. Interoperability and Ecosystem
## 10.1. Application Development API
- Define the primary API for application development (e.g., POSIX-like library in user-space, or a custom grx component API). Consideration will be given to providing a POSIX-compatible library (e.g., similar to Redox OS's relibc or Zircon's Starnix/VDSO model) running entirely in user-space to ease porting of existing applications. However, the primary/native API will be based on the grx component model and IPC mechanisms.
        If a POSIX-compatible user-space library is provided, it would aim to support a subset of POSIX.1 relevant for embedded and real-time applications. This might include:
        -   **Key Headers (Examples):**
            -   `<pthread.h>`: For basic thread creation (`pthread_create`, `pthread_join`, `pthread_exit`) and synchronization primitives (mutexes: `pthread_mutex_init/lock/unlock/destroy`, condition variables: `pthread_cond_init/wait/signal/broadcast/destroy`). These library functions would map to underlying grx TCB control methods and IPC/Notification objects.
            -   `<semaphore.h>`: For POSIX semaphores (`sem_init`, `sem_wait`, `sem_post`, `sem_destroy`), implemented over grx Notification objects or dedicated semaphore services.
            -   `<mqueue.h>`: For POSIX message queues (`mq_open`, `mq_send`, `mq_receive`, `mq_close`), implemented via grx IPC and a message queue service component.
            -   `<time.h>` / `<sys/time.h>`: For time functions (`clock_gettime`, `nanosleep`), interfacing with the grx Time Service.
            -   `<stdio.h>` (subset): For console I/O (`printf`, `scanf` if a console driver is present), mapping to a serial driver component via IPC.
            -   `<stdlib.h>` (subset): For memory allocation (`malloc`, `free`), managed by a user-space heap within the component, potentially initialized with memory from the Resource Manager.
            -   `<unistd.h>` (subset): For constants like `_SC_CLK_TCK`, `sleep()`.
        -   **File I/O (`<fcntl.h>`, `<unistd.h>` for `read`/`write`/`open`/`close`):** If a File System service is present, these would be library calls that use IPC to communicate with the FS service component (see Section 2.2.2).
        This POSIX-like layer would be a separate set of user-space libraries, not part of the core grx microkernel or its native API.
## 10.2. Third-Party Software Integration
- Strategy for integrating third-party libraries and middleware (e.g., networking protocols, graphics libraries), considering their safety/security implications.
        **Vetting and Integration Policy/Process:**
        1.  **License Compliance:** All third-party software must have a compatible license (e.g., permissive licenses like BSD, MIT, or Apache 2.0 preferred, GPL variants to be carefully reviewed for linkage implications with the core OS or proprietary components). A Software Bill of Materials (SBOM) will be maintained.
        2.  **Security Assessment:**
            *   Review known vulnerabilities (e.g., CVE database search).
            *   Perform static analysis on the source code if available.
            *   Assess the software's own security practices (e.g., history of responding to vulnerabilities).
        3.  **Functional Safety Assessment (if used in safety-critical paths):**
            *   Determine if pre-certified versions are available (e.g., for a third-party math library).
            *   If not, assess the effort required to qualify or certify the software according to grx safety standards (may involve extensive testing, code wrapping, or even re-implementation of critical parts).
            *   Analyze potential interference with other grx components.
        4.  **Porting and Integration:**
            *   The software must be adapted to use grx's native APIs or the provided POSIX-like compatibility layer.
            *   It should run as one or more grx components (Section 4.4) with a defined manifest and minimal necessary capabilities.
        5.  **Testing:** Rigorous integration and system testing of the third-party software within the grx environment.
        6.  **Maintenance Plan:** Strategy for monitoring for updates and vulnerabilities in the third-party software and integrating them into grx releases.
## 10.3. Virtualization/Hypervisor Support
- If grx will host other OSes (e.g., a Linux guest for HMI), specify the hypervisor primitives and the level of isolation provided.
        If grx provides hypervisor capabilities, it will function as a **Type 1 (bare-metal) hypervisor** due to its microkernel design. The kernel itself would provide the minimal primitives required for virtualization.
        **Key Hypervisor Features (if implemented):**
        1.  **Guest VM Isolation:** Strong spatial and temporal isolation between guest VMs and between VMs and native grx components, enforced by the kernel's capability system and scheduler.
        2.  **Device Pass-through:** Mechanisms to securely assign physical hardware devices (e.g., network cards, USB controllers) directly to specific guest VMs, potentially using IOMMU hardware if available.
        3.  **Virtual Device Emulation:** User-space components acting as virtual device backends (e.g., virtual network switch, virtual block device) for guest VMs, communicating via virtio or similar paravirtualization interfaces.
        4.  **VM Lifecycle Management:** APIs for creating, pausing, resuming, and terminating guest VMs, exposed to a privileged VM Manager component.
        5.  **Secure Boot for VMs:** Support for verifying guest OS images before boot.
        The grx hypervisor would be designed with the same principles of minimality and security as the rest of the kernel.
## 10.4. Debugging and Tracing
- Specify support for remote debugging, system-wide tracing, and performance profiling across kernel and user-space components. This may include developing or integrating tracing tools capable of capturing IPC interactions and component state changes, potentially inspired by LTTng or Fuchsia's tracing system. Hardware debug interfaces (JTAG/SWD) will be essential for low-level kernel and bootloader debugging.
## 10.5. Developer Community and Support Model
- Define the strategy for building a developer community (e.g., open-source components, forums) and/or a commercial support model (e.g., professional services, qualified versions).
        **Initial Steps for Community Building:**
        1.  **Open Source Core Components:** Release the grx microkernel, key system services, and HAL/BSP for reference platforms under a permissive open-source license (e.g., Apache 2.0 or MIT).
        2.  **Public Code Repository:** Host source code on a public platform like GitHub or GitLab.
        3.  **Documentation Portal:** Create a website with comprehensive documentation (this Product Specification, design documents, API references, tutorials).
        4.  **Communication Channels:** Establish a public mailing list and/or a discussion forum (e.g., Discourse, Discord server) for developer and user interaction.
        5.  **Contribution Guidelines:** Clearly document the process for external contributions (coding standards, testing requirements, CLA if needed, review process).
        6.  **Regular Updates/Roadmap:** Publish a high-level development roadmap and provide regular updates on progress to foster engagement.

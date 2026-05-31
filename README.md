# Netriq OS

Netriq is a minimalist, secure, bare-metal router operating system built in Rust. It is designed with the "WireGuard philosophy" in mind: stripping away legacy bloat in favor of modern, auditable, and highly performant architecture.

Currently, this repository houses the **Phase 1** implementation: a custom, bare-metal UEFI bootloader that establishes the hardware foundation before handing off control to the kernel.

## Architecture & Current Status

The Netriq bootloader operates directly on the UEFI firmware and successfully executes the critical transition to a bare-metal CPU state. Current implemented features include:

* **UEFI Bootstrapping:** Initialization of the UEFI environment and console protocols.
* **GOP Framebuffer Hijacking:** Exclusive locking of the Graphics Output Protocol (GOP) to extract the raw VRAM pointer, resolution, and hardware stride. This ensures the kernel has direct, bare-metal screen access after UEFI is destroyed.
* **Atomic Bare-Metal Handoff:** Secure execution of `ExitBootServices` using `uefi-rs` native allocation. This atomically extracts the physical Memory Map and authorization key, preventing race conditions and leaving the CPU in a raw execution state.

## Getting Started (Developer Guide)

Netriq is developed and tested using the QEMU emulator. To run the current bootloader in your local environment, follow the steps below.

### Prerequisites

1. **Rust Toolchain:** Ensure you have the Rust toolchain installed with the UEFI target.

   rustup target add x86_64-unknown-uefi
2. **QEMU:** Install QEMU for your operating system (`qemu-system-x86_64`).
3. **UEFI Firmware (Crucial):** Because we maintain a clean, binary-free repository, you must manually provide the UEFI firmware file for QEMU.

   * Download the `edk2-x86_64-code.fd` file (from the TianoCore project or from QEMU installation "C:\Program Files\qemu\share").
   * Place the `edk2-x86_64-code.fd` file **directly in the root directory** of this repository.

### Building and Running

We provide a PowerShell runner script that automatically builds the UEFI binary and attaches it to the QEMU emulator as a virtual FAT drive.

Run the following command in the root directory:

```
./runner.ps1
```

If successful, QEMU will launch, allocate the graphics framebuffer, execute the UEFI initialization, print the startup logs, and then permanently exit boot services, halting the CPU in a safe bare-metal state awaiting the future kernel.

## Project Structure

* `src/main.rs`: The entry point. Handles UEFI initialization, coordinates hardware hijacking, and executes the final bare-metal transition.
* `src/framebuffer.rs`: Contains the `FrameBufferInfo` struct and the logic to extract raw display pointers and metadata from the GPU.
* `.cargo/config.toml`: Configures the default build target to `x86_64-unknown-uefi`.
* `runner.ps1`: Automated QEMU environment spin-up script.

## License

MIT License

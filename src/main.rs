#![no_std]
#![no_main]

mod framebuffer;
use uefi::prelude::*;
use uefi::boot;
use core::time::Duration;

/// Entry point of the Netriq Bootloader.
///
/// This function initializes the environment, hijacks hardware resources (like the screen),
/// extracts the physical memory map, and performs the ultimate transition from the UEFI
/// environment into the bare-metal kernel state.
#[entry]
fn efi_main() -> Status {
    // 1. Base Environment Initialization
    // Bootstraps the `uefi-rs` library. This sets up a temporary memory allocator
    // and initializes the console protocol so we can use the `log` macros.
    uefi::helpers::init().unwrap();

    log::info!("Welcome to Netriq bootloader!");

    // 2. Hardware Hijacking (Graphics)
    // We secure the framebuffer's raw pointer and metadata *before* we kill the UEFI firmware.
    // Once UEFI is gone, this pointer will be our only way to display anything on the screen.
    framebuffer::init_graphics();

    // 3. Human Observation Window
    // Pauses the CPU execution for 5 seconds. This gives us time to see the printed logs
    // before the text console is permanently destroyed.
    boot::stall(Duration::from_secs(5));

    // 4. THE POINT OF NO RETURN: Exit Boot Services & Memory Map Extraction
    // This is the most critical and dangerous line in the bootloader.
    //
    // Passing `None` tells the library to use `MemoryType::LOADER_DATA` by default.
    // Under the hood, this single function performs an atomic, multistep operation:
    //
    //   A. Allocation: It asks the UEFI firmware how much RAM is needed to store the map.
    //      Then it allocates a buffer of that size.
    //
    //   B. The Memory Map Structure: It retrieves the complete layout of the system's physical memory.
    //      This map consists of:
    //      - Metadata: Total size and the `map_key` (see step C).
    //      - Descriptors: An array of `MemoryDescriptor` entries. Each descriptor represents
    //        a chunk of physical RAM and contains:
    //          * `phys_start`: The absolute physical starting address of the chunk.
    //          * `page_count`: Size of the chunk in 4KB pages.
    //          * `ty` (Type): What is stored here (e.g., `ConventionalMemory` for free RAM,
    //            `LoaderData` for our code, `Reserved` for MMIO hardware).
    //          * `att`: Hardware attributes (e.g., Uncacheable, Read-Only).
    //
    //   C. The Authorization Key (`map_key`):
    //      UEFI generates a unique cryptographic timestamp (`map_key`) for this exact map.
    //      If even 1 byte of memory allocation changes in the background, this key becomes invalid.
    //
    //   D. The Execution:
    //      Instantly after getting the `map_key`, the library passes it to the firmware's
    //      hardware shutdown function. The firmware verifies the key to prevent Race Conditions.
    //      Upon successful verification, UEFI terminates its services and transfers exclusive
    //      control of the hardware to our execution environment.
    let _memory_map = unsafe {
        uefi::boot::exit_boot_services(None)
    };

    // --- BARE METAL ZONE ---
    // From this line onward, UEFI no longer exists.
    // We cannot use `log::info!`, file systems, or USB drivers. We only have raw CPU,
    // the physical Memory Map, and the Framebuffer pointer.
    loop {}
}

/// System Panic Handler
///
/// This function is triggered if our Rust code encounters an unrecoverable error (e.g., unwrapping a None).
/// Since we operate without a standard operating system,we cannot safely abort the process.
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // While UEFI is still alive, this will print the exact file and line number of the crash.
    // (If it panics AFTER exit_boot_services, we won't see this unless we implement a custom framebuffer logger).
    log::error!("{}", info);

    // We halt the CPU in an infinite loop. This prevents the processor from executing
    // random memory garbage and burning down the system.
    loop {}
}
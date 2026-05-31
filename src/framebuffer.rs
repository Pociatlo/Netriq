use uefi::boot::{get_handle_for_protocol, open_protocol_exclusive};
use uefi::proto::console::gop::GraphicsOutput;

/// Holds essential metadata and the raw memory pointer for the screen's framebuffer.
///
/// This structure is designed to persist even after `ExitBootServices` is called,
/// allowing the kernel to draw directly to the screen without UEFI's help.
#[derive(Debug, Clone, Copy)]
pub struct FrameBufferInfo {
    /// Raw, mutable pointer to the first byte of the video RAM.
    pub base_address: *mut u8,
    /// Total size of the framebuffer memory in bytes.
    pub size: usize,
    /// Visible width of the screen in pixels.
    pub width: usize,
    /// Visible height of the screen in pixels.
    pub height: usize,
    /// Actual number of pixels per memory row (includes hardware padding).
    pub stride: usize,
}

/// Hijacks the Graphics Output Protocol (GOP) from UEFI.
///
/// Locates the active graphics hardware, claims exclusive access to it,
/// and extracts the raw memory pointer and resolution data needed for bare-metal rendering.
pub fn init_graphics() -> FrameBufferInfo{
    //  1. Handle
    // Locate hardware in UEFI register and obtain ID of it. It's information about what Graphical Processing Unit (GPU) supports drawing on screen.
    let handle = get_handle_for_protocol::<GraphicsOutput>().unwrap();
    //  2. Graphics Output Protocol (GOP) Control
    //  Open Graphics Output Protocol (GOP) exclusive to secure it from panic if other UEFI process needed to use this protocol.
    let mut gop = open_protocol_exclusive::<GraphicsOutput>(handle).unwrap();
    //  3. Display Mode
    //  Getting metadata of current display mode.
    let gop_info = gop.current_mode_info();
    //  4. Screen Resolution
    //  Using Graphics Output Protocol (GOP) metadata obtain information about resolution of supported screen.
    let (width,height) = gop_info.resolution();
    //  5. Screen Stride & Framebuffer Metadata
    //  Obtaining information about the actual size of one line on the screen in memory (RAM or VRAM).
    //  Depending on the hardware, this uses system RAM (integrated GPU) or dedicated VRAM (discrete GPU).
    //
    //  The CPU accesses this video memory via Memory-Mapped I/O (MMIO). During boot, the UEFI firmware
    //  (PCI enumerator) probed the GPU's Base Address Register (BAR) to find out how much address space
    //  the GPU required. UEFI then allocated a free region in the system's physical memory map and
    //  programmed the BAR with that specific starting address.
    //  Here, Graphics Output Protocol (GOP) abstracts this away and gives us a direct pointer to that mapped memory region.
    let stride = gop_info.stride();
    //  6. Framebuffer Object
    //  Asking GPU to return from (VRAM) memory, table of mapped colors of pixels.
    let mut framebuffer = gop.frame_buffer();
    //  7. Framebuffer Size
    //  Obtaining size of screen to avoid drawing off-screen.
    let size = framebuffer.size();
    //  8. Base Address
    //  Obtaining pointer of first address in Framebuffer. This is the first pixel of top left in screen.
    //  This is address is the only one element that is used to communicate with Screen after UEFI exits.
    let base_address = framebuffer.as_mut_ptr();

    FrameBufferInfo{
         base_address,
         size,
         width,
         height,
         stride,
    }
}



use core::{arch::asm, fmt};
use memory_addr::VirtAddr;

#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
use alloc::boxed::Box;

/// Saved registers when a trap (interrupt or exception) occurs.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TrapFrame {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,

    // Pushed by `trap.S`
    pub vector: u64,
    pub error_code: u64,

    // Pushed by CPU
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

impl TrapFrame {
    /// Whether the trap is from userspace.
    pub const fn is_user(&self) -> bool {
        self.cs & 0b11 == 3
    }
}

#[repr(C)]
#[derive(Debug, Default)]
struct ContextSwitchFrame {
    #[cfg(feature = "std")]
    fs: u64,    // TLS support
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    rip: u64,
}

/// A 512-byte memory region for the FXSAVE/FXRSTOR instruction to save and
/// restore the x87 FPU, MMX, XMM, and MXCSR registers.
///
/// See <https://www.felixcloutier.com/x86/fxsave> for more details.
#[allow(missing_docs)]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct FxsaveArea {
    pub fcw: u16,
    pub fsw: u16,
    pub ftw: u16,
    pub fop: u16,
    pub fip: u64,
    pub fdp: u64,
    pub mxcsr: u32,
    pub mxcsr_mask: u32,
    pub st: [u64; 16],
    pub xmm: [u64; 32],
    _padding: [u64; 12],
}

static_assertions::const_assert_eq!(core::mem::size_of::<FxsaveArea>(), 512);

/// Extended state of a task, such as FP/SIMD states.
pub struct ExtendedState {
    /// Memory region for the FXSAVE/FXRSTOR instruction.
    pub fxsave_area: FxsaveArea,
}

#[cfg(feature = "fp_simd")]
impl ExtendedState {
    #[inline]
    fn save(&mut self) {
        unsafe { core::arch::x86_64::_fxsave64(&mut self.fxsave_area as *mut _ as *mut u8) }
    }

    #[inline]
    fn restore(&self) {
        unsafe { core::arch::x86_64::_fxrstor64(&self.fxsave_area as *const _ as *const u8) }
    }

    const fn default() -> Self {
        let mut area: FxsaveArea = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
        area.fcw = 0x37f;
        area.ftw = 0xffff;
        area.mxcsr = 0x1f80;
        Self { fxsave_area: area }
    }
}

impl fmt::Debug for ExtendedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ExtendedState")
            .field("fxsave_area", &self.fxsave_area)
            .finish()
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct TaskTLS {
    _block: Box<[u8]>,
    thread_ptr: Box<*mut ()>,
}

/// Saved hardware states of a task.
///
/// The context usually includes:
///
/// - Callee-saved registers
/// - Stack pointer register
/// - Thread pointer register (for thread-local storage, currently unsupported)
/// - FP/SIMD registers
///
/// On context switch, current task saves its context from CPU to memory,
/// and the next task restores its context from memory to CPU.
///
/// On x86_64, callee-saved registers are saved to the kernel stack by the
/// `PUSH` instruction. So that [`rsp`] is the `RSP` after callee-saved
/// registers are pushed, and [`kstack_top`] is the top of the kernel stack
/// (`RSP` before any push).
///
/// [`rsp`]: TaskContext::rsp
/// [`kstack_top`]: TaskContext::kstack_top
#[derive(Debug)]
pub struct TaskContext {
    /// The kernel stack top of the task.
    pub kstack_top: VirtAddr,
    /// `RSP` after all callee-saved registers are pushed.
    pub rsp: u64,
    /// Task Thread-Local-Storage (TLS)
    #[cfg(feature = "std")]
    pub tls: Option<TaskTLS>,
    /// Extended states, i.e., FP/SIMD states.
    #[cfg(feature = "fp_simd")]
    pub ext_state: ExtendedState,
}

impl TaskContext {
    /// Creates a new default context for a new task.
    pub const fn new() -> Self {
        Self {
            kstack_top: VirtAddr::from(0),
            rsp: 0,
            #[cfg(feature = "std")]
            tls: None,
            #[cfg(feature = "fp_simd")]
            ext_state: ExtendedState::default(),
        }
    }

    #[cfg(feature = "std")]
    pub fn init_tls(&mut self) {
        if self.tls.is_none() {
            self.tls = TaskTLS::setup_tls();
        }
    }

    /// Initializes the context for a new task, with the given entry point and
    /// kernel stack.
    pub fn init(&mut self, entry: usize, kstack_top: VirtAddr) {
        unsafe {
            // x86_64 calling convention: the stack must be 16-byte aligned before
            // calling a function. That means when entering a new task (`ret` in `context_switch`
            // is executed), (stack pointer + 8) should be 16-byte aligned.
            let frame_ptr = (kstack_top.as_mut_ptr() as *mut u64).sub(1);
            let frame_ptr = (frame_ptr as *mut ContextSwitchFrame).sub(1);
            #[allow(unused_mut)]
            let mut ctx_frame = ContextSwitchFrame {
                rip: entry as _,
                ..Default::default()
            };
            #[cfg(feature = "std")]
            {
                self.init_tls();
                if let Some(tls) = &self.tls {
                    ctx_frame.fs = tls.thread_ptr() as *const _ as u64;
                }
            }
            core::ptr::write(
                frame_ptr,
                ctx_frame,
            );
            self.rsp = frame_ptr as u64;
        }
        self.kstack_top = kstack_top;
    }

    #[cfg(feature = "std")]
    pub fn thread_local_storage(&self) -> u64 {
        if let Some(tls) = &self.tls {
            tls.thread_ptr() as *const _ as u64
        } else {
            panic!("no tls block found!");
        }
    }

    /// Switches to another task.
    ///
    /// It first saves the current task's context from CPU to this place, and then
    /// restores the next task's context from `next_ctx` to CPU.
    pub fn switch_to(&mut self, next_ctx: &Self) {
        #[cfg(feature = "fp_simd")]
        {
            self.ext_state.save();
            next_ctx.ext_state.restore();
        }
        unsafe {
            context_switch(&mut self.rsp, &next_ctx.rsp)
        }
    }
}

#[cfg(feature = "std")]
impl TaskTLS {
    pub fn setup_tls() -> Option<TaskTLS> {
        extern "C" {
            fn tls_start();
            fn tls_tbss_start();
            fn tls_end();
        }

        let start = tls_start as usize;
        let tbss_start = tls_tbss_start as usize;
        let end = tls_end as usize;

        let tls_len = end - start;
        // Get TLS initialization image
        let tls_init_image = {
            let tls_init_data = start as *const u8;
            let tls_init_len = tbss_start - start;

            // SAFETY: We will have to trust the environment here.
            unsafe { core::slice::from_raw_parts(tls_init_data, tls_init_len) }
        };
        // Allocate TLS block
        let mut block = {
            let tls_align = 8;

            // As described in “ELF Handling For Thread-Local Storage”
            let tls_offset = Self::align_up(tls_len, tls_align);

            // To access TLS blocks on x86-64, TLS offsets are *subtracted* from the thread register value.
            // So the thread pointer needs to be `block_ptr + tls_offset`.
            // Allocating only tls_len bytes would be enough to hold the TLS block.
            // For the thread pointer to be sound though, we need it's value to be included in or one byte past the same allocation.
            alloc::vec![0; tls_offset].into_boxed_slice()
        };
        // Initialize beginning of the TLS block with TLS initialization image
        block[..tls_init_image.len()].copy_from_slice(tls_init_image);

        // The end of the TLS block was already zeroed by the allocator

        // thread_ptr = block_ptr + tls_offset
        // block.len() == tls_offset
        let thread_ptr = block.as_mut_ptr_range().end.cast::<()>();

        // Put thread pointer on heap, so it does not move and can be referenced in fs:0
        let thread_ptr = Box::new(thread_ptr);
        let ret = TaskTLS {
            _block: block,
            thread_ptr,
        };
        Some(ret)
    }

    pub fn thread_ptr(&self) -> &*mut () {
        &self.thread_ptr
    }

    const fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }
}

#[cfg(feature = "std")]
macro_rules! push_fs {
    () => {
        r#"
        mov ecx, 0xc0000100 // FS.Base Model Specific Register
        rdmsr
        sub rsp, 8
        mov [rsp+4], edx
        mov [rsp], eax
        "#
    };
}

#[cfg(not(feature = "std"))]
macro_rules! push_fs {
    () => { "" };
}

#[cfg(feature = "std")]
macro_rules! pop_fs {
    () => {
        r#"
        mov ecx, 0xc0000100 // FS.Base Model Specific Register
        mov edx, [rsp+4]
        mov eax, [rsp]
        add rsp, 8
        wrmsr
        "#
    };
}

#[cfg(not(feature = "std"))]
macro_rules! pop_fs {
    () => { "" };
}

#[naked]
unsafe extern "C" fn context_switch(_current_stack: &mut u64, _next_stack: &u64) {
    asm!(
        "
        push    rbp
        push    rbx
        push    r12
        push    r13
        push    r14
        push    r15
        ",

        push_fs!(),

        "
        mov     [rdi], rsp
        mov     rsp, [rsi]
        ",

        pop_fs!(),

        "
        pop     r15
        pop     r14
        pop     r13
        pop     r12
        pop     rbx
        pop     rbp
        ret",
        options(noreturn),
    )
}

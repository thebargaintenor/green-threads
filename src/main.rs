#![feature(asm)]

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");

    loop {}
}

// manipulating the CPU directly is unsafe?  fancy that.
unsafe fn gt_switch(new: *const ThreadContext) {
    // deprecated LLVM syntax
    // https://doc.rust-lang.org/beta/unstable-book/library-features/llvm-asm.html
    // llvm_asm!("
    //     mov     0x00($0), %rsp
    //     ret
    //     "
    // :
    // : "r"(new)
    // :
    // : "alignstack"
    // );

    // updated ASM macro syntax
    // https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html
    // https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html
    asm!(
        "
        mov     0x00({context}), %rsp
        ret
        ",
        context = in(reg) new,
        // note that `alignstack` is now default behavior in LLVM
    );
}

fn main() {
    println!("Hello, world!");
}

# green-threads

Updated implementation of [Green Threads Explained in 200 Lines of Rust](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/an-example-we-can-build-upon).

NB: This example makes extensive use of `llvm_asm`.  That macro is deprecated.  I had to do some extra poking around to sort out what changed for the new unstable `asm!` macro:

* https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html
* Current `asm` macro: [rust docs](https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html)
* Explanation of deprecated `llvm_asm` macro: [rust docs](https://doc.rust-lang.org/beta/unstable-book/library-features/llvm-asm.html)

Obviously this isn't remotely done yet, but I needed this in a repo for portability.  Build your own.  Or, you know, [use the official version](https://github.com/cfsamson/example-greenthreads).

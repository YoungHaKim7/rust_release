# Result

```
$ valgrind --leak-check=full ./rust_1_73

ell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { value: Cons(5, RefCell { value: Cons(10, RefCell { ==35902== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==35902== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==35902==
==35902== Process terminating with default action of signal 11 (SIGSEGV)
==35902==  Access not within mapped region at address 0x1FFE801FE0
==35902== Stack overflow in thread #1: can't grow stack to 0x1ffe801000
==35902==    at 0x12B242: <std::io::stdio::StdoutLock as std::io::Write>::write_all (memchr.rs:23)
==35902==  If you believe this happened as a result of a stack
==35902==  overflow in your program's main thread (unlikely but
==35902==  possible), you can try to increase the size of the
==35902==  main thread stack using the --main-stacksize= flag.
==35902==  The main thread stack size used in this run was 8388608.
==35902==
==35902== HEAP SUMMARY:
==35902==     in use at exit: 1,189 bytes in 6 blocks
==35902==   total heap usage: 14 allocs, 8 frees, 3,301 bytes allocated
==35902==
==35902== LEAK SUMMARY:
==35902==    definitely lost: 0 bytes in 0 blocks
==35902==    indirectly lost: 0 bytes in 0 blocks
==35902==      possibly lost: 0 bytes in 0 blocks
==35902==    still reachable: 1,189 bytes in 6 blocks
==35902==         suppressed: 0 bytes in 0 blocks
==35902== Reachable blocks (those to which a pointer was found) are not shown.
==35902== To see them, rerun with: --leak-check=full --show-leak-kinds=all
==35902==
==35902== For lists of detected and suppressed errors, rerun with: -s
==35902== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
[1]    35902 segmentation fault (core dumped)  valgrind --leak-check=full ./rust_1_73

```

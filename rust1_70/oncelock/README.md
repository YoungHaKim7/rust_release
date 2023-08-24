# Result

```

$ cargo run
   Compiling oncelock v0.1.0 (/Users/globalyoung/Documents/test/test/rust/rust_release/rust1_70/oncelock)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/oncelock`

main wins!
```

# std::thread::yield_now()

https://doc.rust-lang.org/stable/std/thread/fn.yield_now.html

```
 Cooperatively gives up a timeslice to the OS scheduler.

This calls the underlying OS scheduler’s yield primitive,
 signaling that the calling thread is willing to give up its remaining timeslice so that
  the OS may schedule other threads on the CPU.

A drawback of yielding in a loop is that if the OS does not have any other ready threads
 to run on the current CPU, the thread will effectively busy-wait, which wastes CPU time and energy.

Therefore, when waiting for events of interest, a programmer’s first choice should be
 to use synchronization devices such as channels, Condvars, Mutexes
  or join since these primitives are implemented in a blocking manner,
   giving up the CPU until the event of interest has occurred which avoids repeated yielding.

yield_now should thus be used only rarely,
 mostly in situations where repeated polling is required because there is no other suitable way
  to learn when an event of interest has occurred.
```
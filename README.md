I think I should be able to link against a global variable in Rust.

Rust also seems to tell me that it's fine, because I successfully build.

But then it explodes at runtime :(

This does work in C, so I expect it to work in Rust.

## Overview
I have some `container.c` which is compiled into a static archive.
```c
#include <stdint.h>

uint8_t stuff[] = {
    3, 1, 4
};

uint8_t* get_stuff() {
    return stuff;
}
```

I can link this into my rust application like so
```rust
extern "C" {
    static stuff: *const u8;

    fn get_stuff() -> *const u8;
}
```

`get_stuff()` works great. The following test passes.
```rust
    #[test]
    fn getter() {
        unsafe {
            let mut current = get_stuff();
            assert_eq!(*current, 3);

            current = current.add(1);
            assert_eq!(*current, 1);

            current = current.add(1);
            assert_eq!(*current, 4);
        }
    }
```

However, I can't actually use the directly linked `stuff`. I get a SIGSEGV if I try and read from it, and it's somehow a different pointer than the one returned from the runtime getter.
```
    #[test]
    fn similarity() {
        unsafe {
            assert_eq!(stuff, get_stuff());
        }
    }
```

```
---- tests::similarity stdout ----
thread 'tests::similarity' panicked at src/lib.rs:23:13:
assertion `left == right` failed
  left: 0x1040103
 right: 0xb52882ce0010
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


I'm pretty sure that I am correctly linking `stuff`, because if I change the rust extern C block to an incorrect name like `stufffff` then I get a big linking failure

```
out" "-L" "/home/ubuntu/.rustup/toolchains/stable-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib" "-o" "/home/ubuntu/workspace/link-adventure/target/debug/deps/link_adventure-14c9ed0535a6f0d3" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
  = note: /usr/bin/ld: /home/ubuntu/workspace/link-adventure/target/debug/deps/link_adventure-14c9ed0535a6f0d3.42pztgjo0qz7fs61ircjk4diq.rcgu.o: in function `link_adventure::tests::similarity':
          /home/ubuntu/workspace/link-adventure/src/lib.rs:23:(.text._ZN14link_adventure5tests10similarity17h090216c1182b4675E+0x14): undefined reference to `stufff'
          /usr/bin/ld: /home/ubuntu/workspace/link-adventure/src/lib.rs:23:(.text._ZN14link_adventure5tests10similarity17h090216c1182b4675E+0x18): undefined reference to `stufff'
          /usr/bin/ld: /home/ubuntu/workspace/link-adventure/src/lib.rs:23:(.text._ZN14link_adventure5tests10similarity17h090216c1182b4675E+0x4c): undefined reference to `stufff'
          /usr/bin/ld: /home/ubuntu/workspace/link-adventure/src/lib.rs:23:(.text._ZN14link_adventure5tests10similarity17h090216c1182b4675E+0x50): undefined reference to `stufff'
          collect2: error: ld returned 1 exit status
```


Running the example
## Rust
```
# succeeds
cargo build

# fails
cargo test
```

## C
```
# in lib/
make clean
make 
./app
```
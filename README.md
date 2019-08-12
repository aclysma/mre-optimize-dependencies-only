# mre-optimize-dependencies-only

This is a minimum reproducible example of profile-overrides not working as expected. Through trial and error
I found that the optimization level of the calling code affected performance.

```
nightly-x86_64-apple-darwin (default)
rustc 1.37.0-nightly (7cdaffd79 2019-06-05)
```

My usecase is that my application runs too slowly to be usable in debug mode. I'd like to turn optimizations
on for everything **except** my own code.

## Without profile overrides:

To reproduce, comment out the following in the root cargo.toml

```
[profile.dev]
opt-level = 0

[profile.dev.overrides."*"]
opt-level = 3
```

Results of `cargo run`

```
baseline, do everything in the shim
  shim_test_from_shim update physics took 1139.12ms total
Test with a world created in main crate
  test_step_in_main_world_from_main update physics took 1118.311ms total
  test_step_in_shim_world_from_main update physics took 1110.362ms total
  shim_test_from_main_world_from_main update physics took 1069.597ms total
Now test with a world created in the shim
  test_step_in_main_world_from_shim update physics took 1073.404ms total
  test_step_in_shim_world_from_shim update physics took 1074.331ms total
  shim_test_from_main_world_from_shim update physics took 1072.409ms total
```

Results of `cargo run --release`

```
baseline, do everything in the shim
  shim_test_from_shim update physics took 10.019ms total
Test with a world created in main crate
  test_step_in_main_world_from_main update physics took 10.104ms total
  test_step_in_shim_world_from_main update physics took 10.443ms total
  shim_test_from_main_world_from_main update physics took 9.866ms total
Now test with a world created in the shim
  test_step_in_main_world_from_shim update physics took 9.635ms total
  test_step_in_shim_world_from_shim update physics took 9.89ms total
  shim_test_from_main_world_from_shim update physics took 10.226ms total

```

## With profile overrides:

### (NOTE THE INCONSISTENT BEHAVIOR BASED ON WHERE THE CODE IS CALLED FROM)

Results of `cargo run`

```
baseline, do everything in the shim
  shim_test_from_shim update physics took 10.245ms total
Test with a world created in main crate
  test_step_in_main_world_from_main update physics took 1049.913ms total
  test_step_in_shim_world_from_main update physics took 689.393ms total
  shim_test_from_main_world_from_main update physics took 686.593ms total
Now test with a world created in the shim
  test_step_in_main_world_from_shim update physics took 338.032ms total
  test_step_in_shim_world_from_shim update physics took 9.464ms total
  shim_test_from_main_world_from_shim update physics took 7.917ms total

```

Results of `cargo run --release`

```
baseline, do everything in the shim
  shim_test_from_shim update physics took 10.09ms total
Test with a world created in main crate
  test_step_in_main_world_from_main update physics took 11.479ms total
  test_step_in_shim_world_from_main update physics took 10.116ms total
  shim_test_from_main_world_from_main update physics took 10.247ms total
Now test with a world created in the shim
  test_step_in_main_world_from_shim update physics took 11.161ms total
  test_step_in_shim_world_from_shim update physics took 10.639ms total
  shim_test_from_main_world_from_shim update physics took 11.509ms total
```

## With profile overrides, using cargo run:


### Build output with --verbose

```
   <<snip>>
     Running `rustc --edition=2018 --crate-name nalgebra_glm /Users/philipd/.cargo/registry/src/github.com-1ecc6299db9ec823/nalgebra-glm-0.4.0/src/lib.rs --color always --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C debuginfo=2 -C debug-assertions=on --cfg 'feature="alga"' --cfg 'feature="default"' --cfg 'feature="nalgebra"' --cfg 'feature="std"' -C metadata=fbf9613f1ff6daf9 -C extra-filename=-fbf9613f1ff6daf9 --out-dir /Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps -L dependency=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps --extern alga=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libalga-9e2e22048ec79402.rlib --extern approx=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libapprox-7cd560bf8792fd93.rlib --extern nalgebra=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra-4e1a5f0bfe36c9df.rlib --extern num_traits=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnum_traits-99b04fe15323192a.rlib --cap-lints allow`
   Compiling nphysics2d v0.11.1
     Running `rustc --edition=2018 --crate-name nphysics2d /Users/philipd/.cargo/registry/src/github.com-1ecc6299db9ec823/nphysics2d-0.11.1/src/lib.rs --color always --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C debuginfo=2 -C debug-assertions=on --cfg 'feature="default"' --cfg 'feature="dim2"' --cfg 'feature="stdweb"' -C metadata=f1a8c5caab593923 -C extra-filename=-f1a8c5caab593923 --out-dir /Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps -L dependency=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps --extern alga=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libalga-9e2e22048ec79402.rlib --extern approx=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libapprox-7cd560bf8792fd93.rlib --extern bitflags=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libbitflags-0edcff4997dc1b7c.rlib --extern downcast_rs=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libdowncast_rs-50d1926851ba7182.rlib --extern either=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libeither-6fe3de8d83367ac7.rlib --extern nalgebra=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra-4e1a5f0bfe36c9df.rlib --extern ncollide2d=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libncollide2d-1fee2011cb5c7565.rlib --extern num_traits=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnum_traits-99b04fe15323192a.rlib --extern slab=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libslab-6fe04512c9cc0db3.rlib --extern time=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libtime-20f1cc635fb78c01.rlib --cap-lints allow`
   Compiling physics_shim v0.1.0 (/Users/philipd/dev/rust/mre-optimize-dependencies-only/physics_shim)
     Running `rustc --edition=2018 --crate-name physics_shim physics_shim/src/lib.rs --color always --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C debuginfo=2 -C debug-assertions=on -C metadata=9ea8d363e8008970 -C extra-filename=-9ea8d363e8008970 --out-dir /Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps -C incremental=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/incremental -L dependency=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps --extern nalgebra=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra-4e1a5f0bfe36c9df.rlib --extern nalgebra_glm=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra_glm-fbf9613f1ff6daf9.rlib --extern ncollide2d=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libncollide2d-1fee2011cb5c7565.rlib --extern nphysics2d=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnphysics2d-f1a8c5caab593923.rlib`
   Compiling mre-optimize-dependencies-only v0.1.0 (/Users/philipd/dev/rust/mre-optimize-dependencies-only)
     Running `rustc --edition=2018 --crate-name mre_optimize_dependencies_only src/main.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=3e74b56272ccc034 -C extra-filename=-3e74b56272ccc034 --out-dir /Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps -C incremental=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/incremental -L dependency=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps --extern nalgebra=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra-4e1a5f0bfe36c9df.rlib --extern nalgebra_glm=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnalgebra_glm-fbf9613f1ff6daf9.rlib --extern ncollide2d=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libncollide2d-1fee2011cb5c7565.rlib --extern nphysics2d=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libnphysics2d-f1a8c5caab593923.rlib --extern physics_shim=/Users/philipd/dev/rust/mre-optimize-dependencies-only/target/debug/deps/libphysics_shim-9ea8d363e8008970.rlib`
```

This is mixing optimized and unoptimized code, but I would not expect my linked binary to include an optimized and unoptimized version of the same function. (The crate should have been optimized.)

Trapping on a breakpoint in the function from two different call paths:

### As called from an optimized crate (physics_shim)

```asm
(lldb) dis
mre-optimize-dependencies-only`nphysics2d::world::world::World$LT$N$GT$::step::h65c6f5a8b9ced9bc:
    0x10f3c46d0 <+0>:    pushq  %rbp
    0x10f3c46d1 <+1>:    movq   %rsp, %rbp
    0x10f3c46d4 <+4>:    pushq  %r15
    0x10f3c46d6 <+6>:    pushq  %r14
    0x10f3c46d8 <+8>:    pushq  %r13
    0x10f3c46da <+10>:   pushq  %r12
    0x10f3c46dc <+12>:   pushq  %rbx
    0x10f3c46dd <+13>:   subq   $0x118, %rsp              ; imm = 0x118 
    0x10f3c46e4 <+20>:   movq   %rdi, %rbx
->  0x10f3c46e7 <+23>:   callq  0x10f3d05d0               ; nphysics2d::counters::Counters::step_started::hb8b1813b390e4d1b at mod.rs:55
    0x10f3c46ec <+28>:   leaq   0x140(%rbx), %rax
    0x10f3c46f3 <+35>:   movq   %rax, -0x40(%rbp)
    0x10f3c46f7 <+39>:   movq   0x170(%rbx), %r14
    0x10f3c46fe <+46>:   movq   %rbx, -0x38(%rbp)
    0x10f3c4702 <+50>:   movq   0x180(%rbx), %rax
    0x10f3c4709 <+57>:   shlq   $0x3, %rax
    0x10f3c470d <+61>:   leaq   (%rax,%rax,2), %rax
    0x10f3c4711 <+65>:   movq   %rax, -0x30(%rbp)
    0x10f3c4715 <+69>:   xorl   %r12d, %r12d
    0x10f3c4718 <+72>:   xorl   %r13d, %r13d
    0x10f3c471b <+75>:   cmpq   %r12, -0x30(%rbp)
    0x10f3c471f <+79>:   je     0x10f3c477a               ; <+170> at mod.rs
    0x10f3c4721 <+81>:   nopw   %cs:(%rax,%rax)
    0x10f3c472b <+91>:   nopl   (%rax,%rax)
    0x10f3c4730 <+96>:   movq   %r13, %rax
    0x10f3c4733 <+99>:   addq   $0x1, %rax
    0x10f3c4737 <+103>:  jb     0x10f3c5077               ; <+2471> [inlined] _$LT$slab..IterMut$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::h6f2857461bf700ab at mod.rs:570
    0x10f3c473d <+109>:  cmpq   $0x1, (%r14,%r12)
    0x10f3c4742 <+114>:  jne    0x10f3c476d               ; <+157> [inlined] _$LT$core..slice..IterMut$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::hda2f18af954e9894 at lib.rs:950
    0x10f3c4744 <+116>:  movq   0x8(%r14,%r12), %rbx
    0x10f3c4749 <+121>:  testq  %rbx, %rbx
```

### As called by a non-optimized crate (the root crate, mre_optimize_dependencies_only)

```asm

(lldb) dis
mre-optimize-dependencies-only`nphysics2d::world::world::World$LT$N$GT$::step::hd861b4230936eca1:
    0x10f2a1b60 <+0>:    pushq  %rbp
    0x10f2a1b61 <+1>:    movq   %rsp, %rbp
    0x10f2a1b64 <+4>:    subq   $0x610, %rsp              ; imm = 0x610 
    0x10f2a1b6b <+11>:   movq   %rdi, -0x488(%rbp)
->  0x10f2a1b72 <+18>:   movq   -0x488(%rbp), %rdi
    0x10f2a1b79 <+25>:   callq  0x10f3d05d0               ; nphysics2d::counters::Counters::step_started::hb8b1813b390e4d1b at mod.rs:55
    0x10f2a1b7e <+30>:   jmp    0x10f2a1b8b               ; <+43> at world.rs:236
    0x10f2a1b80 <+32>:   movq   -0x10(%rbp), %rdi
    0x10f2a1b84 <+36>:   callq  0x10f40271c               ; symbol stub for: _Unwind_Resume
    0x10f2a1b89 <+41>:   ud2    
    0x10f2a1b8b <+43>:   movq   -0x488(%rbp), %rax
    0x10f2a1b92 <+50>:   addq   $0x140, %rax              ; imm = 0x140 
    0x10f2a1b98 <+56>:   leaq   -0x468(%rbp), %rdi
    0x10f2a1b9f <+63>:   movq   %rax, %rsi
    0x10f2a1ba2 <+66>:   callq  0x10f394300               ; nphysics2d::object::body_set::BodySet$LT$N$GT$::bodies_mut::h125b829ad3e8d002 at body_set.rs:191
    0x10f2a1ba7 <+71>:   leaq   -0x480(%rbp), %rdi
    0x10f2a1bae <+78>:   leaq   -0x468(%rbp), %rsi
    0x10f2a1bb5 <+85>:   callq  0x10f270240               ; _$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$::into_iter::hfb5566408f227cd1 at collect.rs:245
    0x10f2a1bba <+90>:   movq   -0x480(%rbp), %rax
    0x10f2a1bc1 <+97>:   movq   %rax, -0x450(%rbp)
    0x10f2a1bc8 <+104>:  movq   -0x478(%rbp), %rax
    0x10f2a1bcf <+111>:  movq   %rax, -0x448(%rbp)
    0x10f2a1bd6 <+118>:  movq   -0x470(%rbp), %rax
    0x10f2a1bdd <+125>:  movq   %rax, -0x440(%rbp)
```

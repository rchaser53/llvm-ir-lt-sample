# this is a sample repository for LLVM IR

## ll_sample
  LLVM IR file sample directory.
  just type it like below.

  ```
  $ lli ./ll_sample/hello_world.ll
  ```

## src
  [llvm-sys.rs]() sample direcoty.
  just type it like below.

  ```
  $ cargo run
  ```

### contents
  uncomment what you want to play

  - hello_world
  - loop_struct

## install llvm

  you may need to install llvm at first.

  ```
  # in mac
  $ brew install --with-clang --with-lld --with-python --HEAD llvm
  ```
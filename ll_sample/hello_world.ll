define i32 @main(i32, i8**) {
entry:
  %local_str = alloca [13 x i8]
  store [13 x i8] c"hello world\0A\00", [13 x i8]* %local_str
  %input_puts = getelementptr [13 x i8], [13 x i8]* %local_str, i32 0, i32 0
  call i32 @printf(i8* %input_puts)

  ret i32 0
}

declare i32 @printf(i8*)
;    for i in [1,2,3].iter() {
;      println!("{}", i);
;    }

%test_struct = type { i32 }

@0 = global [8 x i8] c"%d\0A\00aa\00\00"

define i32 @struct_test_func(%test_struct* %a, i32 %b) {
entry:
  %0 = getelementptr inbounds %test_struct, %test_struct* %a, i32 0, i32 0
  %1 = load i32, i32* %0
  %2 = add i32 %b, %1
  ret i32 %2
}

define i32 @main(i32) {
entry:
  %test = alloca %test_struct
  %test_in = getelementptr %test_struct, %test_struct* %test, i32 0, i32 0
  store i32 22, i32* %test_in
  %_ = call i32 @struct_test_func(%test_struct* %test, i32 3)

  %1 = alloca i32
  store i32 0, i32* %1
  br label %loop

loop:                                             ; preds = %loop, %entry
  %2 = load i32, i32* %1
  %a.b = add i32 1, %2
  store i32 %a.b, i32* %1
  %3 = load i32, i32* %1
  %4 = call i32 (...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @0, i32 0, i32 0), i32 %3)
  %5 = icmp ugt i32 %a.b, 2
  br i1 %5, label %loop_end, label %loop

loop_end:                                         ; preds = %loop
  ret i32 0
}

declare i32 @printf(...)
```
gcc -g -shared ./library/awesome_math.c -o ./library/libawesome_math.so
```

cd executable 

```
cargo build --release
```

```
ldd ./target/release/executable | grep awesome
```
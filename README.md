# Performance

This weeks exercise requires you to optimize matrix multiplication as well as mandelbrot set calculation. You're expected to make the code in `matrix.cpp` and `mandelbrot.cpp` as fast as possible. 

Your task is to get as close as possible to our optimized code that we've committed as binaries (or even surpass it!).

## Deliveries

Deliver `libmatrix.so` and `libmandelbrot.so` exporting the symbols from `matrix.h` and `mandelbrot.h`.

## Notes

- If you want to multi-thread your code, use **a maximum of 4 (four)** threads. You can expect all inputs to your code to be a multiple of 4.
- We know that there are faster algorithms for matrix multiplication such as [Strassen Algorithm](https://en.wikipedia.org/wiki/Strassen_algorithm). We **do not** want you to implement that. We want you to **optimize the naive version** given in `matrix.cpp`. The same goes for mandelbrot if you find a faster algorithm.
- For mandelbrot, you're required to use the parameters given in `tests/mandelbrot_params.h`. Do not change them.
- Regarding Rust, use the instrinsics from [core::arch::x86_64](https://doc.rust-lang.org/core/arch/x86_64/index.html) and **not** [std::simd](https://doc.rust-lang.org/nightly/std/simd/index.html).
- We will test your code on the self-hosted runners.
  - You might want to change `runs-on: ubuntu-latest` to `runs-on: self-hosted` in `classroom.yml` to be sure that your benchmarks run on the self-hosted runners (or check manually).

## References

- [Matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication)
- [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set)
- [Intel Intrinsics Guide](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html)
- [Rust Intrinsics](https://doc.rust-lang.org/core/arch/x86_64/index.html)


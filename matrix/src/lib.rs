use std::os::raw::{c_float, c_int};

unsafe fn naive_matrix_multiply(a: *mut c_float, b: *mut c_float, c: *mut c_float, n: c_int) {
    for i in 0..n {
        for j in 0..n {
            *c.add((i * n + j) as usize) = 0.0;
            for k in 0..n {
                *c.add((i * n + j) as usize) +=
                    *a.add((i * n + k) as usize) * *b.add((k * n + j) as usize);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn matrix_multiply(
    a: *mut c_float,
    b: *mut c_float,
    c: *mut c_float,
    n: c_int,
) {
    naive_matrix_multiply(a, b, c, n);
}

use std::os::raw::{c_float, c_int};

const LOOP: c_int = 256;

const X_START: c_float = -2.0;
const X_END: c_float = 2.0;
const Y_START: c_float = -2.0;
const Y_END: c_float = 2.0;

fn mandelbrot_calc_base(x: f32, y: f32) -> i32 {
    let mut re = x;
    let mut im = y;

    for i in 0..LOOP {
        let re2 = re * re;
        let im2 = im * im;

        if re2 + im2 > 4.0 {
            return i;
        }

        im = 2.0 * re * im + y;
        re = re2 - im2 + x;
    }

    LOOP
}

unsafe fn naive_mandelbrot(width: c_int, height: c_int, plot: *mut c_int) {
    let dx = (X_END - X_START) / (width - 1) as c_float;
    let dy = (Y_END - Y_START) / (height - 1) as c_float;

    for i in 0..height {
        for j in 0..width {
            let x = X_START + j as c_float * dx;
            let y = Y_END - i as c_float * dy;

            *plot.add((i * width + j) as usize) = mandelbrot_calc_base(x, y);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mandelbrot(width: c_int, height: c_int, plot: *mut c_int) {
    naive_mandelbrot(width, height, plot);
}

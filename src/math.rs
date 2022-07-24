//List of all math functions
//Ignore unused functions
#[allow(dead_code)]
pub fn sin(x:f32) -> f32 { x.sin() }
#[allow(dead_code)]
pub fn cos(x:f32) -> f32 { x.cos() }
#[allow(dead_code)]
pub fn tan(x:f32) -> f32 { x.tan() }
#[allow(dead_code)]
pub fn asin(x:f32) -> f32 { x.asin() }
#[allow(dead_code)]
pub fn acos(x:f32) -> f32 { x.acos() }
#[allow(dead_code)]
pub fn atan(x:f32) -> f32 { x.atan() }
#[allow(dead_code)]
pub fn atan2(x:f32, y:f32) -> f32 { x.atan2(y) }
#[allow(dead_code)]
pub fn sinh(x:f32) -> f32 { x.sinh() }
#[allow(dead_code)]
pub fn cosh(x:f32) -> f32 { x.cosh() }
#[allow(dead_code)]
pub fn tanh(x:f32) -> f32 { x.tanh() }
#[allow(dead_code)]
pub fn asinh(x:f32) -> f32 { x.asinh() }
#[allow(dead_code)]
pub fn acosh(x:f32) -> f32 { x.acosh() }
#[allow(dead_code)]
pub fn atanh(x:f32) -> f32 { x.atanh() }
#[allow(dead_code)]
pub fn exp(x:f32) -> f32 { x.exp() }
#[allow(dead_code)]
pub fn exp2(x:f32) -> f32 { x.exp2() }
#[allow(dead_code)]
pub fn ln(x:f32) -> f32 { x.ln() }
#[allow(dead_code)]
pub fn log(x:f32, base:f32) -> f32 { x.log(base) }
#[allow(dead_code)]
pub fn log2(x:f32) -> f32 { x.log2() }
#[allow(dead_code)]
pub fn log10(x:f32) -> f32 { x.log10() }
#[allow(dead_code)]
pub fn cbrt(x:f32) -> f32 { x.cbrt() }
#[allow(dead_code)]
pub fn hypot(x:f32, y:f32) -> f32 { x.hypot(y) }
#[allow(dead_code)]
pub fn sqrt(x:f32) -> f32 { x.sqrt() }
#[allow(dead_code)]
pub fn pow(x:i32, y:u32) -> i32 { x.pow(y) }
#[allow(dead_code)]
pub fn powf(x:f32, y:f32) -> f32 { x.powf(y) }
#[allow(dead_code)]
pub fn cpow(x:i32, y:u32) -> Option<i32> { x.checked_pow(y) }
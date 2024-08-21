#[cfg(not(target_os = "emscripten"))]
extern "C" {
    pub fn fs_load_file(ptr: *const i8, len: u32) -> u32;
    pub fn fs_get_buffer_size(file_id: u32) -> i32;
    pub fn fs_take_buffer(file_id: u32, ptr: *mut u8, max_size: u32);
}

#[cfg(target_os = "emscripten")]
pub use self::emscripten_exports::*;
#[cfg(target_os = "emscripten")]
mod emscripten_exports {
    use super::super::eval;

    pub unsafe fn fs_load_file(ptr: *const i8, len: u32) -> u32 {
        eval(format!(
            "importObject.env.fs_load_file({},{})",
            ptr as usize, len
        ))
    }

    pub unsafe fn fs_get_buffer_size(file_id: u32) -> i32 {
        eval(format!("importObject.env.fs_get_buffer_size({})", file_id))
    }

    pub unsafe fn fs_take_buffer(file_id: u32, ptr: *mut u8, max_size: u32) {
        eval(format!(
            "importObject.env.fs_take_buffer({},{},{})",
            file_id, ptr as usize, max_size
        ))
    }
}

use std::ffi::c_void;

pub struct EngineArrayInfo {
    ptr: *const c_void,
    element_size: usize,
    number_of_elements: usize,
}

pub trait ArrayAPI<A> {
    fn engine_array_info(&self, arr: &A) -> EngineArrayInfo;
    unsafe fn engine_array_cast<'a, T>(&self, arr: &'a A) -> Option<&'a [T]> {
        let info = self.engine_array_info(arr);
        if info.ptr.is_null() {
            return None;
        }

        let size = ::std::mem::size_of::<T>();
        let align = ::std::mem::align_of::<T>();

        assert_eq!(info.ptr as usize % align, 0);
        assert_eq!(info.element_size, size);
        let slice = std::slice::from_raw_parts(info.ptr as *const T, info.number_of_elements);
        Some(slice)
    }
}

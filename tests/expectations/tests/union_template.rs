/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
pub struct NastyStruct {
    pub mIsSome: bool,
    pub mStorage: NastyStruct__bindgen_ty_1,
    pub __bindgen_anon_1: NastyStruct__bindgen_ty_2,
}
#[repr(C)]
pub union NastyStruct__bindgen_ty_1 {
    pub mFoo: *mut ::std::os::raw::c_void,
    pub mDummy: ::std::os::raw::c_ulong,
    _bindgen_union_align: u64,
}
impl Default for NastyStruct__bindgen_ty_1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub union NastyStruct__bindgen_ty_2 {
    pub wat: ::std::os::raw::c_short,
    pub wut: *mut ::std::os::raw::c_int,
    _bindgen_union_align: u64,
}
impl Default for NastyStruct__bindgen_ty_2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl Default for NastyStruct {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub union Whatever {
    pub mTPtr: *mut ::std::os::raw::c_void,
    pub mInt: ::std::os::raw::c_int,
    _bindgen_union_align: u64,
}
impl Default for Whatever {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
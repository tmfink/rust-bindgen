/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl <T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl <T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 40usize);
    assert_eq!(::std::mem::align_of::<C>() , 4usize);
}
#[repr(C)]
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array() {
    assert_eq!(::std::mem::size_of::<C_with_zero_length_array>() , 40usize);
    assert_eq!(::std::mem::align_of::<C_with_zero_length_array>() , 4usize);
}
#[repr(C)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array() {
    assert_eq!(::std::mem::size_of::<C_with_incomplete_array>() , 40usize);
    assert_eq!(::std::mem::align_of::<C_with_incomplete_array>() , 4usize);
}
#[repr(C)]
pub struct C_with_zero_length_array_and_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array() {
    assert_eq!(::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array>()
               , 40usize);
    assert_eq!(::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array>()
               , 4usize);
}
#[repr(C)]
#[derive(Debug)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithDtor() {
    assert_eq!(::std::mem::size_of::<WithDtor>() , 4usize);
    assert_eq!(::std::mem::align_of::<WithDtor>() , 4usize);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union {
    pub d: __BindgenUnionField<f32>,
    pub i: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_Union() {
    assert_eq!(::std::mem::size_of::<Union>() , 4usize);
    assert_eq!(::std::mem::align_of::<Union>() , 4usize);
}
impl Clone for Union {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct WithUnion {
    pub data: Union,
}
#[test]
fn bindgen_test_layout_WithUnion() {
    assert_eq!(::std::mem::size_of::<WithUnion>() , 4usize);
    assert_eq!(::std::mem::align_of::<WithUnion>() , 4usize);
}
impl Clone for WithUnion {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_RealAbstractionWithTonsOfMethods() {
    assert_eq!(::std::mem::size_of::<RealAbstractionWithTonsOfMethods>() ,
               1usize);
    assert_eq!(::std::mem::align_of::<RealAbstractionWithTonsOfMethods>() ,
               1usize);
}
extern "C" {
    #[link_name = "_ZNK32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar(this:
                                                    *const RealAbstractionWithTonsOfMethods);
}
extern "C" {
    #[link_name = "_ZN32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar1(this:
                                                     *mut RealAbstractionWithTonsOfMethods);
}
extern "C" {
    #[link_name = "_ZN32RealAbstractionWithTonsOfMethods3barEi"]
    pub fn RealAbstractionWithTonsOfMethods_bar2(this:
                                                     *mut RealAbstractionWithTonsOfMethods,
                                                 foo: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "_ZN32RealAbstractionWithTonsOfMethods3staEv"]
    pub fn RealAbstractionWithTonsOfMethods_sta();
}
impl Clone for RealAbstractionWithTonsOfMethods {
    fn clone(&self) -> Self { *self }
}
impl RealAbstractionWithTonsOfMethods {
    #[inline]
    pub unsafe fn bar(&self) { RealAbstractionWithTonsOfMethods_bar(&*self) }
    #[inline]
    pub unsafe fn bar1(&mut self) {
        RealAbstractionWithTonsOfMethods_bar1(&mut *self)
    }
    #[inline]
    pub unsafe fn bar2(&mut self, foo: ::std::os::raw::c_int) {
        RealAbstractionWithTonsOfMethods_bar2(&mut *self, foo)
    }
    #[inline]
    pub unsafe fn sta() { RealAbstractionWithTonsOfMethods_sta() }
}

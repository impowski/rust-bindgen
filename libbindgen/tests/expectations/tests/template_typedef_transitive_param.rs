/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wrapper<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wrapper_Wrapped<T> {
    pub t: T,
}
pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct List<Elem> {
    pub next: *mut List<Elem>,
}
impl <Elem> Default for List<Elem> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PersistentRooted<GcThing> {
    pub root_list: List<PersistentRooted<GcThing>>,
}
impl <GcThing> Default for PersistentRooted<GcThing> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

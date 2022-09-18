use core::default::Default;

#[derive(Clone, core::marker::Copy)]
pub struct NodeDataContainer<const SIZE: usize> {
    data: [u8; SIZE]
}

impl<const SIZE: usize> NodeDataContainer<SIZE> {
    pub const fn new() -> Self { Self{data: [0u8; SIZE]} }
    pub fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }
}

impl<const SIZE: usize> Default for NodeDataContainer<SIZE> {
    fn default() -> Self { Self{data: [(); SIZE].map(|_| Default::default())} }
}

#[derive(Clone, core::marker::Copy)]
pub struct EdgeDataContainer<const SIZE: usize> {
    data: [u8; SIZE]
}

impl<const SIZE: usize> EdgeDataContainer<SIZE> {
    pub const fn new() -> Self { Self{data: [0u8; SIZE]} }
    pub fn as_ptr(&self) -> *const u8 { self.data.as_ptr() }
}

impl<const SIZE: usize> Default for EdgeDataContainer<SIZE> {
    fn default() -> Self { Self{data: [(); SIZE].map(|_| Default::default())} }
}
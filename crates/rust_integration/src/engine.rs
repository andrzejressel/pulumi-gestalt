use std::rc::Rc;
use pulumi_gestalt_core as core;

pub struct Engine<T> {
    pub(crate) inner: Rc<core::Engine<T>>,
}

pub struct Output<T> {
    pub(crate) inner: core::Output<T>,
    pub(crate) engine: Rc<core::Engine<T>>,
}

pub struct RegisterResourceOutput {
    pub(crate) inner: core::RegisterResourceOutput,
}

impl<T> Engine<T> {
    pub(crate) fn new(inner: core::Engine<T>) -> Self {
        Self { inner: Rc::new(inner) }
    }
}

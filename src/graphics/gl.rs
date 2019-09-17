
use core::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct Gl
{
    pub inner: Rc<glium::Display>,
}

impl Deref for Gl
{
    type Target = glium::Display;

    fn deref(&self) -> &glium::Display
    {
        &self.inner
    }
}
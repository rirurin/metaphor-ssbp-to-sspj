use std::ffi::CStr;
use std::io::{Seek, Write};
use std::marker::PhantomData;
use quick_xml::events::BytesText;
use quick_xml::Writer;

#[allow(dead_code)]
pub(crate) unsafe fn from_bytes<T>(s: &[u8], o: usize) -> &T {
    unsafe { &*(s.as_ptr().add(o) as *const T) }
}

#[repr(C)]
#[derive(Debug)]
pub struct Ptr<T> {
    offset: u32,
    _type_marker: PhantomData<T>
}

impl<T> Ptr<T> {
    pub fn value(&self, s: &[u8]) -> &T {
        unsafe { &*(s.as_ptr().add(self.offset as usize) as *const T) }
    }

    pub fn array(&self, s: &[u8], c: usize) -> &[T] {
        unsafe { std::slice::from_raw_parts(s.as_ptr().add(self.offset as usize) as _, c) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct StringPtr(u32);

impl StringPtr {
    pub fn value(&self, s: &[u8]) -> &str {
        unsafe { CStr::from_ptr(s.as_ptr().add(self.0 as usize) as _).to_str().unwrap() }
    }
}

pub(crate) fn create_blank_element<W>(writer: &mut Writer<W>, name: &str)
    -> std::io::Result<()>
where W: Write + Seek
{
    writer.create_element(name)
        .write_text_content(BytesText::new(""))?;
    Ok(())
}

pub(crate) fn create_name_list<W>(name: &str, values: &[String], writer: &mut Writer<W>)
    -> std::io::Result<()>
where W: Write + Seek
{
    writer.create_element(name)
        .write_inner_content(|writer| {
            for value in values {
                writer.create_element("value")
                    .write_text_content(BytesText::new(value))?;
            }
            Ok(())
        })?;
    Ok(())
}
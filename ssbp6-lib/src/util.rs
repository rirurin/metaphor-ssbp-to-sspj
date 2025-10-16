use std::ffi::CStr;
use std::io::{Seek, Write};
use std::marker::PhantomData;
use quick_xml::events::BytesText;
use quick_xml::Writer;
use crate::anime::AttributeKeyframe;

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

pub(crate) fn to_xml_anime_settings<W: Write + Seek>(writer: &mut Writer<W>) -> std::io::Result<()> {
    writer.create_element("fps")
        .write_text_content(BytesText::new("30"))?;
    writer.create_element("frameCount")
        .write_text_content(BytesText::new("11"))?;
    writer.create_element("sortMode")
        .write_text_content(BytesText::new("prio"))?;
    writer.create_element("canvasSize")
        .write_text_content(BytesText::new("320 320"))?;
    writer.create_element("pivot")
        .write_text_content(BytesText::new("0 0"))?;
    writer.create_element("bgColor")
        .write_text_content(BytesText::new("FF323232"))?;
    writer.create_element("gridSize")
        .write_text_content(BytesText::new("32"))?;
    writer.create_element("gridColor")
        .write_text_content(BytesText::new("FF808080"))?;
    writer.create_element("ik_depth")
        .write_text_content(BytesText::new("3"))?;
    writer.create_element("startFrame")
        .write_text_content(BytesText::new("0"))?;
    writer.create_element("endFrame")
        .write_text_content(BytesText::new("10"))?;
    writer.create_element("bgSettings")
        .write_inner_content(|writer| to_xml_anime_bg_settings(writer))?;
    writer.create_element("outStartNum")
        .write_text_content(BytesText::new("0"))?;
    Ok(())
}

pub(crate) fn to_xml_anime_bg_settings<W: Write + Seek>(writer: &mut Writer<W>) -> std::io::Result<()> {
    for i in 0..2 {
        writer.create_element("value")
            .write_inner_content(|writer| {
                create_blank_element(writer, "imagePath")?;
                writer.create_element("imageDisp")
                    .write_text_content(BytesText::new("1"))?;
                writer.create_element("imageOffset")
                    .write_text_content(BytesText::new("0 0"))?;
                writer.create_element("imageCanvas")
                    .write_text_content(BytesText::new("0 0"))?;
                writer.create_element("imagePivot")
                    .write_text_content(BytesText::new("0 0"))?;
                Ok(())
            })?;
    }
    Ok(())
}
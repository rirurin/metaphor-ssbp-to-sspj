use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Seek, SeekFrom, Write};
use glam::UVec2;
use quick_xml::events::BytesText;
use quick_xml::Writer;
use crate::util::{Ptr, StringPtr};

#[derive(Debug)]
#[allow(dead_code)]
pub struct CastError((&'static str, usize));
impl Error for CastError {}
impl Display for CastError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum InterpolateType {
    none,
    linear,
    hermite,
    bezier,
    acceleration,
    deceleration,
}

impl TryFrom<u32> for InterpolateType {
    type Error = CastError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::deceleration as u32 {
            Ok(unsafe { std::mem::transmute(value)})
        } else {
            Err(CastError(("InterpolateType", value as usize)))
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TexWrapMode {
    clamp,
    repeat,
    mirror,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TexFilterMode {
    nearlest,
    linear,
}

#[repr(C)]
#[derive(Debug)]
pub struct CellEntry {
    name: StringPtr,
    cell_map: Ptr<CellMap>,
    index: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    pivot_x: f32,
    pivot_y: f32,
    u1: f32,
    v1: f32,
    u2: f32,
    v2: f32
}

const _: () = {
    ["Size of CellEntry"][size_of::<CellEntry>() - 0x2c];
};

impl CellEntry {
    pub fn get_cell_map(&self, binary: &[u8]) -> &CellMap {
        self.cell_map.value(binary)
    }
}

impl PartialEq for CellEntry {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for CellEntry {}

impl Hash for CellEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl PartialOrd for CellEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for CellEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl CellEntry {
    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.name.value(binary)
    }
    pub fn to_xml<W: Write>(&self, writer: &mut Writer<W>, binary: &[u8]) -> std::io::Result<()> {
        writer.create_element("cell")
            .write_inner_content(|writer| {
                let cell_name = self.name.value(binary);
                writer.create_element("name").write_text_content(BytesText::new(cell_name))?;
                let position = format!("{} {}", self.x, self.y);
                writer.create_element("pos").write_text_content(BytesText::new(&position))?;
                let size = format!("{} {}", self.width, self.height);
                writer.create_element("size").write_text_content(BytesText::new(&size))?;
                let pivot = format!("{} {}", self.pivot_x, self.pivot_y);
                writer.create_element("pivot").write_text_content(BytesText::new(&pivot))?;
                writer.create_element("rotated").write_text_content(BytesText::new("0"))?;
                writer.create_element("orgImageName").write_text_content(BytesText::new(""))?;
                writer.create_element("posStable").write_text_content(BytesText::new("0"))?;
                writer.create_element("ismesh").write_text_content(BytesText::new("0"))?;
                writer.create_element("divtype").write_text_content(BytesText::new("unknown"))?;
                writer.create_element("innerPoint").write_empty()?;
                writer.create_element("outerPoint").write_empty()?;
                writer.create_element("meshPointList").write_empty()?;
                writer.create_element("meshTriList").write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CellMap {
    name: StringPtr,
    image_path: StringPtr,
    index: u16,
    wrap_mode: TexWrapMode,
    filter_mode: TexFilterMode
}

const _: () = {
    ["Size of CellMap"][size_of::<CellMap>() - 0x10];
};

impl PartialEq for CellMap {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for CellMap {}

impl Hash for CellMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl PartialOrd for CellMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for CellMap {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl CellMap {
    pub fn get_index(&self) -> u16 {
        self.index
    }
    pub fn get_wrap_mode(&self) -> TexWrapMode {
        self.wrap_mode
    }
    pub fn get_filter_mode(&self) -> TexFilterMode {
        self.filter_mode
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Cell<'a> {
    data: &'a CellMap,
    list: Vec<&'a CellEntry>
}

impl<'a> PartialEq for Cell<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(other.data)
    }
}

impl<'a> Eq for Cell<'a> {}

impl<'a> Hash for Cell<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

impl<'a> PartialOrd for Cell<'a>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<'a> Ord for Cell<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<'a> Cell<'a> {
    pub fn new(data: &'a CellMap, first: &'a CellEntry) -> Self {
        Self { data, list: vec![first] }
    }

    pub fn add(&mut self, new: &'a CellEntry) {
        self.list.push(new);
    }

    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.data.name.value(binary)
    }

    pub fn get_name_by_index(&self, index: u16, binary: &[u8]) -> Option<&str> {
        self.list.iter().find(|c| c.index == index)
            .map(|c| c.get_name(binary))
    }

    // .ssce XML format:
    //
    // <?xml version="1.0" encoding="utf-8" standalone="yes"?>
    // <SpriteStudioCellMap version = "2.00.00">
    //  <name>cellmap.name</name>
    //  <exportPath></exportPath>
    //  <generator>SpriteStudio</generator>
    //  <packed>0</packed>
    //  <pixelSize>image_size.x image_size.y</pixelSize>
    //  <overrideTexSettings>0</overrideTexSettings>
    //  <wrapMode>clamp</wrapMode>
    //  <filterMode>linear</filterMode>
    //  <imagePathAtImport></imagePathAtImport>
    //  <packInfoFilePath></packInfoFilePath>
    //  <texPackSettings>
    //   	<maxSize>4096 4096</maxSize>
    //   	<forcePo2>1</forcePo2>
    //   	<forceSquare>0</forceSquare>
    //   	<margin>0</margin>
    //   	<padding>1</padding>
    //  </texPackSettings>
    //  <cells>
    //      {foreach cell in map}
    //      <cell>
    //          <name>cell.name</name>
    //          <pos>cell.x cell.y</pos>
    //          <size>cell.width cell.height</size>
    //          <pivot>cell.pivot_x cell.pivot_y</pivot>
    //          <rotated>0</rotated>
    //          <orgImageName></orgImageName>
    //          <posStable>0</posStable>
    //          <ismesh>0</ismesh>
    //          <divtype>unknown</divtype>
    //      </cell>
    //      {end foreach}
    //  </cells>
    // </SpriteStudioCellMap>
    pub fn to_xml<F>(&self, binary: &[u8], get_img_params: F)
        -> Result<Vec<u8>, Box<dyn Error>>
    where F: Fn(&str) -> std::io::Result<(String, UVec2)>,
    {
        let xml_fmt = "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>\n";
        let mut cursor = Cursor::new(xml_fmt.as_bytes().to_vec());
        cursor.seek(SeekFrom::End(0))?;
        let mut writer = Writer::new_with_indent(cursor, '\t' as u8, 1);
        writer.create_element("SpriteStudioCellMap")
            .with_attributes([("version", "2.00.00")])
            .write_inner_content(|writer| self.to_xml_body(writer, binary, get_img_params))?;
        Ok(writer.into_inner().into_inner())
    }
    pub(crate) fn to_xml_body<F, W: Write>(&self, writer: &mut Writer<W>, binary: &[u8], get_img_params: F) -> std::io::Result<()>
    where F: Fn(&str) -> std::io::Result<(String, UVec2)> {
        writer.create_element("name")
            .write_text_content(BytesText::new(self.data.name.value(binary)))?;
        writer.create_element("exportPath").write_text_content(BytesText::new(""))?;
        writer.create_element("generator").write_text_content(BytesText::new("SpriteStudio"))?;
        writer.create_element("packed").write_text_content(BytesText::new("0"))?;
        let (img_path, dims) = get_img_params(self.data.image_path.value(binary))?;
        writer.create_element("imagePath")
            .write_text_content(BytesText::new(&img_path))?;
        writer.create_element("pixelSize")
            .write_text_content(BytesText::new(&format!("{} {}", dims.x, dims.y)))?;
        writer.create_element("overrideTexSettings").write_text_content(BytesText::new("0"))?;
        writer.create_element("wrapMode")
            .write_text_content(BytesText::new(&format!("{:?}", self.data.wrap_mode)))?;
        writer.create_element("filterMode")
            .write_text_content(BytesText::new(&format!("{:?}", self.data.filter_mode)))?;
        writer.create_element("imagePathAtImport").write_text_content(BytesText::new(""))?;
        writer.create_element("packInfoFilePath").write_text_content(BytesText::new(""))?;
        tex_pack_settings_to_xml(writer)?;
        writer.create_element("cells")
            .write_inner_content(|writer| {
                for cell in self.list.iter() {
                    cell.to_xml(writer, binary)?;
                }
                Ok(())
            })?;
         Ok(())
    }
}

pub(crate) fn tex_pack_settings_to_xml<W: Write>(writer: &mut Writer<W>) -> std::io::Result<()> {
    writer.create_element("texPackSettings")
        .write_inner_content(|writer| {
            writer.create_element("maxSize").write_text_content(BytesText::new("4096 4096"))?;
            writer.create_element("forcePo2").write_text_content(BytesText::new("1"))?;
            writer.create_element("forceSquare").write_text_content(BytesText::new("0"))?;
            writer.create_element("margin").write_text_content(BytesText::new("0"))?;
            writer.create_element("padding").write_text_content(BytesText::new("1"))?;
            Ok(())
        })?;
    Ok(())
}
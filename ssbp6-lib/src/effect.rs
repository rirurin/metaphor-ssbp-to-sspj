// .ssee XML format:
//
// <?xml version="1.0" encoding="utf-8" standalone="yes"?>
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Cursor, Seek, SeekFrom, Write};
use quick_xml::events::BytesText;
use quick_xml::Writer;
use crate::cell::CellEntry;
use crate::util::{create_blank_element, Ptr, StringPtr};

#[derive(Debug)]
pub enum EffectError {
    GotBaseEffect
}

impl Error for EffectError {}

impl Display for EffectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum EffectNodeType {
    Root,
    Emmiter,
    Particle,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum BehaviorType {
    Base,
    Basic	,
    RndSeedChange ,
    Delay,
    Gravity,
    Position,
    //TransPosition,
    Rotation,
    TransRotation,
    TransSpeed,
    TangentialAcceleration,
    InitColor,
    TransColor,
    AlphaFade,
    Size,
    TransSize,
    PointGravity,
    TurnToDirectionEnabled,
    InfiniteEmitEnabled,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum RenderBlendType {
    Mix,
    Add,
}

#[repr(C)]
#[derive(Debug)]
pub struct Behavior {
    _type: BehaviorType
}

impl Behavior {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>, binary: &[u8]) -> std::io::Result<()> {
        match self._type {
            BehaviorType::Base => return Err(std::io::Error::other(Box::new(EffectError::GotBaseEffect))),
            BehaviorType::Basic => unsafe { std::mem::transmute::<_, &Basic>(self) }.to_xml(writer),
            BehaviorType::RndSeedChange => unsafe { std::mem::transmute::<_, &RndSeedChange>(self) }.to_xml(writer),
            BehaviorType::Delay => unsafe { std::mem::transmute::<_, &Delay>(self) }.to_xml(writer),
            BehaviorType::Gravity => unsafe { std::mem::transmute::<_, &Gravity>(self) }.to_xml(writer),
            BehaviorType::Position => unsafe { std::mem::transmute::<_, &Position>(self) }.to_xml(writer),
            BehaviorType::Rotation => unsafe { std::mem::transmute::<_, &Rotation>(self) }.to_xml(writer),
            BehaviorType::TransRotation => unsafe { std::mem::transmute::<_, &TransRotation>(self) }.to_xml(writer),
            BehaviorType::TransSpeed => unsafe { std::mem::transmute::<_, &TransSpeed>(self) }.to_xml(writer),
            BehaviorType::TangentialAcceleration => unsafe { std::mem::transmute::<_, &TangentialAcceleration>(self) }.to_xml(writer),
            BehaviorType::InitColor => unsafe { std::mem::transmute::<_, &InitColor>(self) }.to_xml(writer),
            BehaviorType::TransColor => unsafe { std::mem::transmute::<_, &TransColor>(self) }.to_xml(writer),
            BehaviorType::AlphaFade => unsafe { std::mem::transmute::<_, &AlphaFade>(self) }.to_xml(writer),
            BehaviorType::Size => unsafe { std::mem::transmute::<_, &Size>(self) }.to_xml(writer),
            BehaviorType::TransSize => unsafe { std::mem::transmute::<_, &TransSize>(self) }.to_xml(writer),
            BehaviorType::PointGravity => unsafe { std::mem::transmute::<_, &PointGravity>(self) }.to_xml(writer),
            BehaviorType::TurnToDirectionEnabled => unsafe { std::mem::transmute::<_, &TurnToDirectionEnabled>(self) }.to_xml(writer),
            BehaviorType::InfiniteEmitEnabled => unsafe { std::mem::transmute::<_, &InfiniteEmitEnabled>(self) }.to_xml(writer),
        }?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Basic {
    _super: Behavior,
    priority: u32,
    maximum_particle: u32,
    attime_create: u32,
    interval: u32,
    lifetime: u32,
    speed_min: f32,
    speed_max: f32,
    lifespan_min: u32,
    lifespan_max: u32,
    angle: f32,
    angle_variance: f32
}

impl Basic {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "Basic"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("Basic"))?;
                writer.create_element("priority")
                    .write_text_content(BytesText::new(&format!("{}", self.priority)))?;
                writer.create_element("maxximumParticle")
                    .write_text_content(BytesText::new(&format!("{}", self.maximum_particle)))?;
                writer.create_element("attimeCreate")
                    .write_text_content(BytesText::new(&format!("{}", self.attime_create)))?;
                writer.create_element("interval")
                    .write_text_content(BytesText::new(&format!("{}", self.interval)))?;
                writer.create_element("lifetime")
                    .write_text_content(BytesText::new(&format!("{}", self.lifetime)))?;
                writer.create_element("speed")
                    .with_attributes([
                        ("value", format!("{}", self.speed_min).as_ref()),
                        ("subvalue", format!("{}", self.speed_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("lifespan")
                    .with_attributes([
                        ("value", format!("{}", self.lifespan_min).as_ref()),
                        ("subvalue", format!("{}", self.lifespan_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("angle")
                    .write_text_content(BytesText::new(&format!("{}", self.angle)))?;
                writer.create_element("angleVariance")
                    .write_text_content(BytesText::new(&format!("{}", self.angle_variance)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RndSeedChange {
    _super: Behavior,
    seed: u32
}

impl RndSeedChange {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "OverWriteSeed"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("OverWriteSeed"))?;
                writer.create_element("seed")
                    .write_text_content(BytesText::new(&format!("{}", self.seed)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Delay {
    _super: Behavior,
    delay_time: u32
}

impl Delay {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "Delay"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("Delay"))?;
                writer.create_element("DelayTime")
                    .write_text_content(BytesText::new(&format!("{}", self.delay_time)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Gravity {
    _super: Behavior,
    gravity_x: f32,
    gravity_y: f32,
}

impl Gravity {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "Gravity"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("Gravity"))?;
                writer.create_element("Gravity")
                    .write_text_content(BytesText::new(&format!("{} {}", self.gravity_x, self.gravity_y)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Position {
    _super: Behavior,
    offset_x_min: f32,
    offset_x_max: f32,
    offset_y_min: f32,
    offset_y_max: f32,
}

impl Position {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "init_position"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("init_position"))?;
                writer.create_element("OffsetX")
                    .with_attributes([
                        ("value", format!("{}", self.offset_x_min).as_ref()),
                        ("subvalue", format!("{}", self.offset_x_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("OffsetY")
                    .with_attributes([
                        ("value", format!("{}", self.offset_y_min).as_ref()),
                        ("subvalue", format!("{}", self.offset_y_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Rotation {
    _super: Behavior,
    rotation_min: f32,
    rotation_max: f32,
    rotation_add_min: f32,
    rotation_add_max: f32,
}

impl Rotation {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "init_rotation"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("init_rotation"))?;
                writer.create_element("Rotation")
                    .with_attributes([
                        ("value", format!("{}", self.rotation_min).as_ref()),
                        ("subvalue", format!("{}", self.rotation_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("RotationAdd")
                    .with_attributes([
                        ("value", format!("{}", self.rotation_add_min).as_ref()),
                        ("subvalue", format!("{}", self.rotation_add_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TransRotation {
    _super: Behavior,
    rotation_factor: f32,
    end_life_time_per: f32
}

impl TransRotation {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "trans_rotation"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("trans_rotation"))?;
                writer.create_element("RotationFactor")
                    .write_text_content(BytesText::new(&format!("{}", self.rotation_factor)))?;
                writer.create_element("EndLifePerTime")
                    .write_text_content(BytesText::new(&format!("{}", self.end_life_time_per)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TransSpeed {
    _super: Behavior,
    speed_min: f32,
    speed_max: f32
}

impl TransSpeed {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "trans_speed"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("trans_speed"))?;
                writer.create_element("Speed")
                    .with_attributes([
                        ("value", format!("{}", self.speed_min).as_ref()),
                        ("subvalue", format!("{}", self.speed_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TangentialAcceleration {
    _super: Behavior,
    acceleration_min: f32,
    acceleration_max: f32
}

impl TangentialAcceleration {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "add_tangentiala"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("add_tangentiala"))?;
                writer.create_element("Acceleration")
                    .with_attributes([
                        ("value", format!("{}", self.acceleration_min).as_ref()),
                        ("subvalue", format!("{}", self.acceleration_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InitColor {
    _super: Behavior,
    color_min: u32,
    color_max: u32
}

impl InitColor {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "init_vertexcolor"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("init_vertexcolor"))?;
                writer.create_element("Color")
                    .with_attributes([
                        ("value", format!("{:X}", self.color_min).as_ref()),
                        ("subvalue", format!("{:X}", self.color_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TransColor {
    _super: Behavior,
    color_min: u32,
    color_max: u32
}

impl TransColor {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "trans_vertexcolor"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("trans_vertexcolor"))?;
                writer.create_element("Color")
                    .with_attributes([
                        ("value", format!("{:X}", self.color_min).as_ref()),
                        ("subvalue", format!("{:X}", self.color_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AlphaFade {
    _super: Behavior,
    disprange_min: f32,
    disprange_max: f32
}

impl AlphaFade {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "trans_colorfade"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("trans_colorfade"))?;
                writer.create_element("Disprange")
                    .with_attributes([
                        ("value", format!("{}", self.disprange_min).as_ref()),
                        ("subvalue", format!("{}", self.disprange_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Size {
    _super: Behavior,
    size_x_min: f32,
    size_x_max: f32,
    size_y_min: f32,
    size_y_max: f32,
    scale_factor_min: f32,
    scale_factor_max: f32,
}

impl Size {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "init_size"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("init_size"))?;
                writer.create_element("SizeX")
                    .with_attributes([
                        ("value", format!("{}", self.size_x_min).as_ref()),
                        ("subvalue", format!("{}", self.size_x_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("SizeY")
                    .with_attributes([
                        ("value", format!("{}", self.size_y_min).as_ref()),
                        ("subvalue", format!("{}", self.size_y_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("ScaleFactor")
                    .with_attributes([
                        ("value", format!("{}", self.scale_factor_min).as_ref()),
                        ("subvalue", format!("{}", self.scale_factor_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TransSize {
    _super: Behavior,
    size_x_min: f32,
    size_x_max: f32,
    size_y_min: f32,
    size_y_max: f32,
    scale_factor_min: f32,
    scale_factor_max: f32,
}

impl TransSize {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "trans_size"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("trans_size"))?;
                writer.create_element("SizeX")
                    .with_attributes([
                        ("value", format!("{}", self.size_x_min).as_ref()),
                        ("subvalue", format!("{}", self.size_x_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("SizeY")
                    .with_attributes([
                        ("value", format!("{}", self.size_y_min).as_ref()),
                        ("subvalue", format!("{}", self.size_y_max).as_ref()),
                    ]).write_empty()?;
                writer.create_element("ScaleFactor")
                    .with_attributes([
                        ("value", format!("{}", self.scale_factor_min).as_ref()),
                        ("subvalue", format!("{}", self.scale_factor_max).as_ref()),
                    ]).write_empty()?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointGravity {
    _super: Behavior,
    position_x: f32,
    position_y: f32,
    power: f32,
}

impl PointGravity {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "add_pointgravity"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("add_pointgravity"))?;
                writer.create_element("Position")
                    .write_text_content(BytesText::new(&format!("{} {}", self.position_x, self.position_y)))?;
                writer.create_element("Power")
                    .write_text_content(BytesText::new(&format!("{}", self.power)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TurnToDirectionEnabled {
    _super: Behavior,
    rotation: f32,
}

impl TurnToDirectionEnabled {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "TurnToDirection"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("TurnToDirection"))?;
                writer.create_element("Rotation")
                    .write_text_content(BytesText::new(&format!("{}", self.rotation)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InfiniteEmitEnabled {
    _super: Behavior,
    flag: u32,
}

impl InfiniteEmitEnabled {
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>) -> std::io::Result<()> {
        writer.create_element("value")
            .with_attribute(("name", "InfiniteEmit"))
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new("InfiniteEmit"))?;
                writer.create_element("calcGen")
                    .write_text_content(BytesText::new(&format!("{}", self.flag)))?;
                Ok(())
            })?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Node {
    array_index: i16,
    parent_index: i16,
    _type: EffectNodeType,
    cell_index: i16,
    blend_type: RenderBlendType,
    num_behavior: u16,
    behaviors: Ptr<Ptr<Behavior>>
}

impl Node {
    pub fn get_array_index(&self) -> i16 {
        self.array_index
    }
    pub fn get_parent_index(&self) -> i16 {
        self.parent_index
    }
    pub fn get_type(&self) -> EffectNodeType {
        self._type
    }
    pub fn get_cell_index(&self) -> i16 {
        self.cell_index
    }
    pub fn get_blend_type(&self) -> RenderBlendType {
        self.blend_type
    }
    pub fn get_behaviors(&self, binary: &[u8]) -> &[Ptr<Behavior>] {
        self.behaviors.array(binary, self.num_behavior as usize)
    }
    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>,
        binary: &[u8], cells: &[CellEntry], tracker: &mut NodeTracker) -> std::io::Result<()> {
        let effect_name = match self._type {
            EffectNodeType::Root => "Root".to_string(),
            EffectNodeType::Emmiter => format!("Emitter_{}", tracker.get_emitter()),
            EffectNodeType::Particle => format!("Particle_{}", tracker.get_particle()),
        };
        writer.create_element("name")
            .write_text_content(BytesText::new(&effect_name))?;
        writer.create_element("type")
            .write_text_content(BytesText::new(&format!("{:?}", self._type)))?;
        writer.create_element("arrayIndex")
            .write_text_content(BytesText::new(&format!("{}", self.array_index)))?;
        writer.create_element("parentIndex")
            .write_text_content(BytesText::new(&format!("{}", self.parent_index)))?;
        writer.create_element("visible")
            .write_text_content(BytesText::new("1"))?;
        if self._type == EffectNodeType::Root {
            return Ok(());
        }

        writer.create_element("behavior")
        .write_inner_content(|writer| {
            let cell_name = match self.cell_index {
                -1 => "", v => cells[v as usize].get_name(binary)
            };
            writer.create_element("CellName")
                .write_text_content(BytesText::new(cell_name))?;
            let cell_map_name = match self.cell_index {
                -1 => String::new(), v => format!("{}.ssce", cells[v as usize].get_cell_map(binary).get_name(binary))
            };
            writer.create_element("CellMapName")
                .write_text_content(BytesText::new(&cell_map_name))?;
            writer.create_element("BlendType")
                .write_text_content(BytesText::new(&format!("{:?}", self.blend_type)))?;
            let list = writer.create_element("list");
            match self.num_behavior {
                0 => list.write_empty(),
                _ => list.write_inner_content(|writer| {
                    for p_behavior in self.get_behaviors(binary) {
                        p_behavior.value(binary).to_xml(writer, binary)?;
                    }
                    Ok(())
                })
            }?;
            Ok(())
        })?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct NodeTracker {
    emitters: usize,
    particles: usize
}

impl NodeTracker {
    pub fn new() -> Self {
        Self {
            emitters: 1,
            particles: 1
        }
    }
    pub fn get_emitter(&mut self) -> usize {
        let id = self.emitters;
        self.emitters += 1;
        id
    }
    pub fn get_particle(&mut self) -> usize {
        let id = self.particles;
        self.particles += 1;
        id
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Effect {
    name: StringPtr,
    fps: u16,
    is_lock_random_seed: u16,
    lock_random_seed: u16,
    layout_scale_x: u16,
    layout_scale_y: u16,
    num_node_list: u16,
    nodes: Ptr<Node>
}

impl Effect {
    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.name.value(binary)
    }
    pub fn get_fps(&self) -> u16 {
        self.fps
    }
    pub fn get_is_lock_random_seed(&self) -> u16 {
        self.is_lock_random_seed
    }
    pub fn get_lock_random_seed(&self) -> u16 {
        self.lock_random_seed
    }
    pub fn get_layout_scale_x(&self) -> u16 {
        self.layout_scale_x
    }
    pub fn get_layout_scale_y(&self) -> u16 {
        self.layout_scale_y
    }
    pub fn get_nodes(&self, binary: &[u8]) -> &[Node] {
        self.nodes.array(binary, self.num_node_list as usize)
    }

    pub fn to_xml(&self, binary: &[u8], cells: &[CellEntry]) -> Result<Vec<u8>, Box<dyn Error>> {
        let xml_fmt = "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>\n";
        let mut cursor = Cursor::new(xml_fmt.as_bytes().to_vec());
        cursor.seek(SeekFrom::End(0))?;
        let mut writer = Writer::new_with_indent(cursor, '\t' as u8, 1);
        writer.create_element("SpriteStudioEffect")
            .with_attributes([("version", "2.00.00")])
            .write_inner_content(|writer| self.to_xml_body(writer, binary, cells))?;
        Ok(writer.into_inner().into_inner())
    }

    pub(crate) fn to_xml_body<W: Write + Seek>(&self, writer: &mut Writer<W>,
        binary: &[u8], cells: &[CellEntry]) -> std::io::Result<()> {
        writer.create_element("name")
            .write_text_content(BytesText::new(self.get_name(binary)))?;
        create_blank_element(writer, "exportPath")?;
        writer.create_element("effectData")
            .write_inner_content(|writer| {
                writer.create_element("lockRandSeed")
                    .write_text_content(BytesText::new(&format!("{}", self.lock_random_seed)))?;
                writer.create_element("isLockRandSeed")
                    .write_text_content(BytesText::new(&format!("{}", self.is_lock_random_seed)))?;
                writer.create_element("fps")
                    .write_text_content(BytesText::new(&format!("{}", self.fps)))?;
                writer.create_element("bgColor")
                    .write_text_content(BytesText::new("FF000000"))?;
                writer.create_element("renderVersion")
                    .write_text_content(BytesText::new("2"))?;
                writer.create_element("nodeList")
                    .write_inner_content(|writer| {
                        let mut tracker = NodeTracker::new();
                        for node in self.get_nodes(binary) {
                            writer.create_element("node")
                                .write_inner_content(|writer| {
                                    node.to_xml(writer, binary, cells, &mut tracker)?;
                                    Ok(())
                                })?;
                        }
                        Ok(())
                    })?;
                Ok(())
            })?;
        Ok(())
    }
}
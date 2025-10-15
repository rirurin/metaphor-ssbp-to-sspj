// .ssae XML format:
//
// <?xml version="1.0" encoding="utf-8" standalone="yes"?>
// <SpriteStudioAnimePack version = "2.00.00">
//  <settings>
//      <fps>anim.fps</fps>
//      <frameCount>anim.totalFrames</frameCount>
//      <sortMode>prio</sortMode>
//  </settings>
// </SpriteStudioAnimePack>

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Cursor, Seek, SeekFrom, Write};
use bitflags::bitflags;
use glam::{Vec2, Vec3};
use quick_xml::events::BytesText;
use quick_xml::events::attributes::Attribute;
use quick_xml::Writer;
use crate::util::{create_blank_element, create_name_list, Ptr, StringPtr};

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PartType {
    null,			//< Has no region, only SRT information. However, circular collision detection can be set.
    normal,			//< Normal part. Has a region. May not have an image.
    text,			//< Text (Reserved - Not implemented)
    instance,		//< Instance. Reference to other animations or parts. Replaces scene edit mode.
    armature,		//< Bone Part
    effect,			//< Effect
    mesh,			//< Mesh Part
    movenode,		//< Action Origin
    constraint,		//< Constraint
    mask,			//< Mask
    joint,			//< Mesh-Bone Association Part
    bonepoint,		//< Bone Point
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BlendType {
    mix,
    mul,
    add,
    sub,
    mulalpha,
    screen,
    exclusion,
    invert,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BoundsType {
    none,			//< Not used for collision detection.
    quad,			//< A freely deformable quadrilateral. The area enclosed by the four corners after applying vertex deformation, etc. Heaviest.
    aabb,			//< Collision detection using a non-rotating bounding rectangle
    circle,			//< Determine collision based on distance using the radius of a perfect circle
    circle_smin,	//< Determine collision based on distance using the radius of a perfect circle (scale uses the minimum value of x,y)
    circle_smax,	//< Determine collision based on distance using the radius of a perfect circle (scale uses the maximum value of x,y)
}

#[repr(C)]
#[derive(Debug)]
pub struct PartEntry {
    name: StringPtr,
    index: i16,
    parent_index: i16,
    _type: PartType,
    bounds_type: BoundsType,
    alpha_blend_type: BlendType,
    ref_name: StringPtr,
    effect_name: StringPtr,
    color_label: StringPtr,
    mask_influence: u16,
}

const _: () = {
    ["Size of PartEntry"][size_of::<PartEntry>() - 0x20];
};

#[derive(Debug)]
pub enum PartError {
    MissingPathInRefName(String)
}

impl Error for PartError {}
impl Display for PartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl PartEntry {
    pub fn get_index(&self) -> i16 {
        self.index
    }
    pub fn get_parent_index(&self) -> i16 {
        self.parent_index
    }
    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.name.value(binary)
    }
    pub fn get_type(&self) -> PartType {
        self._type
    }
    pub fn get_bounds_type(&self) -> BoundsType {
        self.bounds_type
    }
    pub fn get_alpha_blend_type(&self) -> BlendType {
        self.alpha_blend_type
    }
    pub fn get_ref_name(&self, binary: &[u8]) -> &str {
        self.ref_name.value(binary)
    }
    pub fn get_effect_name(&self, binary: &[u8]) -> &str {
        self.effect_name.value(binary)
    }
    pub fn get_color_label(&self, binary: &[u8]) -> &str {
        self.color_label.value(binary)
    }

    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>, binary: &[u8])
        -> std::io::Result<()> {
        writer.create_element("name")
            .write_text_content(BytesText::new(self.name.value(binary)))?;
        writer.create_element("arrayIndex")
            .write_text_content(BytesText::new(&format!("{}", self.index)))?;
        writer.create_element("parentIndex")
            .write_text_content(BytesText::new(&format!("{}", self.parent_index)))?;
        writer.create_element("type")
            .write_text_content(BytesText::new(&format!("{:?}", self._type)))?;
        writer.create_element("boundsType")
            .write_text_content(BytesText::new(&format!("{:?}", self.bounds_type)))?;
        writer.create_element("inheritType")
            .write_text_content(BytesText::new(match self.parent_index {
                -1 => "self", _ => "parent"
            }))?;
        writer.create_element("ineheritRates")
            .write_inner_content(|writer| {
                writer.create_element("ALPH")
                    .write_text_content(BytesText::new("1"))?;
                writer.create_element("IFLH")
                    .write_text_content(BytesText::new("0"))?;
                writer.create_element("IFLV")
                    .write_text_content(BytesText::new("0"))?;
                writer.create_element("FLPH")
                    .write_text_content(BytesText::new("0"))?;
                writer.create_element("FLPV")
                    .write_text_content(BytesText::new("0"))?;
                writer.create_element("HIDE")
                    .write_text_content(BytesText::new("0"))?;
                Ok(())
            })?;
        let ref_anime_name = self.ref_name.value(binary);
        if !ref_anime_name.is_empty() {
            let (pack, anim) = ref_anime_name.split_once("/")
                .ok_or(std::io::Error::other(PartError::MissingPathInRefName(ref_anime_name.to_string())))?;
            writer.create_element("refAnimePack")
                .write_text_content(BytesText::new(pack))?;
            writer.create_element("refAnime")
                .write_text_content(BytesText::new(anim))?;
        }
        let ref_effect_name = self.effect_name.value(binary);
        if !ref_effect_name.is_empty() {
            writer.create_element("refEffectName")
                .write_text_content(BytesText::new(ref_effect_name))?;
        }
        writer.create_element("alphaBlendType")
            .write_text_content(BytesText::new(&format!("{:?}", self.alpha_blend_type)))?;
        writer.create_element("show")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("locked")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("expandAttribute")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("expandChildren")
            .write_text_content(BytesText::new("1"))?;
        let ref_color_name = self.color_label.value(binary);
        if !ref_color_name.is_empty() {
            writer.create_element("colorLabel")
                .write_text_content(BytesText::new(ref_color_name))?;
        }
        create_blank_element(writer, "refCellTag")?;
        // writer.create_element("refCellTag")
        //     .write_text_content(BytesText::new(self.ref_name.value(binary)))?;
        writer.create_element("maskInfluence")
            .write_text_content(BytesText::new(&format!("{}", self.mask_influence)))?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AnimEntry {
    name: StringPtr,
    default_data: Ptr<AnimInitial>,
    frame_data: Ptr<FrameData>,
    user_data: u32,
    label_data: Ptr<LabelEntry>,
    mesh_data_uv: Ptr<MeshUV>,
    mesh_data_indices: Ptr<MeshIndex>,
    start_frames: u16,
    end_frames: u16,
    total_frames: u16,
    fps: u16,
    label_num: u16,
    canvas_size_w: u16,
    canvas_size_h: u16,
    canvas_pivot_x: f32,
    canvas_pivot_y: f32
}

const _: () = {
    ["Size of AnimEntry"][size_of::<AnimEntry>() - 0x34];
};

impl AnimEntry {
    pub fn get_default_data(&self, binary: &[u8], num_parts: usize) -> &[AnimInitial] {
        self.default_data.array(binary, num_parts)
    }
    pub fn get_frame_data(&self, binary: &[u8], num_frames: usize) -> &[FrameData] {
        self.frame_data.array(binary, num_frames)
    }
    pub fn get_mesh_uv(&self, binary: &[u8], num_parts: usize) -> &[MeshUV] {
        self.mesh_data_uv.array(binary, num_parts)
    }
    pub fn get_mesh_index(&self, binary: &[u8], num_parts: usize) -> &[MeshIndex] {
        self.mesh_data_indices.array(binary, num_parts)
    }
    pub fn get_start_frames(&self) -> u16 {
        self.start_frames
    }
    pub fn get_total_frames(&self) -> u16 {
        self.total_frames
    }
    pub fn get_fps(&self) -> u16 {
        self.fps
    }
    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.name.value(binary)
    }

    pub fn to_xml<W: Write + Seek>(&self, writer: &mut Writer<W>, binary: &[u8],
        parts: &[PartEntry], cells: &HashMap<usize, (u16, &str)>) -> std::io::Result<()> {
        let anime_name = self.name.value(binary);
        writer.create_element("name")
            .write_text_content(BytesText::new(anime_name))?;
        writer.create_element("overrideSettings")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("settings")
            .write_inner_content(|writer| self.to_xml_settings(writer, binary))?;
        writer.create_element("labels").write_empty()?;
        writer.create_element("isSetup")
            .write_text_content(BytesText::new(&format!("{}", (anime_name == "Setup") as u8)))?;
        writer.create_element("partAnimes")
            .write_inner_content(|writer| {
                let mut attribute_writers: Vec<_> = (0..parts.len()).map(|i| {
                    AttributeWriter::new(parts[i].get_name(binary))
                }).collect();
                for (f, frame) in self.get_frame_data(binary, self.total_frames as usize).iter().enumerate() {
                    let mut data = frame.value(binary);
                    if anime_name == "Setup" {
                        for (i, cell_setup) in self.default_data.array(binary, parts.len()).iter().enumerate() {
                            if parts[i]._type == PartType::normal {
                                if !attribute_writers[i].has_attribute("CELL") {
                                    let index = cell_setup.cell_index;
                                    if let Some((map_id, cell_name)) = cells.get(&(index as usize)) {
                                        attribute_writers[i].add_attribute(f, AttributeKeyframe::Cell((*map_id, cell_name.to_string())));
                                    }
                                }
                                if !attribute_writers[i].has_attribute("HIDE") {
                                    attribute_writers[i].add_attribute(f, AttributeKeyframe::Hide(0));
                                }
                            }
                            if cell_setup.position.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::PositionX(cell_setup.position.x));
                            }
                            if cell_setup.position.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::PositionY(cell_setup.position.y));
                            }
                            if cell_setup.position.z != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::PositionZ(cell_setup.position.z));
                            }
                            if cell_setup.pivot.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::PivotX(cell_setup.pivot.x));
                            }
                            if cell_setup.pivot.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::PivotY(cell_setup.pivot.y));
                            }
                            if cell_setup.rotate.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::RotationX(cell_setup.rotate.x));
                            }
                            if cell_setup.rotate.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::RotationY(cell_setup.rotate.y));
                            }
                            if cell_setup.rotate.z != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::RotationZ(cell_setup.rotate.z));
                            }
                            if cell_setup.scale.x != 1. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::ScaleX(cell_setup.scale.x));
                            }
                            if cell_setup.scale.y != 1. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::ScaleY(cell_setup.scale.y));
                            }
                            if cell_setup.local_scale.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::LocalScaleX(cell_setup.local_scale.x));
                            }
                            if cell_setup.local_scale.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::LocalScaleY(cell_setup.local_scale.y));
                            }
                            if cell_setup.opacity != 255 {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::Opacity(cell_setup.opacity as f32 / 255.));
                            }
                            if cell_setup.local_opacity != 255 {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::Opacity(cell_setup.local_opacity as f32 / 255.));
                            }
                            if cell_setup.size.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::SizeX(cell_setup.size.x));
                            }
                            if cell_setup.size.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::SizeY(cell_setup.size.y));
                            }
                            if cell_setup.uv_move.x != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::UVMoveU(cell_setup.uv_move.x));
                            }
                            if cell_setup.uv_move.y != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::UVMoveV(cell_setup.uv_move.y));
                            }
                            if cell_setup.uv_rotate != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::UVRotate(cell_setup.uv_rotate));
                            }
                            if cell_setup.uv_scale.x != 1. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::UVScaleU(cell_setup.uv_scale.x));
                            }
                            if cell_setup.uv_scale.y != 1. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::UVScaleV(cell_setup.uv_scale.y));
                            }
                            if cell_setup.bounding_radius != 0. {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::BoundingRadius(cell_setup.bounding_radius));
                            }
                            if cell_setup.masklimen != 0 {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::Mask(cell_setup.masklimen));
                            }
                            if cell_setup.priority != 0 {
                                attribute_writers[i].add_attribute(f, AttributeKeyframe::Prio(cell_setup.priority));
                            }
                        }
                    } else {
                        for i in 0..parts.len() {
                            let current = unsafe { data.read::<FrameStart>(binary) };
                            let low_flag = current.get_low_flag(); // high flag is currently unused
                            if low_flag.contains(LowFlag::PART_FLAG_CELL_INDEX) {
                                let _cellIndex = unsafe { data.read::<u16>(&binary) };
                                println!("frame {}, part {}: cell index: {}", f, i, _cellIndex);
                                // mapId - index in cellmapNames
                                // name - name of cell
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_POSITION_X) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::PositionX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_POSITION_Y) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::PositionY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_POSITION_Z) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::PositionZ(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_PIVOT_X) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::PivotX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_PIVOT_Y) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::PivotY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_ROTATIONX) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::RotationX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_ROTATIONY) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::RotationY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_ROTATIONZ) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::RotationZ(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_SCALE_X) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::ScaleX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_SCALE_Y) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::ScaleY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_LOCALSCALE_X) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::LocalScaleX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_LOCALSCALE_Y) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::LocalScaleY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_OPACITY) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::Opacity(unsafe { data.read::<u16>(&binary) } as f32 / 255.));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_LOCALOPACITY) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::LocalOpacity(unsafe { data.read::<u16>(&binary) } as f32 / 255.));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_SIZE_X) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::SizeX(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_SIZE_Y) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::SizeY(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_U_MOVE) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::UVMoveU(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_V_MOVE) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::UVMoveV(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_UV_ROTATION) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::UVRotate(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_U_SCALE) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::UVScaleU(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_V_SCALE) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::UVScaleV(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_BOUNDINGRADIUS) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::BoundingRadius(unsafe { data.read::<f32>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_MASK) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::Mask(unsafe { data.read::<u16>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_PRIORITY) {
                                attribute_writers[i].add_attribute(f,
                                                                   AttributeKeyframe::Prio(unsafe { data.read::<u16>(&binary) }));
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_INSTANCE_KEYFRAME) {
                                let _keyframe = unsafe { data.read::<InstanceKeyframe>(&binary) };
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_EFFECT_KEYFRAME) {
                                let _effframe = unsafe { data.read::<EffectKeyframe>(&binary) };
                            }
                            if low_flag.contains(LowFlag::PART_FLAG_PARTS_COLOR) {
                                let type_and_flags = unsafe { data.read::<u16>(&binary) };
                                let flag = ColorAttributeFlags::from_bits_truncate(type_and_flags >> 8);
                                if flag.contains(ColorAttributeFlags::VERTEX_FLAG_ONE) {
                                    let color = unsafe { data.read::<ColorAttribute>(&binary) };
                                    // println!("{:?}", color);
                                } else {
                                    for i in 0..4 {
                                        let flag = ColorAttributeFlags::from_bits_truncate(1 << i);
                                        if flag.contains(flag) {
                                            let color = unsafe { data.read::<ColorAttribute>(&binary) };
                                            // println!("{:?}: {:?}", flag, color);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                for attrib_writer in attribute_writers {
                    if !attrib_writer.has_attributes() {
                        continue;
                    }
                    let name = attrib_writer.name;
                    writer.create_element("partAnime")
                        .write_inner_content(|writer| {
                            writer.create_element("partName")
                                .write_text_content(BytesText::new(name))?;
                            writer.create_element("attributes")
                                .write_inner_content(|writer| attrib_writer.write_attributes(writer, anime_name == "Setup"))?;
                            Ok(())
                        })?;
                }
                Ok(())
            })?;
        Ok(())
    }

    pub(crate) fn to_xml_settings<W: Write + Seek>(&self, writer: &mut Writer<W>, binary: &[u8])
                                                   -> std::io::Result<()> {
        writer.create_element("fps")
            .write_text_content(BytesText::new(&format!("{}", self.fps)))?;
        writer.create_element("frameCount")
            .write_text_content(BytesText::new(&format!("{}", self.total_frames)))?;
        writer.create_element("sortMode")
            .write_text_content(BytesText::new("prio"))?;
        writer.create_element("canvasSize")
            .write_text_content(BytesText::new(&format!("{} {}", self.canvas_size_w, self.canvas_size_h)))?;
        writer.create_element("pivot")
            .write_text_content(BytesText::new(&format!("{} {}", self.canvas_pivot_x, self.canvas_pivot_y)))?;
        writer.create_element("gridSize")
            .write_text_content(BytesText::new("32"))?;
        writer.create_element("gridColor")
            .write_text_content(BytesText::new("FF808080"))?;
        writer.create_element("ik_depth")
            .write_text_content(BytesText::new("3"))?;
        writer.create_element("startFrame")
            .write_text_content(BytesText::new(&format!("{}", self.start_frames)))?;
        writer.create_element("endFrame")
            .write_text_content(BytesText::new(&format!("{}", self.end_frames)))?;
        writer.create_element("outStartNum")
            .write_text_content(BytesText::new("0"))?;
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct LowFlag: u32 {
        const PART_FLAG_INVISIBLE			= 1 << 0;
	    const PART_FLAG_FLIP_H			= 1 << 1;
	    const PART_FLAG_FLIP_V			= 1 << 2;

	    // optional parameter flags
	    const PART_FLAG_CELL_INDEX		= 1 << 3;
	    const PART_FLAG_POSITION_X		= 1 << 4;
	    const PART_FLAG_POSITION_Y		= 1 << 5;
	    const PART_FLAG_POSITION_Z		= 1 << 6;
	    const PART_FLAG_PIVOT_X			= 1 << 7;
	    const PART_FLAG_PIVOT_Y           = 1 << 8;
	    const PART_FLAG_ROTATIONX			= 1 << 9;
	    const PART_FLAG_ROTATIONY			= 1 << 10;
	    const PART_FLAG_ROTATIONZ			= 1 << 11;
	    const PART_FLAG_SCALE_X			= 1 << 12;
	    const PART_FLAG_SCALE_Y			= 1 << 13;
	    const PART_FLAG_LOCALSCALE_X		= 1 << 14;
	    const PART_FLAG_LOCALSCALE_Y		= 1 << 15;
	    const PART_FLAG_OPACITY			= 1 << 16;
	    const PART_FLAG_LOCALOPACITY		= 1 << 17;
	    const PART_FLAG_PARTS_COLOR		= 1 << 18;
	    const PART_FLAG_VERTEX_TRANSFORM	= 1 << 19;

	    const PART_FLAG_SIZE_X			= 1 << 20;
	    const PART_FLAG_SIZE_Y			= 1 << 21;

	    const PART_FLAG_U_MOVE			= 1 << 22;
	    const PART_FLAG_V_MOVE			= 1 << 23;
	    const PART_FLAG_UV_ROTATION		= 1 << 24;
	    const PART_FLAG_U_SCALE			= 1 << 25;
	    const PART_FLAG_V_SCALE			= 1 << 26;
	    const PART_FLAG_BOUNDINGRADIUS	= 1 << 27;

	    const PART_FLAG_MASK				= 1 << 28;
	    const PART_FLAG_PRIORITY			= 1 << 29;

	    const PART_FLAG_INSTANCE_KEYFRAME	= 1 << 30;
	    const PART_FLAG_EFFECT_KEYFRAME   = 1 << 31;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct HighFlag : u32 {
        const PART_FLAG_MESHDATA = 1 << 0;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AnimInitial {
    index: u16,
    reserve0: u16,
    lowflag: LowFlag,
    highflag: HighFlag,
    priority: u16,
    cell_index: u16,
    opacity: u16,
    local_opacity: u16,
    masklimen: u16,
    reserve1: u16,
    position: Vec3,
    pivot: Vec2,
    rotate: Vec3,
    scale: Vec2,
    local_scale: Vec2,
    size: Vec2,
    uv_move: Vec2,
    uv_rotate: f32,
    uv_scale: Vec2,
    bounding_radius: f32,
    instance_current_frame: u32,
    instance_start_frame: u32,
    instance_end_frame: u32,
    instance_loop_num: u32,
    instance_speed: f32,
    instance_loop_flag: u32,
    effect_current_frame: u32,
    effect_start_time: u32,
    effect_speed: f32,
    effect_loop_flag: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct FramePart(usize);

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct FrameStart {
    index: u16,
    low_flag: LowFlag,
    high_flag: HighFlag
}

impl FrameStart {
    pub fn get_index(&self) -> u16 {
        self.index
    }
    pub fn get_low_flag(&self) -> LowFlag {
        self.low_flag
    }
    pub fn get_high_flag(&self) -> HighFlag {
        self.high_flag
    }
}

impl FramePart {
    pub unsafe fn read<T>(&mut self, binary: &[u8]) -> T {
        let value = unsafe { std::ptr::read(binary.as_ptr().add(self.0) as *const T) };
        // let value = unsafe { &*(binary.as_ptr().add(self.0) as *const T) };
        self.0 += size_of::<T>();
        value
    }
    pub unsafe fn advance(&mut self, bytes: usize) {
        self.0 += bytes;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct FrameData(Ptr<()>);

impl FrameData {
   pub fn value(&self, binary: &[u8]) -> FramePart {
       let offset = ((&raw const *self.0.value(binary)) as usize) - (binary.as_ptr() as usize);
       FramePart(offset)
   }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct ColorAttributeFlags : u16 {
	    const VERTEX_FLAG_LT		= 1 << 0;
	    const VERTEX_FLAG_RT		= 1 << 1;
	    const VERTEX_FLAG_LB		= 1 << 2;
	    const VERTEX_FLAG_RB		= 1 << 3;
	    const VERTEX_FLAG_ONE		= 1 << 4; // color blend only
    }
}

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct ColorAttribute {
    rate: f32,
    rgba: u32
}

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct InstanceKeyframe {
    current_frame: u32,
    start_frame: u32,
    end_frame: u32,
    loop_num: u32,
    speed: f32,
    loop_flag: u32,
}

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct EffectKeyframe {
    current_frame: u32,
    start_time: u32,
    speed: f32,
    loop_flag: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshUV(Ptr<MeshUVData>);

#[repr(C)]
#[derive(Debug)]
pub struct MeshUVData {

}

#[repr(C)]
#[derive(Debug)]
pub struct MeshIndex;

#[repr(C)]
#[derive(Debug)]
pub struct LabelEntry {
    name: StringPtr,
    time: u16
}

#[repr(C)]
#[derive(Debug)]
pub struct Anime {
    name: StringPtr,
    parts: Ptr<PartEntry>,
    anims: Ptr<AnimEntry>,
    part_count: u16,
    anim_count: u16
}

const _: () = {
    ["Size of Anime"][size_of::<Anime>() - 0x10];
};

impl Anime {
    pub fn get_parts(&self, binary: &[u8]) -> &[PartEntry] {
        self.parts.array(binary, self.part_count as usize)
    }

    pub fn get_num_parts(&self) -> u16 {
        self.part_count
    }

    pub fn get_anims(&self, binary: &[u8]) -> &[AnimEntry] {
        self.anims.array(binary, self.anim_count as usize)
    }

    pub fn get_num_anims(&self) -> u16 {
        self.anim_count
    }

    pub fn get_name(&self, binary: &[u8]) -> &str {
        self.name.value(binary)
    }

    pub fn to_xml(&self, cell_names: &[String], binary: &[u8], cells: &HashMap<usize, (u16, &str)>) -> Result<Vec<u8>, Box<dyn Error>> {
        let xml_fmt = "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>\n";
        let mut cursor = Cursor::new(xml_fmt.as_bytes().to_vec());
        cursor.seek(SeekFrom::End(0))?;
        let mut writer = Writer::new_with_indent(cursor, '\t' as u8, 1);
        writer.create_element("SpriteStudioAnimePack")
            .with_attributes([("version", "2.00.00")])
            .write_inner_content(|writer| self.to_xml_body(writer, cell_names, binary, cells))?;
        Ok(writer.into_inner().into_inner())
    }

    pub(crate) fn to_xml_body<W: Write + Seek>(&self,
        writer: &mut Writer<W>, cell_names: &[String], binary: &[u8], cells: &HashMap<usize, (u16, &str)>) -> std::io::Result<()> {
        writer.create_element("name")
            .write_text_content(BytesText::new(self.name.value(binary)))?;
            create_blank_element(writer, "exportPath")?;
        writer.create_element("Model")
            .write_inner_content(|writer|  self.to_xml_model(writer, binary))?;
        create_name_list("cellmapNames", cell_names, writer)?;
        writer.create_element("animeList")
            .write_inner_content(|writer| {
                for anime in self.get_anims(binary) {
                    writer.create_element("anime")
                        .write_inner_content(|writer| anime.to_xml(
                            writer, binary, self.get_parts(binary), cells))?;
                }
                Ok(())
            })?;
        Ok(())
    }

    fn to_xml_model<W: Write + Seek>(&self, writer: &mut Writer<W>, binary: &[u8]) -> std::io::Result<()> {
        writer.create_element("partList")
            .write_inner_content(|writer| {
                for part in self.get_parts(binary) {
                    writer.create_element("value")
                        .write_inner_content(|writer| part.to_xml(writer, binary))?;
                }
                Ok(())
            })?;
        Ok(())
    }
}

pub struct AttributeWriter<'a> {
    name: &'a str,
    attributes: HashMap<&'static str, Vec<(usize, AttributeKeyframe)>>,
}

impl<'a> AttributeWriter<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
        }
    }

    pub fn has_attributes(&self) -> bool {
        !self.attributes.is_empty()
    }

    pub fn has_attribute(&self, attrib: &str) -> bool {
        self.attributes.contains_key(&attrib)
    }

    pub fn add_attribute(&mut self, frame: usize, value: AttributeKeyframe) {
        if !self.attributes.contains_key(value.get_tag_name()) {
            self.attributes.insert(value.get_tag_name(), vec![]);
        }
        self.attributes.get_mut(value.get_tag_name()).as_mut().unwrap().push((frame, value));
    }

    pub fn write_attributes<W: Write + Seek>(&self, writer: &mut Writer<W>, ignore_interpolation: bool) -> std::io::Result<()> {
        for (tag, keyframes) in &self.attributes {
            writer.create_element("attribute")
                .with_attribute(("tag", *tag))
                .write_inner_content(|writer| {
                    for (frame, value) in keyframes {
                        value.to_xml(*frame, writer, ignore_interpolation)?;
                    }
                    Ok(())
                })?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum AttributeKeyframe {
    Cell((u16, String)),
    PositionX(f32),
    PositionY(f32),
    PositionZ(f32),
    PivotX(f32),
    PivotY(f32),
    RotationX(f32),
    RotationY(f32),
    RotationZ(f32),
    ScaleX(f32),
    ScaleY(f32),
    LocalScaleX(f32),
    LocalScaleY(f32),
    Opacity(f32),
    LocalOpacity(f32),
    SizeX(f32),
    SizeY(f32),
    UVMoveU(f32),
    UVMoveV(f32),
    UVRotate(f32),
    UVScaleU(f32),
    UVScaleV(f32),
    BoundingRadius(f32),
    Mask(u16),
    Prio(u16),
    FlipH(u16),
    FlipV(u16),
    Hide(u16)
}

impl AttributeKeyframe {
    fn get_tag_name(&self) -> &'static str {
        match self {
            Self::Cell(_) => "CELL",
            Self::PositionX(_) => "POSX",
            Self::PositionY(_) => "POSY",
            Self::PositionZ(_) => "POSZ",
            Self::PivotX(_) => "PVTX",
            Self::PivotY(_) => "PVTY",
            Self::RotationX(_) => "ROTX",
            Self::RotationY(_) => "ROTY",
            Self::RotationZ(_) => "ROTZ",
            Self::ScaleX(_) => "SCLX",
            Self::ScaleY(_) => "SCLY",
            Self::LocalScaleX(_) => "LSCX",
            Self::LocalScaleY(_) => "LSCY",
            Self::Opacity(_) => "ALPH",
            Self::LocalOpacity(_) => "LALP",
            Self::SizeX(_) => "SIZX",
            Self::SizeY(_) => "SIZY",
            Self::UVMoveU(_) => "UVTX",
            Self::UVMoveV(_) => "UVTY",
            Self::UVRotate(_) => "UVRZ",
            Self::UVScaleU(_) => "UVSX",
            Self::UVScaleV(_) => "UVSY",
            Self::BoundingRadius(_) => "BNDR",
            Self::Mask(_) => "MASK",
            Self::Prio(_) => "PRIO",
            Self::FlipH(_) => "FLPH",
            Self::FlipV(_) => "FLPV",
            Self::Hide(_) => "HIDE",
        }
    }

    fn use_interpolation(&self) -> bool {
        match self {
            Self::Cell(_) | Self::FlipH(_) | Self::FlipV(_) | Self::Hide(_) => false,
            _ => true
        }
    }

    fn into_xml_attributes<'a>(&'a self, frame_str: &'a str, ignore_interpolation: bool) -> Vec<Attribute> {
        let mut attributes = vec![];
        attributes.push(Attribute::from(("time", frame_str)));
        if self.use_interpolation() && !ignore_interpolation {
            attributes.push(Attribute::from(("ipType", "linear")));
        }
        attributes
    }

    fn to_xml<W: Write + Seek>(&self, frame: usize, writer: &mut Writer<W>, ignore_interpolation: bool)
        -> std::io::Result<()> {
        let frame_as_str = format!("{}", frame);
        writer.create_element("key")
            .with_attributes(self.into_xml_attributes(&frame_as_str, ignore_interpolation))
            .write_inner_content(|writer| {
                let value = writer.create_element("value");
                match self {
                    Self::Cell((map_id, name)) => {
                        value.write_inner_content(|writer| {
                            writer.create_element("mapId")
                                .write_text_content(BytesText::new(&format!("{}", map_id)))?;
                            writer.create_element("name")
                                .write_text_content(BytesText::new(&name))?;
                            Ok(())
                        })
                    },
                    Self::PositionX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::PositionY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::PositionZ(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::PivotX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::PivotY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::RotationX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::RotationY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::RotationZ(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::ScaleX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::ScaleY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::LocalScaleX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::LocalScaleY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::Opacity(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::LocalOpacity(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::SizeX(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::SizeY(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::UVMoveU(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::UVMoveV(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::UVRotate(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::UVScaleU(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::UVScaleV(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::BoundingRadius(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::Mask(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::Prio(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::FlipH(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::FlipV(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                    Self::Hide(v) => value.write_text_content(BytesText::new(&format!("{}", *v))),
                }?;
                Ok(())
            })?;
        Ok(())
    }
}
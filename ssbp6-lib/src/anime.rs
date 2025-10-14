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

use bitflags::bitflags;
use glam::{Vec2, Vec3};
use crate::util::{Ptr, StringPtr};

#[repr(C)]
#[derive(Debug)]
pub struct PartEntry {
    name: StringPtr,
    index: u16,
    parent_index: u16,
    _type: u16,
    bounds_type: u16,
    alpha_blend_type: u16,
    ref_name: StringPtr,
    effect_name: StringPtr,
    color_label: StringPtr,
    mask_influence: u16,
}

const _: () = {
    ["Size of PartEntry"][size_of::<PartEntry>() - 0x20];
};

impl PartEntry {
    pub fn get_index(&self) -> u16 {
        self.index
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
    pub fn get_frame_data(&self, binary: &[u8], num_parts: usize) -> &[FrameData] {
        self.frame_data.array(binary, num_parts)
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
pub struct FramePart;

#[repr(C)]
#[derive(Debug)]
pub struct FrameData(Ptr<FramePart>);

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
pub struct Anime {
    name: StringPtr,
    parts: Ptr<PartEntry>,
    anims: Ptr<AnimEntry>,
    part_count: u16,
    anim_count: u16
}

#[repr(C)]
#[derive(Debug)]
pub struct LabelEntry {
    name: StringPtr,
    time: u16
}

const _: () = {
    ["Size of Anime"][size_of::<Anime>() - 0x10];
};

impl Anime {
    pub fn get_parts(&self, binary: &[u8]) -> &[PartEntry] {
        self.parts.array(binary, self.part_count as usize)
    }
    pub fn get_anims(&self, binary: &[u8]) -> &[AnimEntry] {
        self.anims.array(binary, self.anim_count as usize)
    }
}
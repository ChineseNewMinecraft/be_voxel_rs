use nalgebra::*;

enum VoxelTexturePosition{
    U,
    D,
    L,
    R,
    F,
    B
}

enum VoxelTextureType{
    COLOR,
    METALNESS,
    ROUGHNESS,

    NORMAL,
    THICKNESS,
    AO,
}

// should add to one!
struct VoxelTextureBlendingOption{
    color:f32,
    metalness:f32,
    roughness:f32,
}

struct VoxelTextureBlendingGroup{
    blending_group_id: u8,
    // texture id
    textures: [u8;3],
    normal: u8,
    thickness: u8,
    ao: u8,
    // inline
    blending_option: VoxelTextureBlendingOption
}

enum ConnectionMode{
    H, // Horizontal
    HC, // Horizontan Only change current voxel
    V,
    VC,
}

struct VoxelTextureGroup{
    group_id: u8,
    // blending_group_id
    data: Vec<u8>,
}

struct VoxelTexture{
    id: u8,
    type: VoxelTextureType,
    pos: VoxelTexturePosition,
    // true -> look up file(PNG) , false -> read binary in PNG form
    is_filename: bool,
    data: String,
    random_replacements: Option<Vec<String>>
}

struct ConnectionConfig{
    mode: Connection,
    // voxel type id
    with: u64,
    // texture group id
    replace_by_texture_group: u8,
}


struct Voxel{
    name: String,
    type_id: u64,
    enable_alpha_blend: bool,
    enable_inner_wire_frame: bool,
    connection_config: Option<Vec<ConnectionConfig>>,
    default_texture_group: u8,
}
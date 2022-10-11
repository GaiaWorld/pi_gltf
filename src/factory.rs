pub struct World; // 先占位

pub struct EntityID;

pub struct UUID;

pub trait TFactory<EID, GID, MID, TID, AID, ATID, AGID> {
    /// 创建 节点
    fn create_node(
        world: &mut World,
        position: Option<&[f32]>,
        scaling: Option<&[f32]>,
        rotation: Option<&[f32]>,
        rotation_quaterion: Option<&[f32]>,
    ) -> EID;

    /// 赋予节点 层级信息
    fn layer_mask(
        world: &mut World,
        entity: EID,
        layer: u32,
    );

    /// 赋予节点 包围盒信息
    fn bounding_info(
        world: &mut World,
        entity: EID,
        center: &[f32],
        extend: &[f32],
    );

    /// 查询是否已有 目标网格信息
    fn query_geometry(
        id: UUID,
    ) -> Option<GID>;

    /// 创建 网格
    fn create_geometry_base(
        world: &mut World,
        id: UUID,
        positions: &[f32],
        indices: &[u16],
        normals: Option<&[f32]>,
        tangents: Option<&[f32]>,
        colors: Option<&[f32]>,
        uvs: Option<&[f32]>,
        uv2s: Option<&[f32]>,
    ) -> GID;

    /// 查询是否已有 目标材质
    fn query_material(
        id: UUID,
    ) -> Option<MID>;

    /// 创建材质
    fn create_material_base(
        world: &mut World,
        id: UUID,
        alpha: Option<f32>,
        alpha_index: Option<u32>,
        cull_face: Option<u8>,
        z_write: Option<bool>,
    ) -> MID;
    
    /// 查询是否已有 纹理
    fn query_texture(
        id: UUID,
    ) -> Option<TID>;

    /// 创建 纹理
    fn create_texture(
        world: &mut World,
        id: UUID,
        texture: Texture,
    ) -> TID;

    /// 创建 纹理 view
    fn texture_view(
        world: &mut World,
        id: TID,
        has_alpha: bool,
        mag_filter: u8,
        min_filter: u8,
        wrap_u: u8,
        wrap_v: u8,
        format: u8,
    );

    /// 绑定 geometry - 可以多次
    fn mesh_geometry(
        world: &mut World,
        entity: EID,
        geometry: GID,
    );

    /// 绑定 材质 - 可以多次
    fn mesh_material(
        world: &mut World,
        entity: EID,
        material: MID,
    );

    /// 查询是否已有目标 动画数据
    fn query_animation(
        id: UUID,
    ) -> Option<AID>;

    /// 创建 动画数据
    fn animation(
        id: UUID,
        keys: KeyFrames,
        ty: u32,
    ) -> AID;

    /// 创建 Target 动画
    fn target_animation(
        target: EID,
        attr: u32,
        ty: u32,
        animation: &[AID],
    ) -> ATID;

    /// 创建 动画组
    fn animation_group(
        target_animations: &[ATID],
    ) -> AGID;
}
use std::alloc::alloc_zeroed;
use std::alloc::Layout;
use std::alloc::LayoutError;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::raw::c_char;
use std::path::Path;

use crate::address::*;
use crate::constant_flag::Live2DConstantFlag;
use crate::drawable::*;
use crate::dynamic_flag::Live2DDynamicFlag;
use crate::parameter::*;
use crate::part::*;

use crate::vector2::Live2DVector2;

#[derive(Debug, Clone)]
pub struct Live2DVector4(live2d_mini_sys::csmVector4);
impl Live2DVector4 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0.X
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.0.Y
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.0.Z
    }

    #[inline]
    pub fn w(&self) -> f32 {
        self.0.W
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.x()
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.y()
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.z()
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.w()
    }
}

#[derive(Debug, Clone)]
pub struct Live2DReadCanvasInfo {
    /// キャンバスサイズ
    pub out_size_in_pixels: Live2DVector2,
    /// キャンバスの中心点
    pub out_origin_in_pixels: Live2DVector2,
    /// モデルのユニットの大きさ
    pub out_pixels_per_unit: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Live2DModelResource {
    _model_address: Live2DAddress,
    _moc_address: Live2DAddress,

    model: *mut live2d_mini_sys::csmModel,
    not_exists_parameter_ids: HashMap<String, usize>,
}

impl Live2DModelResource {
    /// moc3ファイルを読み込んでLive2DModelを生成する
    pub fn new<T>(moc_path: T) -> Self
    where
        T: AsRef<Path>,
    {
        Self::read_blob_aligned(moc_path.as_ref()).expect("path error")
    }

    pub fn update(&self) {
        self.csm_update_model();
        self.csm_reset_drawable_dynamic_flags();
    }

    pub fn iter_drawables<'a>(&'a self) -> Live2DDrawableIter<'a> {
        Live2DDrawableIter {
            pos: 0,
            len: self.csm_get_drawable_count(),

            inner: self,
        }
    }

    pub fn iter_sorted_drawables<'a>(&'a self) -> Live2DSortedDrawableIter<'a> {
        // なんとかする
        let mut work_vec = self
            .csm_get_drawable_render_orders()
            .iter()
            .enumerate()
            .map(|(index, order)| (index, order))
            .collect::<Vec<(usize, &i32)>>();

        work_vec.sort_by(|a, b| b.1.cmp(a.1));

        let sorted_indices = work_vec
            .into_iter()
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        Live2DSortedDrawableIter {
            sorted_indices,

            inner: self,
        }
    }

    pub fn iter_parameters<'a>(&'a self) -> Live2DParameterIter<'a> {
        Live2DParameterIter {
            pos: 0,
            len: self.csm_get_parameter_count(),

            inner: self,
        }
    }

    pub fn iter_mut_parameters<'a>(&'a self) -> Live2DParameterIterMut<'a> {
        Live2DParameterIterMut {
            pos: 0,
            len: self.csm_get_parameter_count(),

            inner: self,
        }
    }

    pub fn iter_parts<'a>(&'a self) -> Live2DPartIter<'a> {
        Live2DPartIter {
            pos: 0,
            len: self.csm_get_part_count(),

            inner: self,
        }
    }

    pub fn iter_mut_parts<'a>(&'a self) -> Live2DPartIterMut<'a> {
        Live2DPartIterMut {
            pos: 0,
            len: self.csm_get_part_count(),

            inner: self,
        }
    }

    unsafe fn allocate_aligned(size: usize, align: usize) -> Result<Live2DAddress, LayoutError> {
        let layout = Layout::from_size_align(size, align)?;
        Ok(Live2DAddress {
            ptr: alloc_zeroed(layout),
            layout,
        })
    }

    fn read_blob_aligned(path: &Path) -> io::Result<Self> {
        let mut file = File::open(Path::new(path))?;
        let file_size = file.metadata()?.len() as usize;

        unsafe {
            // このアドレスを破棄の対象にする
            let moc_address =
                Self::allocate_aligned(file_size, live2d_mini_sys::csmAlignofMoc as _)
                    .expect("allocate error");
            let moc_slice =
                std::slice::from_raw_parts_mut(moc_address.ptr, moc_address.layout.size());
            file.read(moc_slice)?;
            let moc = live2d_mini_sys::csmReviveMocInPlace(moc_address.ptr as _, file_size as _);

            let model_size = live2d_mini_sys::csmGetSizeofModel(moc);
            // このアドレスを破棄の対象にする
            let model_address =
                Self::allocate_aligned(model_size as _, live2d_mini_sys::csmAlignofModel as _)
                    .expect("allocate error");
            let model =
                live2d_mini_sys::csmInitializeModelInPlace(moc, model_address.ptr as _, model_size);

            Ok(Self {
                _model_address: model_address,
                _moc_address: moc_address,

                model,
                not_exists_parameter_ids: HashMap::new(),
            })
        }
    }

    // parameter idからindexを取得する
    // idがないものが渡される可能性があるのでそれを考慮する
    pub fn get_parameter_index(&mut self, id: &str) -> usize {
        // dbg!(self.csm_get_parameter_count());
        // jsonデータにあるパラメータか
        if let Some(index) = self.iter_parameters().position(|param| param.id() == id) {
            index
        } else {
            // 存在していないパラメータリストに存在しているか
            if let Some(index) = self.not_exists_parameter_ids.get(id) {
                *index
            } else {
                let index = self.csm_get_drawable_count() + self.not_exists_parameter_ids.len();
                self.not_exists_parameter_ids.insert(id.to_string(), index);

                index
            }
        }
    }

    pub fn csm_read_canvas_info(&self) -> Live2DReadCanvasInfo {
        unsafe {
            let mut out_size_in_pixels: Live2DVector2 = std::mem::zeroed();
            let mut out_origin_in_pixels: Live2DVector2 = std::mem::zeroed();
            let mut out_pixels_per_unit: f32 = 0.0;
            live2d_mini_sys::csmReadCanvasInfo(
                self.model,
                &mut out_size_in_pixels.0,
                &mut out_origin_in_pixels.0,
                &mut out_pixels_per_unit,
            );

            Live2DReadCanvasInfo {
                out_size_in_pixels,
                out_origin_in_pixels,
                out_pixels_per_unit,
            }
        }
    }

    /// 更新を適用する
    #[inline]
    pub fn csm_update_model(&self) {
        unsafe { live2d_mini_sys::csmUpdateModel(self.model) }
    }

    #[inline]
    pub fn csm_get_parameter_count(&self) -> usize {
        unsafe {
            let size = live2d_mini_sys::csmGetParameterCount(self.model);
            assert_ne!(size, -1);

            size as usize
        }
    }

    #[inline]
    pub fn csm_get_parameter_ids(&self) -> &[*const c_char] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterIds(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_types(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterTypes(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_minimum_values(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterMinimumValues(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_maximum_values(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterMaximumValues(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_default_values<'a>(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterDefaultValues(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    // ここに書き込むとmodelを操作できる
    #[inline]
    pub fn csm_get_mut_parameter_values<'a>(&self) -> &mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(
                live2d_mini_sys::csmGetParameterValues(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_values<'a>(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterValues(self.model),
                self.csm_get_parameter_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_key_counts<'a>(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterKeyCounts(self.model),
                self.csm_get_part_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_parameter_key_values<'a>(&self) -> &[*const f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetParameterKeyValues(self.model),
                self.csm_get_part_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_part_count(&self) -> usize {
        unsafe {
            let size = live2d_mini_sys::csmGetPartCount(self.model);
            assert_ne!(size, -1);

            size as usize
        }
    }

    #[inline]
    pub fn csm_get_part_ids(&self) -> &[*const c_char] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetPartIds(self.model),
                self.csm_get_part_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_part_opacities(&self) -> &mut [f32] {
        unsafe {
            std::slice::from_raw_parts_mut(
                live2d_mini_sys::csmGetPartOpacities(self.model),
                self.csm_get_part_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_part_parent_part_indices(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetPartParentPartIndices(self.model),
                self.csm_get_part_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_count(&self) -> usize {
        unsafe {
            let size = live2d_mini_sys::csmGetDrawableCount(self.model);
            assert_ne!(size, -1);

            size as usize
        }
    }

    #[inline]
    pub fn csm_get_drawable_ids<'a>(&self) -> &[*const c_char] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableIds(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_constant_flags(&self) -> &[Live2DConstantFlag] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableConstantFlags(self.model)
                    as *const Live2DConstantFlag,
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_dynamic_flags(&self) -> &[Live2DDynamicFlag] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableDynamicFlags(self.model) as *const Live2DDynamicFlag,
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_texture_indices(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableTextureIndices(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_draw_orders(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableDrawOrders(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_render_orders(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableRenderOrders(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_opacities(&self) -> &[f32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableOpacities(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_mask_counts(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableMaskCounts(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    //配列の配列をなんとかする
    #[inline]
    pub fn csm_get_drawable_masks(&self) -> &[*const i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableMasks(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_vertex_counts(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableVertexCounts(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    //配列の配列をなんとかする
    #[inline]
    pub fn csm_get_drawable_vertex_positions(&self) -> &[*const Live2DVector2] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableVertexPositions(self.model) as _,
                self.csm_get_drawable_count(),
            )
        }
    }

    //配列の配列をなんとかする
    #[inline]
    pub fn csm_get_drawable_vertex_uvs(&self) -> &[*const Live2DVector2] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableVertexUvs(self.model) as _,
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_index_counts(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableIndexCounts(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    //配列の配列をなんとかする
    #[inline]
    pub fn csm_get_drawable_indices(&self) -> &[*const u16] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableIndices(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_reset_drawable_dynamic_flags(&self) {
        unsafe { live2d_mini_sys::csmResetDrawableDynamicFlags(self.model) };
    }

    #[inline]
    pub fn csm_get_drawable_multiply_colors(&self) -> &[*const Live2DVector4] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableMultiplyColors(self.model) as _,
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_screen_colors(&self) -> &[Live2DVector4] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableScreenColors(self.model) as _,
                self.csm_get_drawable_count(),
            )
        }
    }

    #[inline]
    pub fn csm_get_drawable_parent_part_indices(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(
                live2d_mini_sys::csmGetDrawableParentPartIndices(self.model),
                self.csm_get_drawable_count(),
            )
        }
    }
}

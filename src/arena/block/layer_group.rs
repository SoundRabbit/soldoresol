uses! {}

use super::super::ImageData;
use super::util::Pack;
use super::BlockMut;
use super::BlockRef;
use super::CanvasTexture;

block! {
    [pub Drawing(constructor, pack)]
    (local_drawing_texture): BlockMut<CanvasTexture>;
    (drawed_texture): BlockMut<CanvasTexture>;
}

impl Drawing {
    pub fn local_drawing_texture(&self) -> BlockRef<CanvasTexture> {
        self.local_drawing_texture.as_ref()
    }

    pub fn local_drawing_texture_mut(&self) -> BlockMut<CanvasTexture> {
        self.local_drawing_texture.clone()
    }

    pub fn drawed_texture(&self) -> BlockRef<CanvasTexture> {
        self.drawed_texture.as_ref()
    }

    pub fn drawed_texture_mut(&self) -> BlockMut<CanvasTexture> {
        self.drawed_texture.clone()
    }
}

block! {
    [pub Fill(constructor, pack)]
    color: crate::libs::color::Pallet = crate::libs::color::Pallet::gray(0);
}

block! {
    [pub Image(constructor, pack)]
    data: BlockRef<ImageData> = BlockRef::<ImageData>::none();
}

impl Image {
    pub fn data(&self) -> BlockRef<ImageData> {
        self.data.as_ref()
    }

    pub fn set_data(&mut self, data: BlockRef<ImageData>) {
        self.data = data;
    }
}

pub enum Layer {
    Drawing(Drawing),
    Fill(Fill),
    Image(Image),
    LayerGroup(BlockMut<LayerGroup>),
}

#[async_trait(?Send)]
impl Pack for Layer {
    async fn pack(&self, is_deep: bool) -> JsValue {
        match self {
            Self::Drawing(data) => (object! {"Drawing": data.pack(is_deep).await}).into(),
            Self::Fill(data) => (object! {"Fill": data.pack(is_deep).await}).into(),
            Self::Image(data) => (object! {"Image": data.pack(is_deep).await}).into(),
            Self::LayerGroup(data) => (object! {"LayerGroup": data.pack(is_deep).await}).into(),
        }
    }
}

block! {
    [pub LayerGroup(constructor, pack)]
    layers: Vec<Layer> = vec![];
    local_selecting_layer_idx: usize = 0;
}

impl LayerGroup {}

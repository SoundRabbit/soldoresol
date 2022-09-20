#[allow(unused_imports)]
use super::util::prelude::*;

use super::super::ImageData;
use super::util::{Pack, PackDepth};
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
    async fn pack(&self, pack_depth: PackDepth) -> JsValue {
        match self {
            Self::Drawing(data) => (object! {
                "type": "Drawing",
                "data": data.pack(pack_depth).await
            })
            .into(),
            Self::Fill(data) => (object! {
                "type": "Fill",
                "data": data.pack(pack_depth).await
            })
            .into(),
            Self::Image(data) => (object! {
                "type": "Image",
                "data": data.pack(pack_depth).await
            })
            .into(),
            Self::LayerGroup(data) => (object! {
                "type": "LayerGroup",
                "data": data.pack(pack_depth).await
            })
            .into(),
        }
    }

    async fn unpack(data: &JsValue, arena: ArenaMut) -> Option<Box<Self>> {
        let data = data.dyn_ref::<crate::libs::js_object::Object>()?;
        let data_type = data.get("type")?.as_string()?;
        let data = data.get("data")?;

        match data_type.as_str() {
            "Drawing" => Drawing::unpack(&data, ArenaMut::clone(&arena))
                .await
                .map(|data| Box::new(Self::Drawing(*data))),
            "Fill" => Fill::unpack(&data, ArenaMut::clone(&arena))
                .await
                .map(|data| Box::new(Self::Fill(*data))),
            "Image" => Image::unpack(&data, ArenaMut::clone(&arena))
                .await
                .map(|data| Box::new(Self::Image(*data))),
            "LayerGroup" => BlockMut::<LayerGroup>::unpack(&data, ArenaMut::clone(&arena))
                .await
                .map(|data| Box::new(Self::LayerGroup(*data))),
            _ => None,
        }
    }
}

block! {
    [pub LayerGroup(constructor, pack)]
    layers: Vec<Layer> = vec![];
    local_selecting_layer_idx: usize = 0;
}

impl LayerGroup {}

use super::BlockId;
use crate::arena::resource::ResourceId;
use crate::libs::select_list::SelectList;

#[derive(Clone)]
pub struct Drawing {
    drawing_texture_id: BlockId,
    drawed_texture_id: BlockId,
}

impl Drawing {
    pub fn new(drawing_texture_id: BlockId, drawed_texture_id: BlockId) -> Self {
        Self {
            drawing_texture_id,
            drawed_texture_id,
        }
    }

    pub fn drawing_texture_id(&self) -> &BlockId {
        &self.drawed_texture_id
    }

    pub fn set_drawing_texture_id(&mut self, block_id: BlockId) {
        self.drawing_texture_id = block_id;
    }

    pub fn drawed_texture_id(&self) -> &BlockId {
        &self.drawed_texture_id
    }

    pub fn set_drawed_texture_id(&mut self, block_id: BlockId) {
        self.drawed_texture_id = block_id;
    }
}

#[derive(Clone)]
pub struct Image {
    resource_id: Option<ResourceId>,
}

impl Image {
    pub fn new() -> Self {
        Self { resource_id: None }
    }

    pub fn resource_id(&self) -> Option<&ResourceId> {
        self.resource_id.as_ref()
    }

    pub fn set_resource_id(&mut self, resource_id: Option<ResourceId>) {
        self.resource_id = resource_id;
    }
}

#[derive(Clone)]
pub struct Fill {
    color: crate::libs::color::Pallet,
}

impl Fill {
    pub fn new() -> Self {
        Self {
            color: crate::libs::color::Pallet::gray(0),
        }
    }
}

#[derive(Clone)]
pub enum Layer {
    ShapeGroup(BlockId),
    Drawing(Drawing),
    Image(Image),
    Fill(Fill),
    LayerGroup(BlockId),
}

impl Layer {
    pub fn new_shape_group(shape_group: BlockId) -> Self {
        Self::ShapeGroup(shape_group)
    }

    pub fn new_drawing(drawing_texture_id: BlockId, drawed_texture_id: BlockId) -> Self {
        Self::Drawing(Drawing::new(drawing_texture_id, drawed_texture_id))
    }

    pub fn new_image() -> Self {
        Self::Image(Image::new())
    }

    pub fn new_layer_group(layer_group: BlockId) -> Self {
        Self::LayerGroup(layer_group)
    }

    pub fn as_drawing(&self) -> Option<&Drawing> {
        match self {
            Self::Drawing(x) => Some(x),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct LayerGroup {
    layers: SelectList<Layer>,
}

impl LayerGroup {
    pub fn new() -> Self {
        Self {
            layers: SelectList::new(vec![], 0),
        }
    }

    pub fn layers(&self) -> std::slice::Iter<Layer> {
        self.layers.iter()
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, idx: usize) -> Option<Layer> {
        self.layers.remove(idx)
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        self.layers.selected()
    }
}

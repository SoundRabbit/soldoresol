uses! {
    super::util::Pack;
    super::BlockMut;
    super::Character;
    super::Scene;
    super::super::resource::ImageData;
    super::super::resource::BlockTexture;
}

block! {
    [pub World(constructor, pack)]
    characters: Vec<BlockMut<Character>> = vec![];
    scenes: Vec<BlockMut<Scene>> = vec![];
    selecting_scene: BlockMut<Scene> = BlockMut::<Scene>::none();
    image_data_resources: Vec<BlockMut<ImageData>> = vec![];
    block_texture_resources: Vec<BlockMut<BlockTexture>> = vec![];
}

impl World {
    pub fn characters(&self) -> &Vec<BlockMut<Character>> {
        &self.characters
    }

    pub fn push_character(&mut self, character: BlockMut<Character>) {
        self.characters.push(character);
    }

    pub fn scenes(&self) -> &Vec<BlockMut<Scene>> {
        &self.scenes
    }

    pub fn selecting_scene(&self) -> &BlockMut<Scene> {
        &self.selecting_scene
    }

    pub fn push_scenes(&mut self, scene: BlockMut<Scene>) {
        if self.scenes.len() == 0 {
            self.selecting_scene = BlockMut::clone(&scene);
        }
        self.scenes.push(scene);
    }

    pub fn image_data_resources(&self) -> &Vec<BlockMut<ImageData>> {
        &self.image_data_resources
    }

    pub fn push_image_data_resource(&mut self, image_data: BlockMut<ImageData>) {
        self.image_data_resources.push(image_data);
    }

    pub fn block_texture_resources(&self) -> &Vec<BlockMut<BlockTexture>> {
        &self.block_texture_resources
    }

    pub fn push_block_texture_resource(&mut self, block_texture: BlockMut<BlockTexture>) {
        self.block_texture_resources.push(block_texture);
    }
}

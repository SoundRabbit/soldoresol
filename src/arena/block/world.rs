uses! {
    super::util::Pack;
    super::BlockMut;
    super::Character;
    super::Scene;
}

block! {
    [pub World(constructor, pack)]
    characters: Vec<BlockMut<Character>> = vec![];
    scenes: Vec<BlockMut<Scene>> = vec![];
    selecting_scene: BlockMut<Scene> = BlockMut::<Scene>::none();
}

impl World {
    pub fn characters(&self) -> &Vec<BlockMut<Character>> {
        &self.characters
    }

    pub fn scenes(&self) -> &Vec<BlockMut<Scene>> {
        &self.scenes
    }

    pub fn selecting_scene(&self) -> &BlockMut<Scene> {
        &self.selecting_scene
    }

    pub fn scenes_push(&mut self, scene: BlockMut<Scene>) {
        if self.scenes.len() == 0 {
            self.selecting_scene = BlockMut::clone(&scene);
        }
        self.scenes.push(scene);
    }
}

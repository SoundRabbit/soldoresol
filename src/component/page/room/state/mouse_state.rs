use super::super::renderer::{CameraMatrix, ObjectId, Renderer};
use crate::arena::block::{self, BlockId};
use crate::libs::annotated::Annotated;

pub struct State<T> {
    last: T,
    now: T,
}

#[derive(Clone)]
pub struct CursorPosition {
    page: [f32; 2],
    canvas: [f32; 2],
    canvas_size: [f32; 2],
    world: ([f32; 3], [f32; 3]),
    craftboard: State<[f32; 3]>,
    craftboard_id: BlockId,
}

pub type MouseBtnState = Annotated<State<bool>, State<CursorPosition>>;

pub struct MouseState {
    primary_btn: MouseBtnState,
    secondary_btn: MouseBtnState,
    cursor: State<CursorPosition>,
    wheel: State<f64>,
}

impl<T> State<T> {
    fn new(last: T, now: T) -> Self {
        Self { last, now }
    }

    fn as_ref(&self) -> State<&T> {
        State {
            now: &self.now,
            last: &self.last,
        }
    }

    fn update(&mut self, v: T) {
        std::mem::swap(&mut self.last, &mut self.now);
        self.now = v;
    }

    pub fn last(&self) -> &T {
        &self.last
    }

    pub fn now(&self) -> &T {
        &self.now
    }
}

impl<T: Clone> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            now: T::clone(&self.now),
            last: T::clone(&self.last),
        }
    }
}

impl<T: PartialEq> State<T> {
    fn is_changed(&self) -> bool {
        self.now != self.last
    }
}

impl CursorPosition {
    fn get_position_in_craftboard(
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        craftboard_id: &BlockId,
        canvas_size: &[f32; 2],
        canvas_pos: &[f32; 2],
    ) -> [f32; 3] {
        if let Some(x) = block_arena.map(&craftboard_id, |craftboard: &block::Craftboard| {
            camera_matrix.collision_point_with_z(
                &canvas_size,
                &canvas_pos,
                craftboard.position()[2],
            )
        }) {
            [x[0], x[1], x[2]]
        } else {
            let x = camera_matrix.collision_point_with_z(&canvas_size, &canvas_pos, 0.0);
            [x[0], x[1], x[2]]
        }
    }

    fn empty() -> Self {
        Self {
            page: [0.0, 0.0],
            canvas: [0.0, 0.0],
            canvas_size: [1.0, 1.0],
            world: ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            craftboard: State::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            craftboard_id: BlockId::none(),
        }
    }

    fn new(
        page: [f32; 2],
        renderer: &Renderer,
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        last_craftboard_id: &BlockId,
        canvas_size: &[f32; 2],
        canvas_pos: &[f32; 2],
    ) -> Self {
        let canvas_size = canvas_size.clone();
        let canvas = [page[0] - canvas_pos[0], page[1] - canvas_pos[1]];
        let world = renderer.get_focused_position(camera_matrix, canvas[0], canvas[1]);
        let craftboard_id = match renderer.get_craftboard_object_id(canvas[0], canvas[1]) {
            ObjectId::Craftboard(block_id, _) => block_id,
            _ => BlockId::none(),
        };
        let last_craftboard = Self::get_position_in_craftboard(
            block_arena,
            camera_matrix,
            last_craftboard_id,
            &canvas_size,
            &canvas,
        );
        let now_craftboard = Self::get_position_in_craftboard(
            block_arena,
            camera_matrix,
            &craftboard_id,
            &canvas_size,
            &canvas,
        );
        let craftboard = State::new(last_craftboard, now_craftboard);

        Self {
            page,
            canvas,
            canvas_size,
            world,
            craftboard,
            craftboard_id,
        }
    }

    fn update_craftboard_id(
        &mut self,
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        craftboard_id: &BlockId,
    ) {
        let now_craftboard = Self::get_position_in_craftboard(
            block_arena,
            camera_matrix,
            craftboard_id,
            &self.canvas_size,
            &self.canvas,
        );

        self.craftboard.update(now_craftboard);
    }

    pub fn position_in_page(&self) -> &[f32; 2] {
        &self.page
    }

    pub fn position_in_canvas(&self) -> &[f32; 2] {
        &self.canvas
    }

    pub fn position_in_world(&self) -> (&[f32; 3], &[f32; 3]) {
        (&self.world.0, &self.world.1)
    }

    pub fn position_in_craftboard(&self) -> State<&[f32; 3]> {
        self.craftboard.as_ref()
    }

    pub fn craftboard_id(&self) -> &BlockId {
        &self.craftboard_id
    }
}

impl MouseBtnState {
    fn new() -> Self {
        Self {
            value: State::new(false, false),
            annot: State::new(CursorPosition::empty(), CursorPosition::empty()),
        }
    }

    fn update(
        &mut self,
        page: [f32; 2],
        renderer: &Renderer,
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        canvas_size: &[f32; 2],
        canvas_pos: &[f32; 2],
        is_downed: bool,
    ) {
        self.value.update(is_downed);

        if self.value.is_changed() {
            let last_craftboard_id = self.annot.now.craftboard_id();
            let cursor = CursorPosition::new(
                page,
                renderer,
                block_arena,
                camera_matrix,
                last_craftboard_id,
                canvas_size,
                canvas_pos,
            );
            self.annot
                .now
                .update_craftboard_id(block_arena, camera_matrix, cursor.craftboard_id());
            self.annot.update(cursor);
        }
    }

    pub fn is_clicked(&self) -> bool {
        self.is_upped()
    }

    pub fn is_downed(&self) -> bool {
        *self.value.now() && self.value.is_changed()
    }

    pub fn is_upped(&self) -> bool {
        !self.value.now() && self.value.is_changed()
    }

    pub fn is_dragging(&self) -> bool {
        *self.value.now()
    }

    pub fn drag_start(&self) -> &CursorPosition {
        if self.is_dragging() {
            self.annot.now()
        } else {
            self.annot.last()
        }
    }

    pub fn drag_end(&self) -> &CursorPosition {
        if !self.is_dragging() {
            self.annot.now()
        } else {
            self.annot.last()
        }
    }
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            primary_btn: Annotated {
                value: State::new(false, false),
                annot: State::new(CursorPosition::empty(), CursorPosition::empty()),
            },
            secondary_btn: Annotated {
                value: State::new(false, false),
                annot: State::new(CursorPosition::empty(), CursorPosition::empty()),
            },
            cursor: State::new(CursorPosition::empty(), CursorPosition::empty()),
            wheel: State::new(0.0, 0.0),
        }
    }

    pub fn update(
        &mut self,
        e: &web_sys::MouseEvent,
        renderer: &Renderer,
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        canvas_size: &[f32; 2],
        canvas_pos: &[f32; 2],
    ) {
        let page_x = e.page_x() as f32;
        let page_y = e.page_y() as f32;

        let last_craftboard_id = self.cursor.now.craftboard_id();
        let cursor = CursorPosition::new(
            [page_x, page_y],
            renderer,
            block_arena,
            camera_matrix,
            last_craftboard_id,
            canvas_size,
            canvas_pos,
        );
        self.cursor
            .now
            .update_craftboard_id(block_arena, camera_matrix, cursor.craftboard_id());
        self.cursor.update(cursor);

        let buttons = e.buttons();
        self.primary_btn.update(
            [page_x, page_y],
            renderer,
            block_arena,
            camera_matrix,
            canvas_size,
            canvas_pos,
            buttons & 1 != 0,
        );
        self.secondary_btn.update(
            [page_x, page_y],
            renderer,
            block_arena,
            camera_matrix,
            canvas_size,
            canvas_pos,
            buttons & 1 != 0,
        );
    }

    pub fn update_wheel(&mut self, e: &web_sys::WheelEvent) {
        let now_y = e.delta_y() + self.wheel.now();
        self.wheel.update(now_y);
    }

    pub fn primary_btn(&self) -> &MouseBtnState {
        &self.primary_btn
    }

    pub fn secondary_btn(&self) -> &MouseBtnState {
        &self.secondary_btn
    }

    pub fn cursor(&self) -> &State<CursorPosition> {
        &self.cursor
    }

    pub fn wheel(&self) -> &State<f64> {
        &self.wheel
    }
}

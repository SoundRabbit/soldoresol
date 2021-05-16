use super::super::renderer::{CameraMatrix, Renderer};
use crate::arena::block;
use crate::libs::annotated::Annotated;

pub struct State<T> {
    last: T,
    now: T,
}

#[derive(Clone)]
pub struct CursorPosition {
    page: [f32; 2],
    canvas: [f32; 2],
    world: ([f32; 3], [f32; 3]),
    table: [f32; 3],
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

impl<T: PartialEq> State<T> {
    fn is_changed(&self) -> bool {
        self.now != self.last
    }
}

impl CursorPosition {
    fn empty() -> Self {
        Self {
            page: [0.0, 0.0],
            canvas: [0.0, 0.0],
            world: ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
            table: [0.0, 0.0, 0.0],
        }
    }

    fn new(
        page: [f32; 2],
        renderer: &Renderer,
        block_arena: &block::Arena,
        camera_matrix: &CameraMatrix,
        canvas_size: &[f32; 2],
        canvas_pos: &[f32; 2],
    ) -> Self {
        let canvas = [page[0] - canvas_pos[0], page[1] - canvas_pos[1]];
        let world = renderer.get_focused_position(block_arena, camera_matrix, canvas[0], canvas[1]);
        let table = camera_matrix.collision_point_on_xy_plane(canvas_size, &canvas);
        let table = [table[0], table[1], table[2]];

        Self {
            page,
            canvas,
            world,
            table,
        }
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

    pub fn position_in_table(&self) -> &[f32; 3] {
        &self.table
    }
}

impl MouseBtnState {
    fn new() -> Self {
        Self {
            value: State::new(false, false),
            annot: State::new(CursorPosition::empty(), CursorPosition::empty()),
        }
    }

    fn update(&mut self, is_downed: bool, cursor: &CursorPosition) {
        self.value.update(is_downed);

        if self.value.is_changed() {
            self.annot.update(cursor.clone());
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
        let cursor = CursorPosition::new(
            [page_x, page_y],
            renderer,
            block_arena,
            camera_matrix,
            canvas_size,
            canvas_pos,
        );

        let buttons = e.buttons();
        self.primary_btn.update(buttons & 1 != 0, &cursor);
        self.secondary_btn.update(buttons & 2 != 0, &cursor);
        self.cursor.update(cursor);
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

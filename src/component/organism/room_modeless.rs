use kagura::prelude::*;

#[derive(Clone)]
pub enum Content {
    ChatChannel,
}

pub enum Msg {}

pub enum On {}

pub struct TabName {}
pub struct RoomModeless {}

impl Component for TabName {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for TabName {
    fn constructor(props: &Content) -> Self {
        Self {}
    }
}

impl Update for TabName {}

impl Render for TabName {}

impl Component for RoomModeless {
    type Props = Content;
    type Msg = Msg;
    type Sub = On;
}

impl Constructor for RoomModeless {
    fn constructor(props: &Content) -> Self {
        Self {}
    }
}

impl Update for RoomModeless {}

impl Render for RoomModeless {}

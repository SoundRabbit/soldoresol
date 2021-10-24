use kagura::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Craftboard {}

impl Component for Craftboard {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Craftboard {
    pub fn new() -> PrepackedComponent<Self> {
        PrepackedComponent::new(Self {})
    }
}

impl Update for Craftboard {}

impl Render for Craftboard {}

use super::*;
use crate::libs::random_id::U128Id;
use isaribi::{
    style,
    styled::{Style, Styled},
};

mod value;

impl Render<Html> for BlockProp {
    type Children = ();
    fn render(&self, _: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class("pure-form"),
            Events::new(),
            vec![{
                let id = self.data.id();
                self.data
                    .map(|prop| self.render_prop(id, prop))
                    .unwrap_or_else(|| Html::none())
            }],
        ))
    }
}

impl BlockProp {
    fn render_prop(&self, id: U128Id, prop: &block::Property) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("property"))
                .string(
                    "data-children",
                    match prop.view() {
                        block::property::PropertyView::List => "list",
                        block::property::PropertyView::Board => "board",
                    },
                )
                .string(
                    "data-value",
                    if prop.data().is_empty() {
                        "empty"
                    } else {
                        match prop.data().view() {
                            block::property::DataView::List => "list",
                            block::property::DataView::Tabular => "tablur",
                        }
                    },
                ),
            Events::new(),
            vec![
                Html::input(
                    Attributes::new()
                        .class(Self::class("property-name"))
                        .string("data-id", &id),
                    Events::new(),
                    vec![],
                ),
                self.render_data(prop.data()),
                Html::div(
                    Attributes::new().class("property-children"),
                    Events::new(),
                    prop.children()
                        .iter()
                        .filter_map(|prop| {
                            let id = prop.id();
                            prop.map(|prop| self.render_prop(id, prop))
                        })
                        .collect(),
                ),
            ],
        )
    }

    pub fn render_data(&self, data: &block::property::Data) -> Html {
        Html::div(
            Attributes::new().class(Self::class("property-data")),
            Events::new(),
            data.values()
                .iter()
                .map(|row| match data.view() {
                    block::property::DataView::List => Html::fragment(self.render_data_row(row)),
                    block::property::DataView::Tabular => {
                        Html::div(Attributes::new(), Events::new(), self.render_data_row(row))
                    }
                })
                .collect(),
        )
    }

    pub fn render_data_row(&self, row: &Vec<block::property::Value>) -> Vec<Html> {
        row.iter().map(|value| self.render_value(value)).collect()
    }
}

impl Styled for BlockProp {
    fn style() -> Style {
        style! {}
    }
}

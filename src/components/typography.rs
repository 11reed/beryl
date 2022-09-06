use yew::prelude::*;
use css_style::prelude::*;
use css_style::{Size, Text};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub font_size: Option<Size>,
    pub color: Option<Text>,
    pub align: Option<Text>,
}

#[function_component(Typography)]
pub fn container(props: &Props) -> Html {
    let style = Style::default()
    .try_size(props.font_size.clone())
    .try_text(props.color.clone())
    .try_text(props.align.clone());
    
    html! {
        <div class="typography" style={ style.to_string() }>
            { props.children.clone() }
        </div>
    }
}
use yew::prelude::*;
use css_style::prelude::*;
use css_style::{Size, Background, Text, Padding, Border};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub font_size: Option<Size>,
    pub background: Option<Background>,
    pub color: Option<Text>,
    pub padding: Option<Padding>,
    pub border: Option<Border>,
    pub align: Option<Text>,
}

#[function_component(Button)]
pub fn container(props: &Props) -> Html {
    let style = Style::default()
    .try_size(props.font_size.clone())
    .try_background(props.background.clone())
    .try_padding(props.padding.clone())
    .try_border(props.border.clone())
    .try_text(props.color.clone())
    .try_text(props.align.clone());
    
    html! {
        <button class="button" style={ style.to_string() }>
            { props.children.clone() }
        </button>
    }
}
use yew::prelude::*;
use css_style::prelude::*;
use css_style::{Background, Border, Margin, Padding, BoxShadow};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub background: Option<Background>,
    pub shadow: Option<BoxShadow>,
    pub padding: Option<Padding>,
    pub border: Option<Border>,
    pub margin: Option<Margin>,
}

#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let style = Style::default()
    .try_padding(props.padding.clone())
    .try_margin(props.margin.clone())
    .try_background(props.background.clone())
    .try_border(props.border.clone())
    .try_box_shadow(props.shadow.clone())
    .insert("box-sizing", "border-box");
    
    html! {
        <div class="container" style={ style.to_string() }>
            { props.children.clone() }
        </div>
    }
}
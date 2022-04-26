use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct DialogProps {
    pub children: Children,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    html! {
        <div class="dialog dialog-backdrop">
            { for props.children.iter() }
        </div>
    }
}

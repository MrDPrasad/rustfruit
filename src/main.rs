use yew::prelude::*;
mod navbar;
use navbar::NavBar;

#[function_component(App)]
fn app() -> Html {
    html! {
        <NavBar />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
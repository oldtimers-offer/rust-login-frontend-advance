use crate::components::header::Header;
use crate::components::menu::Menu;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
                <div class="row">
                    <div class="col-sm-auto">
                        <Menu/>
                    </div>
                    <div class="col-mt-3">
                        <Header />
                    </div>
                </div>
        </div>
    }
}

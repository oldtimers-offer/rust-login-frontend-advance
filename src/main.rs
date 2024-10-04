use crate::contexts::CurrentUserProvider;
use yew::prelude::*;
use yew_router::prelude::*;
mod api;
mod components;
mod contexts;
mod pages;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::Home /> },
        Route::Login => html! { <pages::login::Login /> },
        Route::NotFound => html! { <pages::not_found::NotFound /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <CurrentUserProvider> //GLOBAL CONTEXT
                <Switch<Route> render={switch}/>
            </CurrentUserProvider> //GLOBAL CONTEXT
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

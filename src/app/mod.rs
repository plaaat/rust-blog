use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod blog;

use crate::components::nav::Nav;
use about::About;
use home::Home;
use blog::Blog;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Home,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Blog => html! { <Blog /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div class="flex min-h-screen flex-col">
                <Nav />
                <Switch<AppRoute> render={switch} />
            </div>
        </HashRouter>
    }
}

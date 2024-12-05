use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
html! {
        <nav class="bg-gray-800 p-4">
            <div class="container mx-auto flex justify-between items-center">
                <Link<AppRoute> 
                    classes="flex items-center space-x-4"
                    to={AppRoute::Home}
                >
                    <img src="logo.svg" alt="로고" class="h-10 w-10" />
                </Link<AppRoute>>
                <div class="flex space-x-4">
                    <Link<AppRoute> 
                        classes="px-4 py-2 text-white hover:bg-gray-700 rounded-md transition duration-300"
                        to={AppRoute::Home}>
                        { "Home" }
                    </Link<AppRoute>>
                    
                    <Link<AppRoute> 
                        classes="px-4 py-2 text-white hover:bg-gray-700 rounded-md transition duration-300"
                        to={AppRoute::About}>
                        { "About" }
                    </Link<AppRoute>>
                    <Link<AppRoute> 
                        classes="px-4 py-2 text-white hover:bg-gray-700 rounded-md transition duration-300"
                        to={AppRoute::Blog}>
                        { "Blog" }
                    </Link<AppRoute>>
                    
                </div>
            </div>
        </nav>
    }
}

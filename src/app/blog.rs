use yew::prelude::*;
use yew::virtual_dom::VNode;
use markdown::to_html;

#[derive(Clone, PartialEq)]
pub enum Category {
    All,
    PS,
    WebDev,
    Other,
}

#[derive(Clone, PartialEq)]
pub struct Post {
    title: String,
    content: String,
    date: String,
    category: Category,
    contentnum: i32,
}

#[derive(Properties, PartialEq)]
pub struct CategoryProps {
    pub name: String,
    pub count: i32,
    pub selected: bool,
    pub category: Category,
    pub onclick: Callback<Category>,
}

#[function_component(CategoryItem)]
fn category_item(props: &CategoryProps) -> Html {
    let selected_class = if props.selected {
        "bg-gray-100 text-gray-900"
    } else {
        "text-gray-600 hover:bg-gray-50"
    };

    let category = props.category.clone();
    let onclick = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        onclick.emit(category.clone());
    });

    html! {
        <a onclick={onclick}
           class={format!("flex justify-between px-4 py-2 text-sm {} rounded-md cursor-pointer", selected_class)}>
            <span>{&props.name}</span>
            <span class="text-gray-500">{{&props.count}}</span>
        </a>
    }
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let selected_category = use_state(|| Category::All);
    let posts = use_state(|| {
        vec![
            Post {
                title: "웹개발 카테고리 테스트".to_string(),
                content: "웹개발 카테고리 테스트".to_string(),
                date: "2024년 12월 1일".to_string(),
                category: Category::WebDev,
                contentnum: 1,
            },
            Post {
                title: "PS 카테고리 테스트".to_string(),
                content: "PS 카테고리 테스트".to_string(),
                date: "2024년 12월 2일".to_string(),
                category: Category::PS,
                contentnum: 2,
            },
            Post {
                title: "기타 카테고리 테스트".to_string(),
                content: "기타 카테고리 테스트".to_string(),
                date: "2024년 12월 3일".to_string(),
                category: Category::Other,
                contentnum: 3,
            },
            Post {
                title: "살려주세요".to_string(),
                content: "어케해야될지모르겠어요".to_string(),
                date: "2024년 12월 12일".to_string(),
                category: Category::Other,
                contentnum: 4,
            },
        ]
    });

    let selected_post_content = use_state(|| String::new());
    let content_cache = use_state(|| std::collections::HashMap::<i32, String>::new());

    let markdown_content = {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&to_html(&*selected_post_content));
        VNode::VRef(div.into())
    };
    
    let filtered_posts = posts.iter().filter(|post| {
        match (*selected_category).clone() {
            Category::All => true,
            category => post.category == category,
        }
    });

    let on_category_select = {
        let selected_category = selected_category.clone();
        Callback::from(move |category: Category| {
            selected_category.set(category);
        })
    };

    let count_posts = |category: &Category| {
        posts.iter().filter(|post| {
            match category {
                Category::All => true,
                category => &post.category == category,
            }
        }).count() as i32
    };

    let on_post_click = {
        let selected_post_content = selected_post_content.clone();
        let content_cache = content_cache.clone();
        Callback::from(move |contentnum: i32| {
            let selected_post_content = selected_post_content.clone();
            let content_cache = content_cache.clone();
            
            if let Some(cached_content) = content_cache.get(&contentnum) {
                selected_post_content.set(cached_content.clone());
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                let response = reqwest::get(format!("https://api.plaaa.at/blog/{}.md", contentnum))
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                
                let mut new_cache = (*content_cache).clone();
                new_cache.insert(contentnum, response.clone());
                content_cache.set(new_cache);
                
                selected_post_content.set(response);
            });
        })
    };

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="flex flex-col lg:flex-row gap-8">
                //왼쪽 카테고리 바
                <div class="w-full lg:w-64 flex-shrink-0">
                    <h1 class="text-4xl font-bold font-nanum-gothic mb-8 text-center">{"블로그"}</h1>
                    <div class="bg-white rounded-lg shadow-md p-4">
                        <h2 class="text-lg font-semibold font-nanum-gothic mb-4">{"카테고리"}</h2>
                        <div class="space-y-1">
                            <CategoryItem 
                                name="전체글" 
                                count={count_posts(&Category::All)} 
                                selected={*selected_category == Category::All}
                                category={Category::All}
                                onclick={on_category_select.clone()}
                            />
                            <CategoryItem 
                                name="PS" 
                                count={count_posts(&Category::PS)} 
                                selected={*selected_category == Category::PS}
                                category={Category::PS}
                                onclick={on_category_select.clone()}
                            />
                            <CategoryItem 
                                name="웹 개발" 
                                count={count_posts(&Category::WebDev)} 
                                selected={*selected_category == Category::WebDev}
                                category={Category::WebDev}
                                onclick={on_category_select.clone()}
                            />
                            <CategoryItem 
                                name="기타" 
                                count={count_posts(&Category::Other)} 
                                selected={*selected_category == Category::Other}
                                category={Category::Other}
                                onclick={on_category_select.clone()}
                            />
                        </div>
                    </div>
                    <br/>
                    <br/>
                    <div class="grid grid-cols-1 gap-1">
                        {for filtered_posts.map(|post| {
                            let contentnum = post.contentnum;
                            html! {
                                <div class="bg-white rounded-lg shadow-md p-2 hover:bg-gray-50" 
                                    onclick={on_post_click.reform(move |_| contentnum)} 
                                >
                                    <h2 class="text-l font-semibold font-nanum-gothic mb-1">{&post.title}</h2>
                                    <div class="mt-1">
                                        <span class="text-sm text-gray-500 font-nanum-gothic">{&post.date}</span>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>

                //메인 콘텐츠 영역
                <div class="flex-grow font-nanum-gothic">
                    <div class="mt-8 prose prose-slate max-w-none">
                        {markdown_content}
                    </div>
                </div>
            </div>
        </div>
    }
}
use yew::prelude::*;

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
            },
            Post {
                title: "PS 카테고리 테스트".to_string(),
                content: "PS 카테고리 테스트".to_string(),
                date: "2024년 12월 2일".to_string(),
                category: Category::PS,
            },
            Post {
                title: "기타 카테고리 테스트".to_string(),
                content: "기타 카테고리 테스트".to_string(),
                date: "2024년 12월 3일".to_string(),
                category: Category::Other,
            },
            Post {
                title: "테스트중입니다...".to_string(),
                content: "어케해야될지모르겠어요".to_string(),
                date: "대가리박치기중입니다".to_string(),
                category: Category::Other,
            },
        ]
    });

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

    html! {
        <div class="container mx-auto px-4 py-8">
            <div class="flex gap-8">
                // 왼쪽 카테고리 바
                <div class="w-64 flex-shrink-0">
                    <h1 class="text-4xl font-bold mb-8 text-center">{"블로그"}</h1>
                    <div class="bg-white rounded-lg shadow-md p-4">
                        <h2 class="text-lg font-semibold mb-4">{"카테고리"}</h2>
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
                                name="웹개발" 
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
                </div>

                // 메인 콘텐츠 영역
                <div class="flex-grow">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-6">
                        {for filtered_posts.map(|post| {
                            html! {
                                <div class="bg-white rounded-lg shadow-md p-6">
                                    <h2 class="text-xl font-semibold mb-4">{&post.title}</h2>
                                    <p class="text-gray-600">{&post.content}</p>
                                    <div class="mt-4">
                                        <span class="text-sm text-gray-500">{&post.date}</span>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}
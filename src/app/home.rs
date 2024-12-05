use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container text-center">
            <header class="space-y-8">
                <p>
                    <img class="w-48 h-48 mx-auto mt-24" src="logo.svg" alt="PlaaT" />
                </p>
                <p class="text-3xl font-bold font-nanum-gothic">
                    { "안녕하세요!" }
                    <br/>
                    <code>{ "PlaaaT" }</code>{ "의 블로그입니다." }
                </p>
                <p class="text-xl font-nanum-gothic">
                    { "블로그 아이콘은 GPT의 Dall-E 3으로 만든후 수정을 가했고," }
                    <br/>
                    { "블로그는 " }
                    <a href="https://github.com/jetli/create-yew-app" 
                    target="_blank" 
                    class="text-gray-700 hover:text-gray-900 transition-colors font-bold">{ "create-yew-app" }</a>
                    { " 에 기반하여 "}
                    <br/>
                    { "cloudflare 의 Pages 배포로 만들었습니다. " }
                    <br/>
                    <br/>
                    { "24.11.30 부터 시작합니다. 조금씩 더해갈 예정이예요."}
                    <br/>
                    { " 경험을 쌓기 위해 만들었습니다."}
                </p>
                <p class="flex justify-center items-center space-x-2 font-nanum-gothic">
                    <a 
                        href="https://github.com/plaaat" 
                        target="_blank" 
                        class="text-gray-700 hover:text-gray-900 transition-colors"
                    >
                        <svg 
                            class="w-8 h-8" 
                            viewBox="0 0 24 24" 
                            fill="currentColor"
                        >
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                    </a>
                    <span>{ " || " }</span>
                    <a href="https://www.acmicpc.net/user/plaaat0102" 
                    target="_blank" 
                    class="text-blue-700 hover:text-blue-700 transition-colors font-bold">{ "/<>" }</a>
                </p>
                <br/>
                <br/>
            </header>
        </div>
    }
}

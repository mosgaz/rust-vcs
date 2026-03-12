use leptos::{component, view, IntoView};

#[component]
fn LandingPage() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"rust-vcs MVP"</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-slate-950 text-slate-100 min-h-screen">
                <main class="mx-auto max-w-4xl p-8">
                    <header class="mb-10">
                        <p class="text-sm uppercase tracking-widest text-indigo-300">"Leptos + Tailwind"</p>
                        <h1 class="text-4xl font-bold mt-2">"rust-vcs: Этап 1 (MVP)"</h1>
                        <p class="text-slate-300 mt-3">"Веб-интерфейс на Leptos и Tailwind для входа в встречу по ссылке и сигналинга WebRTC."</p>
                    </header>

                    <section class="grid md:grid-cols-2 gap-6">
                        <article class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow">
                            <h2 class="text-xl font-semibold">"Создать встречу"</h2>
                            <p class="text-slate-400 mt-2">"Для сотрудников (требуется JWT через /v1/auth/login)."</p>
                            <code class="block mt-4 text-sm text-emerald-300">"POST /v1/meetings"</code>
                        </article>

                        <article class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow">
                            <h2 class="text-xl font-semibold">"Подключиться по ссылке"</h2>
                            <p class="text-slate-400 mt-2">"Для внешних участников без регистрации."</p>
                            <code class="block mt-4 text-sm text-emerald-300">"POST /v1/meetings/:slug/join"</code>
                        </article>
                    </section>

                    <section class="mt-8 rounded-2xl border border-indigo-700/40 bg-indigo-950/30 p-6">
                        <h3 class="text-lg font-semibold">"WebRTC signaling"</h3>
                        <p class="text-indigo-100/80 mt-2">"Для SDP/ICE используйте websocket-канал:"</p>
                        <code class="block mt-3 text-indigo-200">"GET /v1/meetings/:slug/signal/ws"</code>
                    </section>
                </main>
            </body>
        </html>
    }
}

pub fn render_landing_page() -> String {
    leptos::ssr::render_to_string(|| view! { <LandingPage/> }).to_string()
}

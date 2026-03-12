use leptos::{component, view, IntoView};

#[component]
fn LandingPage() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"rust-vcs Stage 2"</title>
                <script src="https://cdn.tailwindcss.com"></script>
            </head>
            <body class="bg-slate-950 text-slate-100 min-h-screen">
                <main class="mx-auto max-w-5xl p-8">
                    <header class="mb-10">
                        <p class="text-sm uppercase tracking-widest text-indigo-300">"Leptos + Tailwind"</p>
                        <h1 class="text-4xl font-bold mt-2">"rust-vcs: Этап 2"</h1>
                        <p class="text-slate-300 mt-3">"Корпоративный мессенджер, режим вебинара, запись встреч и статус desktop (Tauri)."</p>
                    </header>

                    <section class="grid lg:grid-cols-2 gap-6">
                        <article class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow">
                            <h2 class="text-xl font-semibold">"Персистентные чаты"</h2>
                            <p class="text-slate-400 mt-2">"Списки диалогов и каналов с аватарами."</p>
                            <ul class="mt-4 space-y-3 text-sm">
                                <li class="flex items-center gap-3"><span class="inline-flex h-8 w-8 items-center justify-center rounded-full bg-indigo-500 font-bold">"A"</span><span>"# backend-team"</span></li>
                                <li class="flex items-center gap-3"><span class="inline-flex h-8 w-8 items-center justify-center rounded-full bg-emerald-500 font-bold">"D"</span><span>"Дмитрий Петров"</span></li>
                            </ul>
                            <code class="block mt-4 text-emerald-300">"/v1/messenger/threads"</code>
                        </article>

                        <article class="rounded-2xl border border-slate-800 bg-slate-900 p-6 shadow">
                            <h2 class="text-xl font-semibold">"Вебинар и запись"</h2>
                            <p class="text-slate-400 mt-2">"Права спикеров и запуск серверной записи."</p>
                            <code class="block mt-4 text-emerald-300">"POST /v1/meetings/:slug/webinar/speakers"</code>
                            <code class="block mt-2 text-emerald-300">"POST /v1/meetings/:slug/recordings/start"</code>
                        </article>
                    </section>

                    <section class="mt-8 rounded-2xl border border-indigo-700/40 bg-indigo-950/30 p-6">
                        <h3 class="text-lg font-semibold">"Desktop readiness (Tauri)"</h3>
                        <p class="text-indigo-100/80 mt-2">"Статус целевых desktop-платформ:"</p>
                        <code class="block mt-3 text-indigo-200">"GET /v1/desktop/status"</code>
                    </section>
                </main>
            </body>
        </html>
    }
}

pub fn render_landing_page() -> String {
    leptos::ssr::render_to_string(|| view! { <LandingPage/> }).to_string()
}

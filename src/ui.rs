use leptos::{component, view, IntoView};

fn render_shell(title: &str, body: impl FnOnce() -> leptos::View + 'static) -> String {
    let page_title = title.to_string();
    leptos::ssr::render_to_string(move || {
        let content = body();
        view! {
            <!DOCTYPE html>
            <html lang="ru">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <title>{page_title.clone()}</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                </head>
                <body class="bg-slate-950 text-slate-100 min-h-screen">
                    <main class="mx-auto max-w-5xl p-8">{content}</main>
                </body>
            </html>
        }
    })
    .to_string()
}

#[component]
fn LandingPage() -> impl IntoView {
    view! {
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
            <h3 class="text-lg font-semibold">"UI страницы Этапа 2"</h3>
            <ul class="mt-3 text-indigo-100/90 space-y-1">
                <li>"/auth/login"</li>
                <li>"/auth/register"</li>
                <li>"/messenger"</li>
                <li>"/meetings/:slug"</li>
                <li>"/meetings/:slug/waiting-room"</li>
            </ul>
        </section>
    }
}

pub fn render_landing_page() -> String {
    render_shell("rust-vcs Stage 2", || view! { <LandingPage/> }.into_view())
}

pub fn render_login_page() -> String {
    render_shell("Вход в rust-vcs", || {
        view! {
            <h1 class="text-3xl font-bold mb-6">"Авторизация"</h1>
            <form class="space-y-4 max-w-md rounded-xl bg-slate-900 p-6 border border-slate-800">
                <input class="w-full rounded bg-slate-800 p-3" type="email" placeholder="Email" />
                <input class="w-full rounded bg-slate-800 p-3" type="password" placeholder="Пароль" />
                <button class="w-full rounded bg-indigo-600 py-3 font-semibold">"Войти"</button>
            </form>
        }
        .into_view()
    })
}

pub fn render_register_page() -> String {
    render_shell("Регистрация в rust-vcs", || {
        view! {
            <h1 class="text-3xl font-bold mb-6">"Регистрация"</h1>
            <form class="space-y-4 max-w-md rounded-xl bg-slate-900 p-6 border border-slate-800">
                <input class="w-full rounded bg-slate-800 p-3" type="text" placeholder="Имя" />
                <input class="w-full rounded bg-slate-800 p-3" type="email" placeholder="Корпоративный email" />
                <input class="w-full rounded bg-slate-800 p-3" type="password" placeholder="Пароль" />
                <button class="w-full rounded bg-emerald-600 py-3 font-semibold">"Создать аккаунт"</button>
            </form>
        }
        .into_view()
    })
}

pub fn render_messenger_page() -> String {
    render_shell("Мессенджер rust-vcs", || {
        view! {
            <h1 class="text-3xl font-bold mb-6">"Корпоративный мессенджер"</h1>
            <section class="grid md:grid-cols-[280px_1fr] gap-6">
                <aside class="rounded-xl border border-slate-800 bg-slate-900 p-4">
                    <h2 class="font-semibold mb-3">"Чаты"</h2>
                    <ul class="space-y-2 text-sm text-slate-300">
                        <li class="p-2 rounded bg-slate-800">"# general"</li>
                        <li class="p-2 rounded bg-slate-800">"# backend-team"</li>
                        <li class="p-2 rounded bg-slate-800">"Алиса ↔ Дмитрий"</li>
                    </ul>
                </aside>
                <article class="rounded-xl border border-slate-800 bg-slate-900 p-4">
                    <h2 class="font-semibold mb-3">"Сообщения"</h2>
                    <div class="space-y-3 text-sm">
                        <p class="rounded bg-slate-800 p-3">"Привет, подготовили демо Этапа 2"</p>
                        <p class="rounded bg-indigo-900/60 p-3">"Да, добавили треды и записи встреч"</p>
                    </div>
                </article>
            </section>
        }
        .into_view()
    })
}

pub fn render_meeting_page(slug: &str) -> String {
    let slug = slug.to_string();
    render_shell("Встреча rust-vcs", move || {
        view! {
            <h1 class="text-3xl font-bold">{format!("Встреча {slug}")}</h1>
            <p class="text-slate-300 mt-2">"Комната активна. Здесь будут видео-плитки, чат и управление записью."</p>
            <div class="mt-6 rounded-xl border border-slate-800 bg-slate-900 p-5">
                <code class="text-emerald-300">{format!("GET /v1/meetings/{slug}/signal/ws")}</code>
            </div>
        }
        .into_view()
    })
}

pub fn render_waiting_room_page(slug: &str) -> String {
    let slug = slug.to_string();
    render_shell("Комната ожидания", move || {
        view! {
            <h1 class="text-3xl font-bold">"Комната ожидания"</h1>
            <p class="text-slate-300 mt-2">{format!("Вы ожидаете подтверждения входа во встречу {slug}")}</p>
            <button class="mt-6 rounded bg-indigo-600 px-6 py-3 font-semibold">"Запросить вход"</button>
        }
        .into_view()
    })
}

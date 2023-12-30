use crate::Page;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar(
    logged_in: Signal<bool>,
    #[prop(into)] on_logout: Callback<()>,
) -> impl IntoView {
    let click_count = create_rw_signal(0);
    let show_links = Signal::derive(move || {
        click_count.get() >= 6
    });
    view! {
        <div class="navbar bg-base-100">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl no-animation"
                    on:click=move |_| {click_count.update(|c| *c += 1);}
                >"How much did I lift?"</a>
            </div>
            <div class="flex-none">
                <Show when=move || show_links.get()>
                    <ul class="menu menu-horizontal px-1">
                        <li><A href=Page::Convert.path()>"Convert"</A></li>
                        <Show
                            when=move || logged_in.get()
                            fallback=|| {
                                view! {
                                    <li><A href=Page::Login.path()>"Login"</A></li>
                                    <li><A href=Page::Register.path()>"Register"</A></li>
                                }
                            }
                        >
                            <li><A href=Page::MeasureList.path()>"Measures"</A></li>
                            <li><a href="#" on:click=move |_| on_logout.call(())>
                                "Logout"
                            </a></li>
                        </Show>
                    </ul>
                </Show>
            </div>
        </div>
    }
}
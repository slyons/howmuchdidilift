use leptos::{ev, *};

#[component]
pub fn CredentialsForm(
    title: &'static str,
    action_label: &'static str,
    action: Action<(String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>
) -> impl IntoView {
    let (pw, set_pw) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());

    let dispatch_action = move || action.dispatch((email.get(), pw.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || pw.get().is_empty() || email.get().is_empty()
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <p>{title}</p>
            {
                move || {
                    error
                        .get()
                        .map(|err| {
                            view! { <p style="color: red;">{err}</p> }
                        })
                }
            }
            <input
                type="email"
                required
                placeholder="Email Address"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
            />
            <input
                type="password"
                required
                placeholder="Password"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    match &*ev.key() {
                        "Enter" => dispatch_action(),
                        _ => {
                            let val = event_target_value(&ev);
                            set_pw.update(|p| *p = val);
                        }
                    }
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_pw.update(|p| *p = val);
                }
            />
            <button
                prop:disabled=move || button_is_disabled.get()
                on:click=move |_| dispatch_action()
            >
                {action_label}
            </button>
        </form>
    }
}

#[component]
pub fn RegistrationForm(
    title: &'static str,
    action_label: &'static str,
    action: Action<(String, String, String, String), ()>,
    error: Signal<Option<String>>,
    disabled: Signal<bool>
) -> impl IntoView {
    let (pw, set_pw) = create_signal(String::new());
    let (pw_confirm, set_pw_confirm) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());

    let dispatch_action = move || action.dispatch((name.get(), email.get(), pw.get(), pw_confirm.get()));

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || pw.get().is_empty() || email.get().is_empty() ||
            pw_confirm.get().is_empty() || name.get().is_empty() ||
            (pw.get() != pw_confirm.get())
    });

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <p>{title}</p>
            {
                move || {
                    error
                        .get()
                        .map(|err| {
                            view! { <p style="color: red;">{err}</p> }
                        })
                }
            }
            <input
                type="email"
                required
                placeholder="Email Address"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_email.update(|v| *v = val);
                }
            />
            <input
                type="text"
                required
                placeholder="Username"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    set_name.update(|v| *v = val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_name.update(|v| *v = val);
                }
            />
            <input
                type="password"
                required
                placeholder="Password"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    match &*ev.key() {
                        "Enter" => dispatch_action(),
                        _ => {
                            let val = event_target_value(&ev);
                            set_pw.update(|p| *p = val);
                        }
                    }
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_pw.update(|p| *p = val);
                }
            />
            <input
                type="password"
                required
                placeholder="Password Confirmation"
                prop:disabled=move || disabled.get()
                on:keyup=move |ev: ev::KeyboardEvent| {
                    match &*ev.key() {
                        "Enter" => dispatch_action(),
                        _ => {
                            let val = event_target_value(&ev);
                            set_pw_confirm.update(|p| *p = val);
                        }
                    }
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_pw_confirm.update(|p| *p = val);
                }
            />
            <button
                prop:disabled=move || button_is_disabled.get()
                on:click=move |_| dispatch_action()
            >
                {action_label}
            </button>
        </form>
    }
}
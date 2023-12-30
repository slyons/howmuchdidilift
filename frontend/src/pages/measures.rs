use leptos::*;
use leptos_router::*;
//use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use crate::api::AuthorizedApi;
use interface::{Measure, MeasureCreate};
use crate::Page;

#[component]
pub fn MeasureForm(
    api: Signal<Option<AuthorizedApi>>,
    existing: Signal<Option<Measure>>,
    #[prop(into)] on_save: Callback<()>
) -> impl IntoView {
    //let name = create_slice(existing, |existing| existing.)
    let name = create_rw_signal(String::new());
    let name_plural = create_rw_signal(String::new());
    let grams = create_rw_signal(String::new());
    let submit_error = create_rw_signal(None::<String>);
    let waiting = create_rw_signal(false);

    let grams_valid = Signal::derive(move || {
        if let Ok(f) = grams.get().parse::<f64>() {
            f > 0.0
        } else {
            false
        }
    });

    create_effect(move |_| {
        log::debug!("Existing is {:?}", existing.get());
        match existing.get() {
            None => {
                name.set(String::new());
                name_plural.set(String::new());
                grams.set(String::new());
            }
            Some(e) => {
                name.set(e.name);
                name_plural.set(e.name_plural);
                grams.set(e.grams.to_string());
            }
        }
    });

    let disabled = Signal::derive(move || {
        waiting.get() || name.get().is_empty() || name_plural.get().is_empty() || !grams_valid.get()
    });
    let submit_action = create_action(move |_| {
        async move {
            log::debug!("Submitting measurement");
            if let Some(ex) = existing.get_untracked() {
                let measure = Measure {
                    id: ex.id,
                    name: name.get(),
                    name_plural: name_plural.get(),
                    grams: grams.get().parse::<f64>().unwrap()
                };
                waiting.set(true);
                let result =
                    //api.with(|a| async { a.unwrap().update_one(measure).await; }).await;
                    api.get_untracked().as_ref().unwrap().update_one(measure).await;
                waiting.set(false);
                log::debug!("Measure is {:?}", result);
                match result {
                    Ok(m) => {
                        name.set(String::new());
                        name_plural.set(String::new());
                        grams.set(String::new());
                        submit_error.set(None);
                        on_save.call(())
                    }
                    Err(err) => {
                        submit_error.set(Some(err.to_string()))
                    }
                };
            } else {
                let measure_create = MeasureCreate {
                    name: name.get(),
                    name_plural: name_plural.get(),
                    grams: grams.get().parse::<f64>().unwrap()
                };
                waiting.set(true);
                let result =
                    api.get_untracked().as_ref().unwrap().add(measure_create).await;
                //api.get().unwrap().add(measure_create).await;
                waiting.set(false);
                match result {
                    Ok(m) => {
                        name.set(String::new());
                        name_plural.set(String::new());
                        grams.set(String::new());
                        submit_error.set(None);
                        on_save.call(())
                    }
                    Err(err) => {
                        submit_error.set(Some(err.to_string()))
                    }
                };
            }
        }
    });


    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <input
                type="text"
                placeholder="Name"
                prop:value=name
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    name.set(val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    name.set(val);
                }
            />
            <input
                type="text"
                placeholder="Plural name"
                prop:value=name_plural
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    name_plural.set(val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    name_plural.set(val);
                }
            />
            <input
                type="number"
                placeholder="Grams"
                prop:value=grams
                on:keyup=move |ev: ev::KeyboardEvent| {
                    let val = event_target_value(&ev);
                    grams.set(val);
                }
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    grams.set(val);
                }
            />
            <button
                prop:disabled=move|| disabled.get()
                on:click=move |_| submit_action.dispatch(())
            >"Submit"</button>
        </form>
    }
}

#[component]
pub fn MeasureList(#[prop(into)] api: Signal<Option<AuthorizedApi>>) -> impl IntoView {
    if api.get().is_none() {
        use_navigate()(Page::Login.path(), Default::default());
    }

    let fetch_error = create_rw_signal(None::<String>);
    let measures = create_resource(|| (), move |_|  {
        async move {
            match api.get_untracked().as_ref().unwrap().list().await {
                Ok(m) => m,
                Err(err) => {
                    fetch_error.set(Some(err.to_string()));
                    vec![]
                }
            }
        }
    });

    let ms = Signal::derive(move || {
        measures.get().unwrap_or_default()
    });

    let edit_measure = create_rw_signal(None::<Measure>);
    let delete_action = create_action(move |id:&i32| {
        let id = *id;
        async move {
            //api.with(|a| async { a.and_then()a.as_ref().unwrap().delete_one(*id).await; }).await;
            api.get().as_ref().unwrap().delete_one(id).await
        }
    });

    view! {
        <div>
            <MeasureForm
                api=api
                existing=edit_measure.into()
                on_save=move |_m| {
                    measures.refetch();
                    edit_measure.set(None);
                }
            />
        </div>
        <Transition
            fallback=move|| view! {<p>"Loading..."</p>}
        >
            <div>
                <p>{fetch_error}</p>
            </div>
            <div>
                <table class="table">
                    <thead>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Plural Name</th>
                        <th>Grams</th>
                        <th> </th>
                    </thead>
                    <tbody>
                        <For
                            each=move || {ms.get()}
                            key=move |state| state.id
                            let:child
                            children=move |m| {
                                let m2 = m.clone();
                                view! {
                                    <tr>
                                        <td>{m.id}</td>
                                        <td>{m.name}</td>
                                        <td>{m.name_plural}</td>
                                        <td>{m.grams}</td>
                                        <td>
                                            <button on:click=move |_| {
                                                log::debug!("Setting Edit measure to {:?}", m2.clone());
                                                edit_measure.set(Some(m2.clone()))
                                            }>
                                                "Edit"
                                            </button>
                                            <button on:click=move |_| {
                                                delete_action.dispatch(m.id)
                                            }>"X"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        >

                        </For>
                    </tbody>
                </table>
            </div>
        </Transition>
    }
}
use leptos::*;
use leptos_router::*;
//use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use crate::api::AuthorizedApi;
use interface::{Measure, MeasureCreate};
use crate::Page;

#[component]
pub fn MeasureForm(
    existing: Signal<Option<Measure>>,
    #[prop(into)] on_save: Callback<()>
) -> impl IntoView {
    let api = expect_context::<Signal<Option<AuthorizedApi>>>();
    let name = create_rw_signal(existing.get().as_ref().map(|e| e.name.to_string()).unwrap_or_default());
    let name_plural = create_rw_signal(existing.get().as_ref().map(|e| e.name_plural.to_string()).unwrap_or_default());
    let grams = create_rw_signal(existing.get().as_ref().map(|e| e.grams.to_string()).unwrap_or_default());
    let submit_error = create_rw_signal(None::<String>);
    let waiting = create_rw_signal(false);

    let grams_valid = Signal::derive(move || {
        if let Ok(f) = grams.get().parse::<f64>() {
            f > 0.0
        } else {
            false
        }
    });

    let disabled = Signal::derive(move || {
        waiting.get() || name.get().is_empty() || name_plural.get().is_empty() || !grams_valid.get()
    });
    let submit_action = create_action(move |_| {
        let api = expect_context::<Signal<Option<AuthorizedApi>>>();
        async move {
            log::debug!("Submitting measurement");
            if let Some(ex) = existing.get() {
                let measure = Measure {
                    id: ex.id,
                    name: name.get(),
                    name_plural: name_plural.get(),
                    grams: grams.get().parse::<f64>().unwrap()
                };
                waiting.set(true);
                let result =
                    //api.with(|a| async { a.unwrap().update_one(measure).await; }).await;
                    api.get().as_ref().unwrap().update_one(measure).await;
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
                    api.get().as_ref().unwrap().add(measure_create).await;
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
        let api = expect_context::<Signal<Option<AuthorizedApi>>>();
        async move {
            match api.get().as_ref().unwrap().list().await {
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
            let api = expect_context::<Signal<Option<AuthorizedApi>>>();
            //api.with(|a| async { a.and_then()a.as_ref().unwrap().delete_one(*id).await; }).await;
            api.get().as_ref().unwrap().delete_one(id).await
        }
    });

    view! {
        <div>
            <MeasureForm
                existing=edit_measure.into()
                on_save=move |_m| {
                    measures.refetch();
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
                <table>
                    <thead>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Plural Name</th>
                        <th>Grams</th>
                        <th> </th>
                    </thead>
                    <tbody>
                        <For
                            each=ms
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
                                                edit_measure.set(Some(m2.clone()))
                                            }>
                                                "Edit"
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
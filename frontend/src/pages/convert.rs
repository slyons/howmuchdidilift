use crate::{
    api::UnauthorizedApi
};
use interface::*;
use leptos::*;
use std::str::FromStr;
use thaw::{Button, ButtonVariant};

#[component]
pub fn Convert(api: UnauthorizedApi) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    let (amt, set_amt) = create_signal(String::new());
    let (input_type, set_input_type) = create_signal("Lbs".to_string());
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let (responses, set_responses) = create_signal(vec![]);

    let convert_action = create_action(move |(weight, unit): &(String, String)| {
        log::debug!("Requesting weight conversion of {}{}", weight, unit);
        let weight = weight.to_string().parse::<f64>().unwrap();
        let unit = InputWeightType::from_str(unit).unwrap();
        let req = RandomWeightRequest {
            input_amt: weight,
            input_type: unit
        };
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = api.convert(req).await;
            log::debug!("Got response {:?}", result);
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_responses.update(|resps| resps.insert(0, res));
                }
                Err(e) => {
                    set_error.update(|err| *err = Some(e.to_string()))
                }
            }
        }

    });

    let disabled = Signal::derive(move || {
        wait_for_response.get()
    });
    let submit_disabled = Signal::derive(move || {
        disabled.get() || error.get().is_some() || amt.get().is_empty()
    });
    let dispatch_action = move || {
        if !submit_disabled.get() {
            convert_action.dispatch((amt.get(), input_type.get()));
        }
    };

    create_effect(move |_| {
        if !amt.get().is_empty() {
            match amt.get().parse::<f64>() {
                Err(e) => set_error.update(|v| *v = Some(format!("Unable to parse {} as a float: {:?}", amt.get(), e))),
                _ => set_error.update(|v| *v = None)
            }
        }
    });

    view! {
        <section class="items-center justify-center bg-white py-12 px-4 sm:px-6 lg:px-8 hero min-h-screen flex">
            <div class="items-center flex flex-col w-1/3">
                <p class="text-center text-5xl leading-9 font-extrabold text-gray-900">How much did I lift?</p>
                <form on:submit=|ev| ev.prevent_default() class="mt-8 w-full max-w-md space-y-6">
                    <div>
                        <div class="items-center flex">
                            <input
                                type="number"
                                class="focus:border-indigo-700 focus:outline-none
                                    focus:shadow-outline flex-grow transition duration-200 appearance-none p-2 border-2 border-gray-300
                                    text-black bg-gray-100 font-normal w-full h-12 text-xl rounded-md shadow-sm"
                                placeholder="Amount lifted"
                                prop:disabled=move|| disabled.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    match &*ev.key() {
                                        "Enter" => dispatch_action(),
                                        _ => {
                                            let val = event_target_value(&ev);
                                            set_amt.update(|v| *v = val);
                                        }
                                    }

                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_amt.update(|v| *v = val);
                                }
                                prop:disabled=move|| disabled.get()
                            />
                            <select
                                class="focus:border-indigo-700 focus:outline-none
                                    focus:shadow-outline flex-grow transition duration-200 appearance-none p-2 border-2 border-gray-300
                                    text-black bg-gray-100 font-normal w-1/6 h-12 text-xl rounded-md shadow-sm"
                                prop:disabled=move|| disabled.get() on:change= move|ev| {
                                    log::debug!("Changing! {:?}", event_target_value(&ev));
                                    set_input_type.update(|v| *v = event_target_value(&ev))
                            }>
                                <option value="Lbs" selected>"lbs"</option>
                                <option value="Kgs">"kgs"</option>
                            </select>
                            <button
                                class="btn btn-primary btn-lg flex p-2 hover:text-blue-400 bg-white w-auto justify-end
                                    items-center text-blue-500"
                                prop:disabled=submit_disabled
                                on:click=move|_| dispatch_action()
                            >"Convert"</button>
                        </div>
                        <div class="overflow-y-auto h-48">
                            <div class="overflow-auto">
                                <ul class="mt-2 w-full bg-white text-black rounded-md shadow-lg z-10">
                                    <Transition fallback=move|| view!{}>
                                        <For
                                            each=move|| responses.get()
                                            key= |state| state.when.clone()
                                            let:child
                                        >
                                            <li class="px-3 py-1 border-gray-200 hover:bg-indigo-500 hover:text-white tx-lg">{child.input_amt.to_string()} " " {child.input_type.to_string().to_lowercase()} " is " {child.output_weight}</li>
                                        </For>
                                    </Transition>
                                </ul>
                            </div>
                        </div>
                    </div>
                </form>
            </div>
        </section>
    }
}
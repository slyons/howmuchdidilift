use crate::{
    api::UnauthorizedApi
};
use interface::*;
use leptos::*;
use std::str::FromStr;
use leptos_animated_for::AnimatedFor;

#[component]
pub fn Convert(api: UnauthorizedApi, show_links: RwSignal<bool>) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    let (amt, set_amt) = create_signal(String::new());
    let (input_type, set_input_type) = create_signal("Lbs".to_string());
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let responses = create_rw_signal(vec![]);

    let responses_idx = Signal::derive(move || {
        responses.get().into_iter().enumerate()//.collect::<Vec<(usize, RandomWeightResponse)>>()
    });

    //let previous_responses = create_slice(responses, )

    let click_count = create_rw_signal(0);
    create_effect(move |_| {
        if click_count.get() >= 6 {
            show_links.update(|s| *s = !(*s));
            click_count.set(0);
        }
    });

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
                    responses.update(|resps| {
                        resps.insert(0, (6 as usize, res));
                        while resps.len() > 5 {
                            resps.pop();
                        }
                        resps.iter_mut().for_each(|entry| {
                            entry.0 -= 1;
                        });
                    });
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
        <section class="items-center justify-center py-12 px-4 sm:px-6 lg:px-8 hero min-h-screen flex">
            <div class="items-center flex flex-col w-full">
                <button class="btn no-animation text-center text-4xl lg:text-5xl leading-9 font-extrabold bg-base-100 hover:bg-base-100 outline-none" on:click=move |_| {
                    click_count.set(click_count.get() + 1);
                }>How much did I lift?</button>
                <form on:submit=|ev| ev.prevent_default() class="mt-8 w-full space-y-6">
                    <div>
                        <div class="items-center flex w-full md:w-2/3 mx-auto flex-col md:flex-row">
                            <input
                                type="number"
                                class="focus:border-indigo-700 focus:outline-none
                                    focus:shadow-outline flex-grow transition duration-200 appearance-none p-2 border-2 border-gray-300
                                    text-black bg-gray-100 font-normal w-full h-14 text-xl rounded-md shadow-sm"
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
                                    focus:shadow-outline flex-grow transition duration-200 appearance-none p-2 m-2 h-14 border-2
                                    font-normal w-1/6 min-w-14 max-w-14 h-12 text-xl rounded-md shadow-sm"
                                prop:disabled=move|| disabled.get() on:change= move|ev| {
                                    log::debug!("Changing! {:?}", event_target_value(&ev));
                                    set_input_type.update(|v| *v = event_target_value(&ev))
                            }>
                                <option value="Lbs" selected>"lbs"</option>
                                <option value="Kgs">"kgs"</option>
                            </select>
                            <button
                                class="btn btn-primary btn-lg flex p-2 h-14 w-auto justify-end
                                    items-center "
                                prop:disabled=submit_disabled
                                on:click=move|_| dispatch_action()
                            >"Convert"</button>
                        </div>
                            <div class="overflow-y-hidden h-full prose lg:prose-xl">
                                <ul class="mt-2 w-full text-center rounded-md  z-10 ">
                                    <Transition fallback=move|| view!{}>
                                        <AnimatedFor
                                            each=move|| responses.get()
                                            key= |state| (state.0, state.1.when)
                                            children=move |(idx, child)| {
                                                log::debug!("Idx for {:?} is {}", child, idx);
                                                let li_class=format!("p-3 border-gray-200 hover:text-white text-{}xl shadow shadow-slate-600", idx);
                                                view! {
                                                    <li class=li_class><code class="bg-primary">{child.input_amt.to_string()} {child.input_type.to_string().to_lowercase()}</code> " is " <code class="bg-primary">{child.output_weight.clone()}</code> " " {child.units.clone()}</li>
                                                }
                                            }
                                            enter_from_class="opacity-0"
                                            enter_class="duration-800"
                                            move_class="duration-1200"
                                            leave_class="opacity-0 duration-500"
                                            appear=true
                                        />
                                    </Transition>
                                </ul>
                            </div>
                    </div>
                </form>
            </div>
        </section>
    }
}
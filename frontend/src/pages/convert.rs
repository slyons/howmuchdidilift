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
                    set_responses.update(|resps| resps.push(res));
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
        <p>{move || error.get()}</p>
        <form on:submit=|ev| ev.prevent_default()>
            <input
                type="number"
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
            <select prop:disabled=move|| disabled.get() on:change= move|ev| {
                log::debug!("Changing! {:?}", event_target_value(&ev));
                set_input_type.update(|v| *v = event_target_value(&ev))
            }>
                <option value="Lbs" selected>"Pounds (lbs)"</option>
                <option value="Kgs">"Kilograms (kgs)"</option>
            </select>
            <Button
                variant=ButtonVariant::Primary
                disabled=submit_disabled
                on_click=move|_| dispatch_action()
            >"Convert"</Button>
        </form>
        <div>
            <For
                each=move|| responses.get()
                key= |state| state.when.clone()
                let:child
            >
            <p>"You lifted " {child.output_weight}</p>
            </For>
        </div>
    }
}
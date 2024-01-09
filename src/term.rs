use leptos::*;

fn window_size() -> i32 {
    window().inner_width().unwrap().as_f64().unwrap() as i32
}

#[component]
pub fn Term() -> impl IntoView {
    let (form, set_form) = create_signal(String::new());
    create_effect(move |_| set_form.set("a".repeat(if window_size() < 700 { 50 } else { 80 })));
    view! {
        <div id="term">
            <p>"hello world"</p>
            <form>
                <label>"cmd ~> "</label>
                <input
                    type="text"
                    on:input=move |ev| {
                        set_form(event_target_value(&ev));
                    }

                    prop:value=form
                    prop:size=move || { if window_size() < 700 { 50 } else { 80 } }
                />
            </form>

        </div>
    }
}

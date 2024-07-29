use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    let counter_increase = {
        let counter = counter.clone();
        let on_click = Callback::from(move |_| {
            counter.set(*counter + 1)
        });

        html!(<button onclick={on_click}>{"+1"}</button>)
    };

    let counter_decrease = {
        let counter = counter.clone();
        let on_click = Callback::from(move |_| {
            counter.set(*counter - 1)
        });

        html!(<button onclick={on_click}>{"-1"}</button>)
    };

    html!(
        <>
            <h1>{"Counter value: "} {*counter}</h1>
            <div> 
                {counter_increase}
                {counter_decrease}
            </div>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}

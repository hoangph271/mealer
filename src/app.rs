use leptos::*;
use rand::prelude::*;

const MEALS: [&str; 6] = [
    "Bún đậu mắm tôm",
    "Sushi",
    "Cơm rang các thể loại",
    "Phở",
    "Bún",
    "Cơm thố",
];

fn get_random_index () -> usize {
    thread_rng().gen_range(0..MEALS.len())
}

#[component]
pub fn App() -> impl IntoView {
    let (index, set_index) = create_signal::<usize>(get_random_index());

    let meal = move || MEALS.get(index.get()).unwrap().to_string();

    view! {
        <button
            type="button"
            on:click=move |_| {
                set_index.update(|index| {
                    *index = get_random_index();
                });
            }
        >
            Roll...!
        </button>
        <code>
            "// TODO: "
            {meal}
        </code>
    }
}
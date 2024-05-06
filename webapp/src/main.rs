use yew::prelude::*;
use web_time::{SystemTime};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let pairs = use_state(|| vec![]);
    let onclick = {
        let pairs = pairs.clone();
        move |_| {
            let instant = SystemTime::now();
            let mut pairs_vec: Vec<PunchPair> = pairs.to_vec();
            let last = pairs_vec.pop();
            match last {
                Some(PunchPair::Open(start)) => pairs_vec.push(PunchPair::Closed(start, instant)),
                Some(closed) => {
                    pairs_vec.push(closed);
                    pairs_vec.push(instant.into());
                }
                None => pairs_vec.push(instant.into()),
            }
            pairs.set(pairs_vec);
        }
    };

    let punches_html = pairs.iter()
        .map(|pair| html! { <PunchComponent pair={pair.clone()} /> })
        .collect::<Vec<Html>>();

    html! {
        <div class={"sheet"}>
            {punches_html}
            <button {onclick}>{ "punch" }</button>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PunchPair {
    Open(SystemTime),
    Closed(SystemTime, SystemTime),
}

impl From<SystemTime> for PunchPair {
    fn from(value: SystemTime) -> Self {
        PunchPair::Open(value)
    }
}

#[derive(PartialEq, Properties)]
struct PunchProps {
    pair: PunchPair,
}

#[function_component]
fn PunchComponent(props: &PunchProps) -> Html {
    let PunchProps { pair } = props;
    match pair {
        PunchPair::Open(start) => html! {
            <>
                <div>
                    <DisplaySystemTime system_time={start.clone()} />
                </div>
                <div></div>
            </>
        },
        PunchPair::Closed(start, end) => html! {
            <>
                <div>
                    <DisplaySystemTime system_time={start.clone()} />
                </div>
                <div>
                    <DisplaySystemTime system_time={end.clone()} />
                </div>
            </>
        },
    }
}

#[derive(PartialEq, Properties)]
struct DisplaySystemTimeProps {
    system_time: SystemTime,
}

#[function_component]
fn DisplaySystemTime(props: &DisplaySystemTimeProps) -> Html {
    let DisplaySystemTimeProps { system_time } = props;
    html! { format!("{system_time:?}") }
}
use gloo::console;
use math_library::{self, Matrix, matrix::PosElem};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    fn input_process();
}

enum CounterEvent {
    Increment,
    Decrement,
}

#[derive(PartialEq, Properties)]
pub struct CounterProps {
    name: String,
    callback: Callback<CounterEvent>,
}

#[function_component]
pub fn Counter(props: &CounterProps) -> Html {
    let CounterProps { name, callback } = props;
    let on_increment = {
        let callback = callback.clone();
        Callback::from(move |_| callback.emit(CounterEvent::Increment))
    };
    let on_decrement = {
        let callback = callback.clone();
        Callback::from(move |_| callback.emit(CounterEvent::Decrement))
    };
    html! {
        <div class="counter">
            { name }
            <button onclick={on_increment} class="increment"></button>
            <button onclick={on_decrement} class="decrement"></button>
        </div>
    }
}
pub struct App {
    source: Matrix,
    target: Matrix,
}
#[derive(Debug)]
pub enum AppMsg {
    Edited((usize, usize), f64),
    Resize(usize, usize),
    Evaluate,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        gloo::console::log!("Created");
        
        Self {
            source: Matrix::new(PosElem::new(2, 2),0f64),
            target: Matrix::new(PosElem::new(2,2), 0f64),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Edited((row,col),value) => {
            self.source[PosElem::new(row,col)] = value;
            true
        }
        AppMsg::Resize(row, col) => {
            gloo::console::log!(format!("{:?}",self.source.count));
            self.source.change_size(PosElem::new(row,col));
            gloo::console::log!(format!("{:?}",self.source.count));
            true
        },
        AppMsg::Evaluate => { 
                self.target = self.source.clone();
                self.target.simplex(0);
                true
            },
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        input_process();
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let size = self.source.count;
        let row_counter = ctx.link().callback(move |ev| {
            gloo::console::log!("new size: {}", size.row,size.col);
            match ev{
                CounterEvent::Increment => AppMsg::Resize(size.row+1, size.col),
                CounterEvent::Decrement => AppMsg::Resize(size.row-1, size.col),
            }
        });
        let col_counter = ctx.link().callback(move |ev| {
            match ev{
                CounterEvent::Increment => AppMsg::Resize(size.row, size.col+1),
                CounterEvent::Decrement => AppMsg::Resize(size.row, size.col-1),
            }
        });

        html! {
            <div>
            <img src="https://lastfm.freetls.fastly.net/i/u/avatar170s/83338d787b4c49bdc996b9b6644fd8d7.png"/>
            <img src="data:image/jpeg;base64,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" id="scrimer"/>

            <div class="calculator">
            <section class="source">
            {self.view_table(self.source, true, ctx.link())}
            </section>
            <section class="target">
            {self.view_table(self.target, false, ctx.link())}
            </section>
            </div>
            <div class="toolbar">
            <section>
            <span>{"Change size"}</span>
            <div class="counters">
            <Counter name="Row:" callback={row_counter}/>
            <Counter name="Col:" callback={col_counter}/>
            </div>
            </section>
            <button class="action" id="play_audio" onclick={ctx.link().callback(|_| AppMsg::Evaluate)}>{"Evaluate linear program"}</button>
            </div>
            </div>
        }
    }
}


impl App {
    fn view_table(&self, matrix: Matrix, editable: bool, link: &Scope<Self>) -> Html {
        let content = (0..=matrix.count.row).into_iter().map(|r| {
        let row = (0..=matrix.count.col).into_iter().map(|c| {
            let oninput = {
                let c = c.clone();
                let r= r.clone();
                link.callback(move |e:InputEvent|{ 
                let value = e.target_unchecked_into::<HtmlInputElement>().value().parse::<f64>().unwrap_or_default();
                AppMsg::Edited((r,c),value)
            })};
            html! {
                <td> if editable {<input type="number" placeholder="0" id={format!("{r}-{c}")} {oninput} value={matrix[PosElem::new(r,c)].to_string()}/>} else {{matrix[PosElem::new(r,c)]}}</td>
            }
        }).collect::<Html>();
        html!(
            <tr>{row}</tr>
        )
    }).collect::<Html>();
        html! {
            <div>
                <table>
                {content}
                </table>
                
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

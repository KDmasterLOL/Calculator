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
    Resize((isize, isize)),
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
        AppMsg::Resize(offset) => {
            self.source.offset_size(offset);
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
                CounterEvent::Increment => AppMsg::Resize((1,0)),
                CounterEvent::Decrement => AppMsg::Resize((-1,0)),
            }
        });
        let col_counter = ctx.link().callback(move |ev| {
            match ev{
                CounterEvent::Increment => AppMsg::Resize((0,1)),
                CounterEvent::Decrement => AppMsg::Resize((0,-1)),
            }
        });

        html! {
            <div>
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
            <button class="action" onclick={ctx.link().callback(|_| AppMsg::Evaluate)}>{"Evaluate linear program"}</button>
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
                    let mut input =  e.target_unchecked_into::<HtmlInputElement>().value();
                    let chars = input.as_bytes();
                    match chars.last().unwrap_or(&0u8) {
                        b'-' => {
                            if chars[0]== b'-'{
                                input.remove(0);
                            }
                            else{
                                input.insert(0, '-');
                            }
                            input.pop();
                            }
                        b'+' => {input.pop();},
                        _ => (),
                    };
                gloo::console::log!(input.clone());
                let value = input.parse::<f64>().unwrap_or_default();
                AppMsg::Edited((r,c),value)
            })};
            html! {
                <td> if editable {<input placeholder="0" id={format!("{r}-{c}")} {oninput} value={matrix[PosElem::new(r,c)].to_string()}/>} else {{matrix[PosElem::new(r,c)]}}</td>
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

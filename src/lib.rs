#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;


mod is_in_triangle;
use stdweb::web::Date;
use stdweb::Value;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    value: f32,
}

pub enum Msg {
    Smart,
    Dumb,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Smart => {
                self.console.log("go!");
                let my_test_stuff = is_in_triangle::TestStuff::new();
                js! {
                    console.log(performance.now());
                }
                for n in 0..10000000  {
                    let a = is_in_triangle::triangle_test(
                        my_test_stuff.px[n],
                        my_test_stuff.py[n],

                        my_test_stuff.ax[n],
                        my_test_stuff.ay[n],


                        my_test_stuff.bx[n],
                        my_test_stuff.by[n],


                        my_test_stuff.cx[n],
                        my_test_stuff.cy[n],
                    );
                }
                js! {
                    console.log(performance.now());
                }
            }
            Msg::Dumb => {
                self.console.log("go dumb!");
                let my_test_stuff = is_in_triangle::TestStuff::new();
                js! {
                    console.log(performance.now());
                }
                for n in 0..1000  {
                    is_in_triangle::dumb_test(
                        my_test_stuff.px[n],
                        my_test_stuff.py[n],

                        my_test_stuff.ax[n],
                        my_test_stuff.ay[n],


                        my_test_stuff.bx[n],
                        my_test_stuff.by[n],


                        my_test_stuff.cx[n],
                        my_test_stuff.cy[n],
                    )
                }
                js! {
                    console.log(performance.now());
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("container"),>
              <h1>{"Speed Test!!!"}</h1>
                <nav class="menu",>
                    <button onclick=|_| Msg::Smart,>{ "Smart" }</button>
                    <button onclick=|_| Msg::Dumb,>{ "Dumb" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}
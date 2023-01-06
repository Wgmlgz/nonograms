mod app;
mod nonogram;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

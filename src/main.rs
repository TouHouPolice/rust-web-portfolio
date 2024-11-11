mod app; use app::App;

fn main() {
    println!("Hello, zzzworlsd!");
    yew::Renderer::<App>::new().render();
}

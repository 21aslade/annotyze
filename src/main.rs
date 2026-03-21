use leptos::mount::mount_to_body;

mod app;
use app::App;

fn main() {
    mount_to_body(App);
}

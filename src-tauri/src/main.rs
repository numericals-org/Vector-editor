mod renderer;

use renderer::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    renderer.initialize();

    if renderer.is_ready() {
        println!("Renderer ready.");
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
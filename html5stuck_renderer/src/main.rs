use html5stuck_renderer::batch_render;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let out_path = std::env::args().nth(2).unwrap();
    batch_render(path, out_path).unwrap();
}

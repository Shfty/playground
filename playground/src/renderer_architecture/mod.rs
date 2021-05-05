mod example_render_pass_a;
mod example_render_pass_b;
mod example_renderer;
mod render_data;
mod render_pass;
mod renderer;

pub use example_render_pass_a::*;
pub use example_render_pass_b::*;
pub use example_renderer::*;
pub use render_data::*;
pub use render_pass::*;
pub use renderer::*;

// Functions
pub fn main() {
    let mut renderer = ExampleRenderer::default();
    renderer.push_render_data(ExampleRenderDataA::new("One"));
    renderer.push_render_data(ExampleRenderDataA::new("Two"));
    renderer.push_render_data(ExampleRenderDataB::new("Three"));
    renderer.push_render_data(ExampleRenderDataB::new("Four"));
    renderer.push_render_data(ExampleRenderDataA::new("Five"));
    renderer.push_render_data(ExampleRenderDataA::new("Six"));
    renderer.render();
}

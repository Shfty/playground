use super::{RenderData, RenderDataPass, RenderPass};

#[derive(Debug, Default)]
pub struct ExampleRenderPassA {}

impl RenderPass for ExampleRenderPassA {
    fn render(&self, data: &dyn RenderData) {
        let data = data.downcast_ref::<ExampleRenderDataA>().unwrap();
        println!("Example Render Pass A. Data: {:?}", data);
    }
}

#[derive(Debug, Default)]
pub struct ExampleRenderDataA {
    name: &'static str,
}

impl ExampleRenderDataA {
    pub fn new(name: &'static str) -> Self {
        ExampleRenderDataA { name }
    }
}

impl RenderData for ExampleRenderDataA {}

impl RenderDataPass for ExampleRenderDataA {
    type Pass = ExampleRenderPassA;
}

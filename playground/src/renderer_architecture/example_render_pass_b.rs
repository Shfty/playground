use super::{RenderData, RenderDataPass, RenderPass};

#[derive(Debug, Default)]
pub struct ExampleRenderPassB {}

impl RenderPass for ExampleRenderPassB {
    fn render(&self, data: &dyn RenderData) {
        let data = data.downcast_ref::<ExampleRenderDataB>().unwrap();
        println!("Example Render Pass B. Data: {:?}", data);
    }
}

#[derive(Debug, Default)]
pub struct ExampleRenderDataB {
    name: &'static str,
}

impl ExampleRenderDataB {
    pub fn new(name: &'static str) -> Self {
        ExampleRenderDataB { name }
    }
}

impl RenderData for ExampleRenderDataB {}

impl RenderDataPass for ExampleRenderDataB {
    type Pass = ExampleRenderPassB;
}

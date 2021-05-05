use super::RenderData;

pub trait RenderPass {
    fn render(&self, data: &dyn RenderData);
}

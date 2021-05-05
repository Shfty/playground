use std::{any::TypeId, collections::HashMap};

use super::{RenderData, RenderDataPass, RenderPass, Renderer};

#[derive(Default)]
pub struct ExampleRenderer {
    data: Vec<(TypeId, Box<dyn RenderData>)>,
    passes: HashMap<TypeId, Box<dyn RenderPass>>,
}

impl ExampleRenderer {
    pub fn push_render_data<T>(&mut self, data: T)
    where
        T: RenderDataPass,
        <T as RenderDataPass>::Pass: Default,
    {
        let pass_type = TypeId::of::<<T as RenderDataPass>::Pass>();

        self.data.push((pass_type, Box::new(data)));

        self.passes
            .entry(pass_type)
            .or_insert_with(|| Box::new(<T as RenderDataPass>::Pass::default()));
    }
}

impl Renderer for ExampleRenderer {
    fn render(&self) {
        for (pass_type, data) in &self.data {
            self.passes[&pass_type].render(data.as_ref());
        }
    }
}

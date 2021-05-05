use downcast_rs::{impl_downcast, Downcast};
use super::RenderPass;

pub trait RenderData: Downcast {}
impl_downcast!(RenderData);

pub trait RenderDataPass: RenderData {
    type Pass: RenderPass;
}

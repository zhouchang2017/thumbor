mod photon;

use image::ImageOutputFormat;

use crate::pb::Spec;
pub use photon::Photon;

/// Engine 图片处理引擎
pub trait Engine {
    // 按照 specs进行一系列的处理
    fn apply(&mut self,specs: &[Spec]);
    // 通过 format 输出对应的字节流
    fn generate(self, format: ImageOutputFormat) ->Vec<u8>;
}

// SpecTransform：未来如果添加更多的 spec，只需要实现它即可
pub trait SpecTransform<T> {
    // 对图片使用 op 做 transform
    fn transform(&mut self,op: T);
}
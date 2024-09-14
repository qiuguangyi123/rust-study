use super::{Engine, SpecTransform};
use crate::pb::*;
use anyhow::Result;
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{
    effects, filters, multiple, native::open_image_from_bytes, transform, PhotonImage,
};
use std::convert::TryFrom;

lazy_static! {
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes!(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

// 我们目前支持 Photon engine
pub struct Photon(PhotonImage);

impl TryFrom for Photon {
    type Error = anyhow::Error;
    fn try_from(data: Bytes) -> Result {
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v), // 对于目前不认识的 spec，不做任何处理
                _ => {}
            }
        }
    }
    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

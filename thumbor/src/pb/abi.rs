/// 一个 ImageSpec 是一个有序的数组，服务器按照 spec 的顺序处理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    /// 包含一组 ImageLayer 对象的数组
    #[prost(message, repeated, tag="1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
/// 处理图片改变大小
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    /// 宽度
    #[prost(uint32, tag="1")]
    pub width: u32,
    /// 高度
    #[prost(uint32, tag="2")]
    pub height: u32,
    /// 类型
    #[prost(enumeration="resize::ResizeType", tag="3")]
    pub rtype: i32,
    /// 采样滤波器
    #[prost(enumeration="resize::SampleFilter", tag="4")]
    pub filter: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResizeType {
        /// 普通
        Normal = 0,
        /// 缝合
        SeamCarve = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleFilter {
        Undefined = 0,
        /// 最近
        Nearest = 1,
        /// 三角形
        Triangle = 2,
        /// CatmullRom
        CatmullRom = 3,
        /// 高斯
        Gaussian = 4,
        /// Lanczos
        Lanczos3 = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag="1")]
    pub x1: u32,
    #[prost(uint32, tag="2")]
    pub y1: u32,
    #[prost(uint32, tag="3")]
    pub x2: u32,
    #[prost(uint32, tag="4")]
    pub y2: u32,
}
/// 处理水平翻转
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fliph {
}
/// 处理垂直翻转
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flipv {
}
/// 处理对比度
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contrast {
    #[prost(float, tag="1")]
    pub contrast: f32,
}
/// 处理滤镜
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(enumeration="filter::Filter", tag="1")]
    pub filter: i32,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Filter {
        Unspecified = 0,
        Oceanic = 1,
        Islands = 2,
        /// more: https://docs.rs/photon-rs/0.3.1/photon_rs/filters/fn.filter.html
        Marine = 3,
    }
}
/// 处理水印
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watermark {
    #[prost(uint32, tag="1")]
    pub x: u32,
    #[prost(uint32, tag="2")]
    pub y: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof="spec::Data", tags="1, 2, 3, 4, 5, 6, 7")]
    pub data: ::core::option::Option<spec::Data>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        /// 处理图片改变大小
        #[prost(message, tag="1")]
        Resize(super::Resize),
        /// 处理裁剪
        #[prost(message, tag="2")]
        Crop(super::Crop),
        /// 处理水平翻转
        #[prost(message, tag="3")]
        Fliph(super::Fliph),
        /// 处理垂直翻转
        #[prost(message, tag="4")]
        Flipv(super::Flipv),
        /// 处理对比度
        #[prost(message, tag="5")]
        Contrast(super::Contrast),
        /// 处理滤镜
        #[prost(message, tag="6")]
        Filter(super::Filter),
        /// 处理水印
        #[prost(message, tag="7")]
        Watermark(super::Watermark),
    }
}

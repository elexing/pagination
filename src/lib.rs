//! 分页库一共包含3个分页请求模型：`PageParam`,`PageRequest` 和 `OffsetRequest`. 一般情况下, `PageRequest`
//! 和`PageParam`应该是来自于客户端或者外部的模型; 在内部，当求请求数据库时，通过`PageRequest.into_offset`将其转
//! 换成`OffsetRequest`.
//!
//! Usage ：
//! ``` rust
//! use pagination::{
//! 	PageParams,
//! 	OffsetParams,
//! 	DefaultIntoOffset
//! };
//! let page_param = PageParams::new(5, 20);
//! let offset_param = page_param.into_offset();
//! assert_eq!(offset_param.offset(), 80_u64);
//! assert_eq!(offset_param.limit(), 20);
//! ```

#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

mod composite;
mod page;
mod simple;

use cfg_if;

cfg_if::cfg_if! {
    if #[cfg(feature = "page-size-5")]{
        pub const DEFAULT_PAGE_SIZE: u32 = 5;
    } else if #[cfg(feature = "page-size-10")]{
        pub const DEFAULT_PAGE_SIZE: u32 = 10;
    } else if #[cfg(feature = "page-size-15")] {
        pub const DEFAULT_PAGE_SIZE: u32 = 15;
    } else if #[cfg(feature = "page-size-20")] {
        pub const DEFAULT_PAGE_SIZE: u32 = 20;
    } else if #[cfg(feature = "page-size-50")] {
        pub const DEFAULT_PAGE_SIZE: u32 = 50;
    } else {
        pub const DEFAULT_PAGE_SIZE: u32 = 20;
    }
}

/// 默认的每页最大条数
pub const DEFAULT_MAX_PAGE_SIZE: u32 = 100;

/// 可分页的查询
pub trait Pageable {
    /// 页码
    fn page_number(&self) -> u32;

    /// 每页的条数
    fn page_size(&self) -> u32;
}

/// **offset** base request/condition
///
/// 基于**偏移量** 的查询
pub trait Offsetable {
    /// 偏移量
    fn offset(&self) -> u64;
    /// 限定条数
    fn limit(&self) -> u32;
}

/// 由分页查询参数计算出的基于便宜量的查询。
pub trait IntoOffset {
    /// see `Offsetable`
    type Offset: Offsetable + Sized;

    /// 转换成基于偏移量的查询
    fn into_offset(self, default_page_size: u32, max_page_size: u32) -> Self::Offset;
}

/// 使用默认的页面量设定， 将分页查询参数转换成基于便宜量的查询。
/// - page size : `DEFAULT_PAGE_SIZE`
/// - max page size : `DEFAULT_MAX_PAGE_SIZE`
pub trait DefaultIntoOffset {
    /// see `Offsetable`
    type Offset: Offsetable + Sized;

    /// 转换成基于偏移量的查询
    fn into_offset(self) -> Self::Offset;
}

pub use composite::OffsetRequest;
pub use composite::PageRequest;
pub use page::Page;
pub use simple::OffsetParams;
pub use simple::PageParams;

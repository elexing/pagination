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

use std::ops::Index;
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

/// 由分页查询参数计算出的基于便宜量的查询。
pub trait IntoOffset {
    /// 转换成基于偏移量的查询
    fn into_offset(&self, default_page_size: u32, max_page_size: u32) -> OffsetParams;
}

/// 使用默认的页面量设定， 将分页查询参数转换成基于便宜量的查询。
/// - page size : `DEFAULT_PAGE_SIZE`
/// - max page size : `DEFAULT_MAX_PAGE_SIZE`
pub trait DefaultIntoOffset {
    /// 转换成基于偏移量的查询
    fn into_offset(&self) -> OffsetParams;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct OffsetParams {
    /// 偏移量
    offset: u64,
    /// 限定条数
    limit: u32,
}

impl OffsetParams {
    fn new<T: Pageable>(pageable: &T, default_page_size: u32, max_page_size: u32) -> OffsetParams {
        let page_number = match pageable.page_number() {
            0 => 1,
            x => x,
        };
        let page_size = Self::build_page_size(pageable, default_page_size, max_page_size);
        let offset: u64 = ((page_number as u64) - 1) * (page_size as u64);
        OffsetParams {
            offset,
            limit: page_size,
        }
    }

    fn build_page_size<T: Pageable>(pageable: &T, default_page_size: u32, max_page_size: u32) -> u32{
        match pageable.page_size() {
            0 => {
                if default_page_size > 0 {
                    default_page_size
                } else {
                    DEFAULT_PAGE_SIZE
                }
            }
            x if x > max_page_size => max_page_size,
            x => x,
        }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }
}

impl<T> IntoOffset for T
where
    T: Pageable,
{
    /// 获取分页查询时,由分页查询参数计算出的便宜量值.
    fn into_offset(&self, default_page_size: u32, max_page_size: u32) -> OffsetParams {
        OffsetParams::new(self, default_page_size, max_page_size)
    }
}

impl<T> DefaultIntoOffset for T
where
    T: Pageable,
{
    /// 获取分页查询时,由分页查询参数计算出的便宜量值.
    fn into_offset(&self) -> OffsetParams {
        OffsetParams::new(self, DEFAULT_PAGE_SIZE, DEFAULT_MAX_PAGE_SIZE)
    }
}

/// Page Query Condition
///
/// # Example
/// ``` rust
/// use pagination::PageRequest;
///
/// struct UserQuery {
///    pub user_id : Option<u64>,
///    pub user_name : Option<String>
/// }
///
/// PageRequest::new(10 , 20 , UserQuery{user_id : Some(10) , user_name : Some("test".to_string())});
///
/// PageRequest::new(10 , 20 , ());
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct PageRequest<T: Sized> {
    page_number: u32,
    page_size: u32,
    request: Option<T>,
}

impl<T: Sized> PageRequest<T> {
    /// # Arguments
    /// * page_number 页码
    /// * page_size 每页的条数.
    /// * request 与分页查询无关的其他查询参数。
    pub fn new<E>(page_number: u32, page_size: u32, request: E) -> PageRequest<T>
    where
        E: Into<Option<T>>,
    {
        PageRequest {
            page_number,
            page_size,
            request: request.into(),
        }
    }

    /// 与分页查询无关的其他查询参数。
    pub fn request(&self) -> Option<&T> {
        self.request.as_ref()
    }
}

impl<T: Sized> Pageable for PageRequest<T> {
    /// 页码
    fn page_number(&self) -> u32 {
        self.page_number
    }

    /// 每页的条数
    fn page_size(&self) -> u32 {
        self.page_size
    }
}

/// Page Query Param
///
/// # Example
/// ``` rust
/// use pagination::PageParams;
///
///
/// PageParams::new(10 , 20);
///
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct PageParams {
    page_number: u32,
    page_size: u32,
}

impl PageParams {
    /// # Arguments
    /// * page_number 页码
    /// * page_size 每页的条数.
    /// * request 与分页查询无关的其他查询参数。
    pub fn new(page_number: u32, page_size: u32) -> PageParams {
        PageParams {
            page_number,
            page_size,
        }
    }
}

impl Pageable for PageParams {
    /// 页码
    fn page_number(&self) -> u32 {
        self.page_number
    }

    /// 每页的条数
    fn page_size(&self) -> u32 {
        self.page_size
    }
}

/// Page Query Result Model
///
/// # Example
/// ``` rust
/// use pagination::Page;
///
/// let page = Page::new(vec!(10,20,30), 10);
/// let f1 = page[0];
/// assert_eq!(f1 , 10);
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Page<T> {
    records: Vec<T>,
    total: u64,
}

impl<T> Page<T> {
    /// # Arguments
    /// * records 当前页的记录数
    /// * total 总记录数
    pub fn new(records: Vec<T>, total: u64) -> Page<T> {
        Page { records, total }
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    /// 获取当前页记录数
    pub fn size(&self) -> u32 {
        self.records.len() as u32
    }

    pub fn records(&self) -> &Vec<T> {
        &self.records
    }
}

impl<T> IntoIterator for Page<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}

impl<T> Index<u32> for Page<T> {
    type Output = T;

    fn index(&self, index: u32) -> &T {
        &self.records[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::DefaultIntoOffset;
    use crate::Page;
    use crate::PageRequest;
    use crate::Pageable;
    #[cfg(feature = "with-serde")]
    use serde_json::to_string;

    #[test]
    pub fn test_page_request() {
        let page_request = PageRequest::new(5, 20, 10);
        assert_eq!(page_request.page_size(), 20);
        assert_eq!(page_request.page_number(), 5);
        assert_eq!(page_request.request().unwrap(), &10);

        let offset_req = page_request.into_offset();
        assert_eq!(offset_req.offset(), 80_u64);
        assert_eq!(offset_req.limit(), 20);
    }
    #[test]
    #[cfg(feature = "page-size-10")]
    pub fn test_page_size_feature() {
        let req = PageRequest::new(5, 0, 10);
        let param = req.into_offset();
        assert_eq!(param.offset, 40);
    }

    #[test]
    #[cfg(all(feature = "with-serde", feature = "page-size-10"))]
    fn test_serialized_page_condition() {
        let req = PageRequest::new(5, 20, 10);
        println!("{:?}", to_string(&req));
    }

    #[test]
    fn test_page() {
        let page = Page::new(vec![10, 20, 30, 40, 50, 60], 10);
        assert_eq!(page.size(), 6);
        assert_eq!(page.total(), 10);
        assert_eq!(page[5], 60);
        assert_eq!(page[2], 30);
    }
}

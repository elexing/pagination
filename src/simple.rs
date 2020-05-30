use crate::{Offsetable, Pageable};
use crate::{DEFAULT_MAX_PAGE_SIZE, DEFAULT_PAGE_SIZE};
use std::cmp::min;

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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct OffsetParams {
    /// 偏移量
    offset: u64,
    /// 限定条数
    limit: u32,
}

impl OffsetParams {
    pub(crate) fn new<T: Pageable>(
        pageable: &T,
        default_page_size: u32,
        max_page_size: u32,
    ) -> OffsetParams {
        let page_number = if pageable.page_number() == 0 {
            1
        } else {
            pageable.page_number()
        };
        let page_size = build_page_size(pageable, default_page_size, max_page_size);
        let offset: u64 = ((page_number as u64) - 1) * (page_size as u64);
        OffsetParams {
            offset,
            limit: page_size,
        }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }
}

#[inline(always)]
fn build_page_size<T: Pageable>(pageable: &T, default_size: u32, max_size: u32) -> u32 {
    match pageable.page_size() {
        0 if default_size > 0 => default_size,
        0 => DEFAULT_PAGE_SIZE,
        x if max_size == 0 => min(DEFAULT_MAX_PAGE_SIZE, x),
        x => min(max_size, x),
    }
}

impl Offsetable for OffsetParams {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn limit(&self) -> u32 {
        self.limit
    }
}

impl crate::IntoOffset for PageParams {
    type Offset = OffsetParams;
    /// 获取分页查询时,由分页查询参数计算出的便宜量值.
    fn into_offset(self, default_page_size: u32, max_page_size: u32) -> OffsetParams {
        OffsetParams::new(&self, default_page_size, max_page_size)
    }
}

impl crate::DefaultIntoOffset for PageParams {
    type Offset = OffsetParams;

    /// 获取分页查询时,由分页查询参数计算出的便宜量值.
    fn into_offset(self) -> OffsetParams {
        OffsetParams::new(&self, DEFAULT_PAGE_SIZE, DEFAULT_MAX_PAGE_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "with-serde")]
    use serde_json::to_string;

    #[test]
    pub fn test_build_page_size() {
        // default page size
        let page_param = PageParams::new(0, 0);
        let size = build_page_size(&page_param, 0, 0);
        assert_eq!(size, 20);

        // user page size
        let page_param = PageParams::new(0, 10);
        let size = build_page_size(&page_param, 0, 0);
        assert_eq!(size, 10);

        // default max page size
        let page_param = PageParams::new(0, 300);
        let size = build_page_size(&page_param, 0, 0);
        assert_eq!(size, 100);

        // user default page size
        let page_param = PageParams::new(0, 0);
        let size = build_page_size(&page_param, 6, 0);
        assert_eq!(size, 6);

        // user page size with user default page size
        let page_param = PageParams::new(0, 3);
        let size = build_page_size(&page_param, 6, 0);
        assert_eq!(size, 3);

        // user default page size
        let page_param = PageParams::new(0, 0);
        let size = build_page_size(&page_param, 6, 30);
        assert_eq!(size, 6);

        let page_param = PageParams::new(0, 11);
        let size = build_page_size(&page_param, 6, 30);
        assert_eq!(size, 11);

        let page_param = PageParams::new(0, 42);
        let size = build_page_size(&page_param, 6, 33);
        assert_eq!(size, 33);
    }

    #[test]
    pub fn test_new_offset_param() {
        let page_param = PageParams::new(0, 0);
        let offset_param = OffsetParams::new(&page_param, 0, 0);
        assert_eq!(offset_param.offset(), 0);
        assert_eq!(offset_param.limit(), 20);

        let page_param = PageParams::new(3, 0);
        let offset_param = OffsetParams::new(&page_param, 0, 0);
        assert_eq!(offset_param.offset(), 40);
        assert_eq!(offset_param.limit(), 20);
    }

    #[test]
    pub fn test_default_into_offset() {
        use crate::DefaultIntoOffset;
        let page_param = PageParams::new(2, 0);
        let offset_param = page_param.into_offset();
        assert_eq!(offset_param.offset(), 20);
        assert_eq!(offset_param.limit(), 20);

        let page_param = PageParams::new(3, 120);
        let offset_param = page_param.into_offset();
        assert_eq!(offset_param.offset(), 200);
        assert_eq!(offset_param.limit(), 100);
    }

    #[test]
    pub fn test_into_offset() {
        use crate::IntoOffset;
        let page_param = PageParams::new(5, 0);
        let offset_param = page_param.into_offset(7, 15);
        assert_eq!(offset_param.offset(), 28);
        assert_eq!(offset_param.limit(), 7);

        let page_param = PageParams::new(7, 20);
        let offset_param = page_param.into_offset(7, 15);
        assert_eq!(offset_param.offset(), 90);
        assert_eq!(offset_param.limit(), 15);
    }
}

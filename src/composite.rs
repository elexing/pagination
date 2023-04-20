use crate::simple::OffsetParams;
use crate::{Offsetable, Pageable, DEFAULT_MAX_PAGE_SIZE, DEFAULT_PAGE_SIZE};

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
pub struct PageRequest<T: Sized = ()> {
    page_number: u32,
    page_size: u32,
    request: Option<T>,
}

pub struct OffsetRequest<T: Sized =()> {
    offset: u64,
    limit: u32,
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

    /// 页码
    pub fn page_number(&self) -> u32 {
        self.page_number
    }

    /// 每页的条数
    pub fn page_size(&self) -> u32 {
        self.page_size
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

impl<T: Sized> OffsetRequest<T> {

    pub fn new(offset : u64, limit : u32) -> Self{
        Self{offset ,limit, request : None }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn request(&self) -> Option<&T> {
        self.request.as_ref()
    }
}

impl<T: Sized> Offsetable for OffsetRequest<T> {
    fn offset(&self) -> u64 {
        self.offset
    }

    fn limit(&self) -> u32 {
        self.limit
    }
}

impl<T> crate::IntoOffset for PageRequest<T> {
    type Offset = OffsetRequest<T>;

    fn into_offset(self, default_page_size: u32, max_page_size: u32) -> Self::Offset {
        let param = OffsetParams::new(&self, default_page_size, max_page_size);
        OffsetRequest {
            offset: param.offset(),
            limit: param.limit(),
            request: self.request,
        }
    }
}

impl<T> crate::DefaultIntoOffset for PageRequest<T> {
    type Offset = OffsetRequest<T>;

    fn into_offset(self) -> Self::Offset {
        let param = OffsetParams::new(&self, DEFAULT_PAGE_SIZE, DEFAULT_MAX_PAGE_SIZE);
        OffsetRequest {
            offset: param.offset(),
            limit: param.limit(),
            request: self.request,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PageRequest;
    #[cfg(feature = "with-serde")]
    use serde_json::to_string;

    #[test]
    pub fn test_default_into_offset() {
        use crate::DefaultIntoOffset;

        let page_request = PageRequest::new(5, 20, 10_isize);
        assert_eq!(page_request.page_size(), 20);
        assert_eq!(page_request.page_number(), 5);
        assert_eq!(page_request.request().unwrap(), &10_isize);

        let offset_req = page_request.into_offset();
        assert_eq!(offset_req.offset(), 80_u64);
        assert_eq!(offset_req.limit(), 20);
        assert_eq!(offset_req.request().unwrap(), &10_isize);
    }

    #[test]
    pub fn test_into_offset() {
        use crate::IntoOffset;

        let page_request = PageRequest::new(5, 20, 10_isize);
        assert_eq!(page_request.page_size(), 20);
        assert_eq!(page_request.page_number(), 5);
        assert_eq!(page_request.request().unwrap(), &10_isize);

        let offset_req = page_request.into_offset(0, 40);
        assert_eq!(offset_req.offset(), 80_u64);
        assert_eq!(offset_req.limit(), 20);
        assert_eq!(offset_req.request().unwrap(), &10_isize);
    }
}

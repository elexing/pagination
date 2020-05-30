use std::ops::Index;

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
    use super::Page;
    #[cfg(feature = "with-serde")]
    use serde_json::to_string;

    #[test]
    fn test_page() {
        let page = Page::new(vec![10, 20, 30, 40, 50, 60], 10);
        assert_eq!(page.size(), 6);
        assert_eq!(page.total(), 10);
        assert_eq!(page[5], 60);
        assert_eq!(page[2], 30);
    }

    #[test]
    #[cfg(feature = "with-serde")]
    fn test_serial_to_json(){
        let page = Page::new(vec![10, 20, 30, 40, 50, 60], 10);
        let json = to_string(&page);
        assert!(json.is_ok());
    }
}
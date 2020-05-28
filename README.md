# what

`Pagination` is a page query lib for rust.

# Usage 

``` rust
 use pagination::{
 	PageParams,
 	OffsetParams,
 	DefaultIntoOffset
 };
 let page_param = PageParams::new(5, 20);
 let offset_param = page_param.into_offset();
 assert_eq!(offset_param.offset(), 80_u64);
 assert_eq!(offset_param.limit(), 20);
```

# License

Licensed under either of these:
- MIT([https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)) 
- Apache-2.0([https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)) 
# What

`Pagination` is a page query lib for rust.

# Usage 

## Build pageable request param

### Only pagination params

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

### Support complex params

``` rust
use pagination::{
 	PageRequest,
 	OffsetRequest,
 	DefaultIntoOffset
 };

 struct UserQuery {
    name : &'static str,
    age : u8
 }
 let page_req = PageRequest::new(5, 20, UserQuery{name : "alex", "age" : 18});
 let offset_req = page_req.into_offset();
 assert_eq!(offset_param.offset(), 80_u64);
 assert_eq!(offset_param.limit(), 20);
```

### Default page size
  If the **page size** from user is `0` or greater than `max page size`, the `DEFAULT_PAGE_SIZE` will be the page size.
The value of `DEFAULT_PAGE_SIZE` is `20`, and you can choose the value by the features, `page-size-5`,`page-size-10`,
`page-size-15`, `page-size-20`, `page-size-30`,`page-size-40`, `page-size-50`.


## Wrap the database result

``` rust

use pagination::Page;

let page = Page::new(vec!(10,20,30), 10);
let f1 = page[0];
assert_eq!(f1 , 10);

```

# License

Licensed under either of these:
- MIT([https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT)) 
- Apache-2.0([https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)) 
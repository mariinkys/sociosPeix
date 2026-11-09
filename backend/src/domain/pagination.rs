/// A zero-based page request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    pub page: u32,
    pub size: u32,
}

impl PageRequest {
    pub const MAX_SIZE: u32 = 200;

    /// Clamps `size` into `1..=MAX_SIZE` so a client can't ask for an unbounded page.
    pub fn new(page: u32, size: u32) -> Self {
        Self {
            page,
            size: size.clamp(1, Self::MAX_SIZE),
        }
    }

    pub fn limit(&self) -> i64 {
        i64::from(self.size)
    }

    pub fn offset(&self) -> i64 {
        i64::from(self.page) * i64::from(self.size)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl SortDirection {
    /// Anything other than "asc" (case-insensitive) is descending, as before.
    pub fn parse(s: &str) -> Self {
        if s.eq_ignore_ascii_case("asc") {
            Self::Asc
        } else {
            Self::Desc
        }
    }

    pub fn as_sql(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

/// One page of results plus the total number of matching items.
#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub request: PageRequest,
    pub total: i64,
}

impl<T> Page<T> {
    pub fn total_pages(&self) -> i64 {
        let size = i64::from(self.request.size);
        (self.total + size - 1) / size
    }

    pub fn is_last(&self) -> bool {
        i64::from(self.request.page) + 1 >= self.total_pages()
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        Page {
            items: self.items.into_iter().map(f).collect(),
            request: self.request,
            total: self.total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_maths() {
        let p = Page {
            items: vec![1, 2],
            request: PageRequest::new(0, 10),
            total: 21,
        };
        assert_eq!(p.total_pages(), 3);
        assert!(!p.is_last());
        let p = Page {
            items: vec![1],
            request: PageRequest::new(2, 10),
            total: 21,
        };
        assert!(p.is_last());
        let empty: Page<i32> = Page {
            items: vec![],
            request: PageRequest::new(0, 10),
            total: 0,
        };
        assert_eq!(empty.total_pages(), 0);
        assert!(empty.is_last());
        assert_eq!(PageRequest::new(3, 0).size, 1);
        assert_eq!(PageRequest::new(3, 10).offset(), 30);
    }
}

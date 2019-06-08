use std::str;
use std::rc::Rc;
use std::slice;
use std::ops::Deref;

#[derive(Clone)]
pub struct RcStr {
    ptr: *const u8,
    len: usize,
    inner: Rc<str>,
}

impl RcStr {
    pub fn new(s: impl Into<String>) -> Self {
        RcStr::from(s.into())
    }

    pub fn from_slice(owner: &Self, s: &str) -> Option<Self> {
        owner.sliced(s)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.as_bytes())
        }
    }

    pub fn trim(&self) -> Self {
        self.sliced(self.as_str().trim()).unwrap()
    }

    pub fn trim_start(&self) -> Self {
        self.sliced(self.as_str().trim_start()).unwrap()
    }

    pub fn trim_end(&self) -> Self {
        self.sliced(self.as_str().trim_end()).unwrap()
    }

    pub fn sliced(&self, s: &str) -> Option<Self> {
        let start_ptr = self.inner.as_ptr();
        let end_ptr = self.inner[self.inner.len()..].as_ptr();
        let ptr = s.as_ptr();

        if ptr < start_ptr || end_ptr < ptr {
            return None;
        }

        Some(Self {
            ptr,
            len: s.len(),
            inner: self.inner.clone(),
        })
    }

    pub fn slice_with<F>(&self, f: F) -> Option<Self>
    where
        F: FnOnce(&str) -> &str
    {
        self.sliced(f(self.as_str()))
    }
}

impl From<&'_ str> for RcStr {
    fn from(s: &'_ str) -> Self {
        Self::from(Rc::<str>::from(s))
    }
}

impl From<String> for RcStr {
    fn from(s: String) -> Self {
        Self::from(Rc::<str>::from(s))
    }
}

impl From<Rc<str>> for RcStr {
    fn from(inner: Rc<str>) -> Self {
        Self {
            ptr: inner.as_ptr(),
            len: inner.len(),
            inner,
        }
    }
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::RcStr;

    const STR: &str = "hello world";

    #[test]
    fn sliced() {
        let rcs = RcStr::new(STR);
        let str = rcs.as_str();

        assert_eq!(rcs.sliced(&str[0..]).unwrap().as_str(), &STR[0..]);
        assert_eq!(rcs.sliced(&str[..str.len()]).unwrap().as_str(), &STR[..STR.len()]);
        assert_eq!(rcs.sliced(&str[3..6]).unwrap().as_str(), &STR[3..6]);
        assert!(rcs.sliced("foo").is_none());
    }

    #[test]
    fn len() {
        let rcs = RcStr::new(STR);
        assert_eq!(rcs.len(), STR.len());
    }

    #[test]
    fn as_str() {
        let rcs = RcStr::new(STR);
        assert_eq!(rcs.as_str(), STR);
    }
}

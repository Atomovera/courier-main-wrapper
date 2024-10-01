use crate::utils;

#[derive(Debug,Clone, Copy)]
pub struct Arg{
    argc:usize,
    argv:*mut *mut u8
}

impl Arg {
    pub unsafe fn new(argc:isize,argv:*mut *mut u8)->Result<Arg,core::str::Utf8Error>{
        assert!(argc > 0);
        assert!(!argv.is_null());
        let this = Arg {
            argc:argc as usize,
            argv
        };
        let slice = utils::ptr_to_slice(this.argv, this.argc);
        for idx in 0..slice.len(){
            let result = utils::c_string_to_rust(*slice.get(idx).unwrap());
            if let Err(e) = result {
                return Err(e);
            }
        }
        Ok(this)
    }

    pub unsafe fn get_str(&self,idx:usize)->&'static str{
        let cache = core::slice::from_raw_parts_mut(self.argv, self.argc).get_unchecked(idx);
        utils::c_string_to_rust(*cache).unwrap()
    }

    pub fn get_raw_data(&self)->(*mut *mut u8,usize){
        (self.argv,self.argc)
    }
}

impl<'a> IntoIterator for &'a Arg{
    type Item = &'a str;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            data: self,
            index:0
        }
    }
}

pub struct Iter<'a>{
    data:&'a Arg,
    index:usize
}

impl<'a> Iterator for Iter<'a>{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.argc {
            return None;
        } else {
            let cache = unsafe {
                self.data.get_str(self.index)
            };
            self.index += 1;
            Some(cache)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.data.argc - self.index;
        (count,Some(count))
    }

    fn count(self) -> usize{
        self.data.argc
    }
}
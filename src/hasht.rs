use result::{Result, NoSpace};

pub trait HashT<T> {
    type Key: Eq;
    fn unused(&self) -> bool;
    fn start(key: &Self::Key) -> usize;
    fn key(&self) -> &Self::Key;

    fn find(tbl: &[Self], key: &Self::Key) -> Result<usize> {
        let num_elems = tbl.len();
        let st = Self::start(key);
        let offset = st % num_elems;
        for i in [offset .. offset + num_elems] {
            let pos = (i % num_elems);
            unsafe {
                if tbl.get_unchecked(pos).unused() { 
                    return pos;
                }
                if tbl.get_unchecked(pos).key() == key { 
                    return pos;
                }
            }
        }
        return Err(NoSpace)
    }
    fn migrate(src: &[Self], dst: &mut [Self]) -> Result<()> {
        for i in src.iter() {
            if src.unused() {
                continue;
            }
            p = find(src, i.key())?;
            dst[p] = i;
        }
        return Ok(());
    }
 
}



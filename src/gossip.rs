use data;
use hasht::{HashT, Key, Val};
use result::{Result};

#[derive(Default, Copy, Clone)]
#[repr(C)]
struct Subscriber {
    key: [u8; 32],
    addr: u64,
    lastping: u64,
}

impl Val<[u8;32]> for Subscriber {
    fn key(&self) -> &[u8;32] {
        return &self.key;
    }
}

type SubT = HashT<[u8;32], Subscriber>;

pub struct Gossip {
    subs: Vec<Subscriber>,
    now: u64,
    used: usize,
}

impl Gossip {
    pub fn new(size: usize) -> Gossip {
        let mut v = Vec::new();
        v.clear();
        v.resize(size, Subscriber::default());
        return Gossip{subs: v, now: 0, used: 0};
    }
    fn double(&mut self) -> Result<()> {
        let mut v = Vec::new();
        let size = self.subs.len()*2;
        v.resize(size, Subscriber::default());
        SubT::migrate(&self.subs, &mut v)?;
        self.subs = v;
        return Ok(());
    }
    unsafe fn exec(&mut self,
                   m: &data::Message,
                   new: &mut usize) -> Result<()> {
        match m.kind {
            data::Kind::Signature => self.now = m.data.poh.counter,
            data::Kind::Subscribe => {
                let pos = SubT::find(&self.subs,
                                     &m.data.sub.key)?;
                let now = self.now;
                let update = Subscriber{key: m.data.sub.key,
                                        addr: m.data.sub.addr,
                                        lastping: now};
                let g = self.subs.get_unchecked_mut(pos);
                if g.key.unused() {
                    *new = *new + 1;
                }
                *g = update;
            }
            _ => return Ok(()),
        }
        return Ok(());
    }
    pub fn execute(&mut self, msgs: &mut [data::Message]) -> Result<()> {
        for m in msgs.iter() {
            let mut new = 0;
            unsafe {
                self.exec(&m, &mut new)?;
            }
            self.used = self.used + new;
            if ((4*(self.used))/3) > self.subs.len() {
                self.double()?;
            }
        }
        return Ok(());
    }
    //downstream broadcast algorithm
    //everyone lower rank
    //      l
    //   s s s s 
    // ss ss ss ss
    // so basically arange a heap based on "rank" and 
    // broadcast down the heap based on the width of the heap
    // rank is based on bond size
    pub fn downstream(&mut self) -> Result<()> {
        return Ok(());
    }

}

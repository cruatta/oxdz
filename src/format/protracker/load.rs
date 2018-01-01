use std::cmp::max;
use Error;
use format::ModuleFormat;
use module::{Module, Sample, Instrument, Orders, Patterns};
use util::{BinaryRead, period_to_note};

pub struct Mod {
    name: &'static str,
}

impl Mod {
    pub fn new() -> Self {
        Mod{name: "Protracker MOD"}
    }

    fn load_instrument(&self, b: &[u8], mut m: Module, i: usize) -> Result<Module, Error> {
        let mut ins = Instrument::new();
        let mut smp = Sample::new();

        let mut ofs = 20 + i * 30;
        ins.num = i + 1;
        smp.num = i + 1;
        ins.name = b.read_string(ofs, 22)?;
        smp.name = ins.name.to_owned();

        smp.size = b.read16b(ofs + 22)? as u32 * 2;
        smp.rate = 8287.0;
        ins.volume = b.read8(ofs + 25)? as usize;
        smp.loop_start = b.read16b(ofs + 26)? as u32 * 2;
        let loop_size = b.read16b(ofs + 28)?;
        smp.loop_end = smp.loop_start + loop_size as u32 * 2;
        smp.has_loop = loop_size > 1 && smp.loop_end >= 4;

        m.instrument.push(ins);
        m.sample.push(smp);

        Ok(m)
    }
}

impl ModuleFormat for Mod {
    fn name(&self) -> &'static str {
        self.name
    }
  
    fn probe(&self, b: &[u8]) -> Result<(), Error> {
        if b.len() < 1084 {
            return Err(Error::Format("file too short"));
        }

        if b.read32b(1080)? == 0x4d2e4b2e {
            Ok(())
        } else {
            Err(Error::Format("bad magic"))
        }
    }

    fn load(self: Box<Self>, b: &[u8]) -> Result<Module, Error> {
        let mut m = Module::new();
        m.title = b.read_string(0, 20)?;
        m.chn = 4;

        for i in 0..31 {
            m = try!(self.load_instrument(b, m, i));
        }

        let len = b.read8(950)? as usize;
        let rst = b.read8(951)?;
        let ord = ModOrders::from_slice(rst, b.slice(952, len)?);
        let pat = ord.patterns();
        m.orders = Box::new(ord);

        let p = ModPatterns::from_slice(pat, b.slice(1084, 1024*pat)?)?;
        m.patterns = Box::new(p);

        Ok(m)
    }

}

struct ModEvent {
    note: u8,
    ins : u8,
    fxt : u8,
    fxp : u8,
}

impl ModEvent {
    fn from_slice(b: &[u8]) -> Self {
        ModEvent {
            note: period_to_note((((b[0] & 0x0f) as u32) << 8) | b[1] as u32) as u8,
            ins : (b[0] & 0xf0) | ((b[2] & 0xf0) >> 4),
            fxt : b[2] & 0x0f,
            fxp : b[3],
        }
    }
}

struct ModPatterns {
    num : usize,
    data: Vec<ModEvent>,
}

impl ModPatterns {
    fn from_slice(num: usize, b: &[u8]) -> Result<Self, Error> {
        let mut pat = ModPatterns{
            num,
            data: Vec::new(),
        };

        for p in 0..num {
            for r in 0..64 {
                for c in 0..4 {
                    let ofs = p * 1024 + r * 16 + c * 4;
                    let e = ModEvent::from_slice(b.slice(ofs, 4)?);
                    pat.data.push(e);
                }
            }
        }

        Ok(pat)
    }
}

impl Patterns for ModPatterns {
    fn num(&self) -> usize {
        self.num 
    }

    fn rows(&self, pat: usize) -> usize{
        if pat >= self.num() {
            0
        } else {
            64
        }
    }
}


struct ModOrders {
    song  : usize,
    pos   : usize,
    rstpos: usize,
    orders: Vec<u8>,
}

impl ModOrders {
    fn from_slice(r: u8, o: &[u8]) -> Self {
        
        let mut r = r as usize;

        if r >= o.len() {
            r = 0;
        }

        ModOrders {
            song  : 0,
            pos   : 0,
            rstpos: r,
            orders: o.to_vec(),
        }
    }

    fn patterns(&self) -> usize {
        let mut num = 0;
        self.orders.iter().for_each(|x| num = max(*x as usize, num));
        num 
    }
}

impl Orders for ModOrders {
    fn num(&self) -> usize {
        self.orders.len()
    }

    fn restart(&mut self) -> usize {
        let p = self.rstpos;
        self.set(p)
    }

    fn current(&self) -> usize {
        self.pos
    }

    fn pattern(&self) -> usize {
        let p = self.pos;
        self.orders[p] as usize
    }

    fn set(&mut self, p: usize) -> usize {
        self.pos = p;
        self.pos
    }

    fn next(&mut self) -> usize {
        if self.pos < self.num() - 1 {
            self.pos += 1;
        }
        self.pos
    }

    fn prev(&mut self) -> usize {
        if self.pos > 0 {
            self.pos -= 1;
        }
        self.pos
    }

    fn current_song(&self) -> usize {
        0
    }

    fn set_song(&mut self, _: usize) -> usize {
        0
    }
}

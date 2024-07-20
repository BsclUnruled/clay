use super::ToCross;
use crate::clay::var::Cross;

#[derive(Debug)]
struct Undef();

impl ToCross for Undef {
    fn to_cross(self) -> super::Cross {
        Cross::new(
            Box::new(self)
        )
    }
}

thread_local! {
    static UD:super::Cross = Undef().to_cross();
}

pub fn undef() -> super::Cross {
    UD.with(|ud| ud.clone())
}

pub fn test() {
    let ud = undef();
    let ud2 = ud.uncross();
    let ud3 = ud2.cast::<Undef>();
    println!("{:?}", ud3);
}
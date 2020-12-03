use fixnum::{impl_op, typenum, FixedPoint::U9};

type FP = FixedPoint<i64, U9>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Size(i32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Price(FP);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct PriceDelta(FP);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Amount(FP);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Ratio(FP);

impl_op!(Size [cadd] Size = Size);
impl_op!(Size [csub] Size = Size);
impl_op!(Size [rdiv] Size = Ratio);
impl_op!(Size [cmul] Price = Amount);
impl_op!(Price [csub] Price = PriceDelta);
impl_op!(Price [cadd] PriceDelta = Price);
impl_op!(Price [rdiv] Price = Ratio);
impl_op!(Price [rmul] Ratio = Price);
impl_op!(PriceDelta [cadd] PriceDelta = PriceDelta);
impl_op!(Amount [cadd] Amount = Amount);
impl_op!(Amount [csub] Amount = Amount);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use fixnum::ops::*;

    let size = Size(2);
    let price = Price("4.25".parse()?);
    let amount = size.cmul(price)?;
    assert_eq!(amount, Amount("8.5".parse()?));

    let half = price.rmul(Ratio("0.5".parse()?), RoundMode::Ceil)?;
    assert_eq!(half, Price("2.125".parse()?));

    Ok(())
}

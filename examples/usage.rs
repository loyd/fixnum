use fixnum::{legit_op, FixedPoint};
use typenum::U9;

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

legit_op!(Size [cadd] Size = Size);
legit_op!(Size [csub] Size = Size);
legit_op!(Size [rdiv] Size = Ratio);
legit_op!(Size [cmul] Price = Amount);
legit_op!(Price [csub] Price = PriceDelta);
legit_op!(Price [cadd] PriceDelta = Price);
legit_op!(Price [rdiv] Price = Ratio);
legit_op!(Price [rmul] Ratio = Price);
legit_op!(PriceDelta [cadd] PriceDelta = PriceDelta);
legit_op!(Amount [cadd] Amount = Amount);
legit_op!(Amount [csub] Amount = Amount);

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

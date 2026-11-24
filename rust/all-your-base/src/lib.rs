#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    };
    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    };

    number
        .iter()
        .try_fold(0, |sol, d| {
            if *d < from_base {
                Ok(*d + from_base * sol)
            } else {
                Err(Error::InvalidDigit(*d))
            }
        })
        .map(|mut tenbase| {
            if tenbase == 0 {
                return vec![0];
            }
            let digits = tenbase.ilog(to_base);
            let mut v = Vec::with_capacity(digits as usize + 1);
            let mut powers = to_base.pow(digits);
            while powers > 0 {
                v.push(tenbase / powers);
                tenbase %= powers;
                powers /= to_base;
            }
            v
        })
}

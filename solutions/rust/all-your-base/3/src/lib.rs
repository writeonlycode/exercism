#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut r = 0;

    for (i, &v) in number.iter().rev().enumerate() {
        if v >= from_base {
            return Err(Error::InvalidDigit(v));
        }

        r += v * from_base.pow(i as u32);
    }

    let mut t = Vec::new();

    loop {
        if r == 0 {
            break;
        } else {
            t.push(r as u32 % to_base);
            r /= r / to_base;
        }
    }

    if t.is_empty() {
        t.push(0);
    }

    Ok(t.into_iter().rev().collect())
}

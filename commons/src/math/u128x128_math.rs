use dlmm_interface::Rounding;
use ruint::aliases::U256;

/// (x * y) / denominator
pub(crate) fn mul_div(x: u128, y: u128, denominator: u128, rounding: Rounding) -> Option<u128> {
    if denominator == 0 {
        return None;
    }

    let x = U256::from(x);
    let y = U256::from(y);
    let denominator = U256::from(denominator);

    let prod = x.checked_mul(y)?;

    match rounding {
        Rounding::Up => prod.div_ceil(denominator).try_into().ok(),
        Rounding::Down => {
            let (quotient, _) = prod.div_rem(denominator);
            quotient.try_into().ok()
        }
    }
}

/// (x * y) >> offset
#[inline]
pub fn mul_shr(x: u128, y: u128, offset: u8, rounding: Rounding) -> Option<u128> {
    /*
        (a * b) >> 64
            => (a * b) / (1<<64)
            => if roundup ((a * b) + u64::MAX) >> 64
               else (a * b) >> 64
     */
    match rounding {
        Rounding::Up => {
            ((x * y) + u64::MAX as u128).checked_shr(offset.into())
        }
        Rounding::Down => {
            (x * y).checked_shr(offset.into())
        }
    }
}

/// (x << offset) / y
#[inline]
pub fn shl_div(x: u128, y: u128, offset: u8, rounding: Rounding) -> Option<u128> {
    // Remove the multiple and just do a shift.
    let x = U256::from(x);
    let x_shifted = x.checked_shl(offset.into())?;

    let y = U256::from(y);

    match rounding {
        Rounding::Up => x_shifted.div_ceil(y).try_into().ok(),
        Rounding::Down => {
            let (quotient, _) = x_shifted.div_rem(y);
            quotient.try_into().ok()
        }
    }
}

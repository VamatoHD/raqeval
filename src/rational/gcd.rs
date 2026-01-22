#![no_std]

use core::num::NonZeroU128;

/// Calculates the Greatest Commom Divider (GCD) of two `u128` using the **Binary GCD** algorithm
///
/// https://en.wikipedia.org/wiki/Binary_GCD_algorithm
///
/// Algorithm overview:
///
/// 1. gcd(n, 0) = gcd(0, n) = n
/// 2. gcd(2ᵃ ⋅ u, 2ᵇ ⋅ v) = 2ᵏ ⋅ gcd(u, v) where k = min(a, b)
/// 3. gcd(u, 2ᵇ ⋅ v) = gcd(u, v) if u is odd
/// 4. gcd(u, v) = gcd(u, v-u) if u,v are both odd and u ≤ v
pub fn gcd_u128(mut u: u128, mut v: u128) -> u128 {
    //Quick exit for base cases (identity 1)
    if u == v {
        return u; //or v
    } else if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Identities 2 and 3:
    // Count are remove all common factors of 2
    let k = u.trailing_zeros().min(v.trailing_zeros());
    // Remove all factors of 2
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    //Both u and v are now odd

    loop {
        //Make sure u ≤ v
        if u > v {
            // Same as: std::mem::swap(&mut u, &mut v)
            // But this is no_std
            let temp = u;
            u = v;
            v = temp;
        }

        // u and v are odd, so v-u is even
        v -= u;

        if v == 0 {
            break;
        };

        //Identity 3, since v is even
        v >>= v.trailing_zeros();
        // v is odd again or zero
    }

    // Multiply back the common powers of 2
    // Safe left shift (u << k)
    u.checked_shl(k).unwrap_or(1)
}

pub fn gdc_nonzerou128(u: NonZeroU128, v: NonZeroU128) -> NonZeroU128 {
    match NonZeroU128::new(gcd_u128(u.get(), v.get())) {
        Some(result) => result,
        None => {
            // gcd_u128 is only 0 if both u and v are zero
            // however u and v are never 0
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    const MAX: u128 = !0;
    use super::gcd_u128 as gcd;

    #[test]
    fn base_cases() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, MAX), MAX);
        assert_eq!(gcd(MAX, 0), MAX);
        assert_eq!(gcd(1, MAX), 1);
        assert_eq!(gcd(MAX, 1), 1);
        assert_eq!(gcd(MAX, MAX), MAX);
    }

    #[test]
    fn random_cases() {
        assert_eq!(gcd(100, 10), 10);
        assert_eq!(gcd(23, 7), 1);
        assert_eq!(gcd(7, 5), 1);
    }
}

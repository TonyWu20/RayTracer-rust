use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Sub, SubAssign};

use crate::Float;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use bytemuck::{Pod, Zeroable};

use super::linalg::tuple::Tuple;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[repr(transparent)]
pub struct Color<T: Float>(pub(crate) Tuple<T, 3>);

impl<T: Float> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self(Tuple::from([r, g, b]))
    }
}

#[cfg(test)]
mod test {

    use approx::assert_relative_eq;

    use super::Color;

    #[test]
    fn mul_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_relative_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
// `Zeroable` impls for "Color" types are sound:
//
// - They are inhabited: structs plus bound `T: Zeroable`.
// - They only consists of `Zeroable` fields, thus zero bit pattern is fine.
unsafe impl<T: Float + Zeroable> Zeroable for Color<T> {}
unsafe impl<T: Float + Zeroable> Zeroable for RgbView<T> {}

// `Pod` impls for "Color" types are sound:
//
// - "The type must be inhabited": guaranteed by all being structs and the bound `T: Pod`.
// - "The type must not contain any padding bytes": this is true according to [1].
// - "The type needs to have all fields also be `Pod`": trivially true due to `T: Pod`.
// - "The type must allow any bit pattern": true based on the previous two facts.
// - "The type needs to be `repr(C)` or `repr(transparent)`": trivially true.
//
// [1] https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
unsafe impl<T: Float + Pod> Pod for Color<T> {}
unsafe impl<T: Float + Pod> Pod for RgbView<T> {}

/// Helper struct giving access to the individual components of a 3D
/// tuple.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct RgbView<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}
/// Enable access by `.r`, `.g` and `.b`
impl<T: Float> Deref for Color<T> {
    type Target = RgbView<T>;

    fn deref(&self) -> &Self::Target {
        bytemuck::cast_ref(self)
    }
}

impl<T: Float> DerefMut for Color<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        bytemuck::cast_mut(self)
    }
}

impl<T: Float> Default for Color<T> {
    fn default() -> Self {
        Self(Tuple([(); 3].map(|_| T::zero())))
    }
}

impl<T: Float + AbsDiffEq> AbsDiffEq for Color<T>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.r, &other.r, epsilon)
            && T::abs_diff_eq(&self.g, &other.g, epsilon)
            && T::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

impl<T: Float + RelativeEq> RelativeEq for Color<T>
where
    T::Epsilon: Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(&self.r, &other.r, epsilon, max_relative)
            && T::relative_eq(&self.g, &other.g, epsilon, max_relative)
            && T::relative_eq(&self.b, &other.b, epsilon, max_relative)
    }
}
impl<T: Float + UlpsEq> UlpsEq for Color<T>
where
    T::Epsilon: Copy,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.r, &other.r, epsilon, max_ulps)
            && T::ulps_eq(&self.g, &other.g, epsilon, max_ulps)
            && T::ulps_eq(&self.b, &other.b, epsilon, max_ulps)
    }
}

impl<T: Float> Add for Color<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(tc1) = self;
        let Self(tc2) = rhs;
        Self(tc1 + tc2)
    }
}

impl<T: Float> AddAssign for Color<T> {
    fn add_assign(&mut self, rhs: Self) {
        let Self(tc1) = self;
        let Self(tc2) = rhs;
        *tc1 += tc2;
    }
}

impl<T: Float> Sub for Color<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let Self(tc1) = self;
        let Self(tc2) = rhs;
        Self(tc1 - tc2)
    }
}

impl<T: Float> SubAssign for Color<T> {
    fn sub_assign(&mut self, rhs: Self) {
        let Self(tc1) = self;
        let Self(tc2) = rhs;
        *tc1 -= tc2;
    }
}

impl<T: Float> Mul<T> for Color<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl<T: Float> MulAssign<T> for Color<T> {
    fn mul_assign(&mut self, rhs: T) {
        let Self(tc1) = self;
        *tc1 *= rhs;
    }
}
/// Hadamard product for `Color` * `Color`
impl<T: Float> Mul<Color<T>> for Color<T> {
    type Output = Color<T>;

    fn mul(self, rhs: Color<T>) -> Self::Output {
        Self(Tuple([self.r * rhs.r, self.g * rhs.g, self.b * rhs.b]))
    }
}

impl<T: Float> Div<T> for Color<T> {
    type Output = Color<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

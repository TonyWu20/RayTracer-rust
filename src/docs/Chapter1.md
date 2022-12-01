# Chapter 1. Tuples, Points and Vectors

##Todo
1. Utilization of `rust`'s type system and generics.
2. Implementation of name access (`.x`)

## Building the fundamental blocks: `Tuple`, `Point`, and `Vector`

In the world of 3d computer graphics, we express the concept of *location* and *displacement* with a tuple-like structure `(x, y, z)` as *cartesian coordinate*. Considering their difference in maths, we usually represent them in the form of *homogeneous coordinate*: `(x, y, z, w)`. We set `w` to `1` for `Point` and `0` for `Vector`.

In `rust`, with the powerful type system, we can implement these building blocks with clear definition and strict management of their viable behaviour and methods. For example, we know that adding two `Point` together produces nothing meaningful. Meanwhile, adding a `Point` with a `Vector` is to translate the `Point` along the direction given by the `Vector`, producing a new `Point`.

To regulate the interactions between `Point` and `Vector`, the type and trait system is leveraged.

Example:
```
/// `Vector` + `Point` = translated `Point`
impl<T: Scalar, const N: usize> Add<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        Self(Tuple(array::from_fn(|i| self[i] + rhs[i])))
    }
}
/// `Point` B - `Point` A = `Vector` AB
impl<T: Scalar, const N: usize> Sub<Self> for Point<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Point<T, N>) -> Self::Output {
        Vector(Tuple(array::from_fn(|i| self[i] - rhs[i])))
    }
}
/// Substract a `Vector` from `Point` = translated `Point`
impl<T: Scalar, const N: usize> Sub<Vector<T, N>> for Point<T, N> {
    type Output = Self;
    fn sub(self, rhs: Vector<T, N>) -> Self::Output {
        Self(Tuple(array::from_fn(|i| self[i] - rhs[i])))
    }
}
```
Although we are going to utilize the `Point` and `Vector` in the three-dimensional space, which means we will deal with `f64` in almost the entire project, I still implement the `Point` and `Vector` in a generic way.
```
pub struct Point<T: Scalar, const N: usize>(pub(crate)[T;N]);
```
The `T: Scalar` part defines the type of the data contained in the array, while `N` is the size of the array. In particular, we define `Point3<T> = Point<T,4>` and `Vector3<T> = Vector<T, 4>`. That means, we will represent the 3D-point and vector in homogeneous coordinate. The `w` detail is handled behind the scene, ensuring `w` for `Point3` is one and `w` for `Vector3` is zero.  

pub mod math {
    use std::f32;

    pub const PI: f32 = 3.1415926535;
    pub const TWO_PI: f32 = PI * 2.0;
    pub const PI_OVER_2: f32 = PI / 2.0;
    pub const INFINITY: f32 = f32::INFINITY;
    pub const NEG_INFINITY: f32 = f32::NEG_INFINITY;

    pub fn to_radians(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

    pub fn to_degrees(radians: f32) -> f32 {
        radians * 180.0 / PI
    }

    pub fn near_zero(val: f32, epsilon: f32) -> bool {
        val.abs() <= epsilon
    }

    pub fn max<T: PartialOrd>(a: T, b: T) -> T {
        if a < b { b } else { a }
    }

    pub fn min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b { a } else { b }
    }

    pub fn clamp<T: PartialOrd>(value: T, lower: T, upper: T) -> T {
        min(upper, max(lower, value))
    }

    pub fn abs(value: f32) -> f32 {
        value.abs()
    }

    pub fn cos(angle: f32) -> f32 {
        angle.cos()
    }

    pub fn sin(angle: f32) -> f32 {
        angle.sin()
    }

    pub fn tan(angle: f32) -> f32 {
        angle.tan()
    }

    pub fn acos(value: f32) -> f32 {
        value.acos()
    }

    pub fn atan2(y: f32, x: f32) -> f32 {
        y.atan2(x)
    }

    pub fn cot(angle: f32) -> f32 {
        1.0 / tan(angle)
    }

    pub fn lerp(a: f32, b: f32, f: f32) -> f32 {
        a + f * (b - a)
    }

    pub fn sqrt(value: f32) -> f32 {
        value.sqrt()
    }

    pub fn fmod(numer: f32, denom: f32) -> f32 {
        numer % denom
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const UNIT_X: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Vector2 = Vector2 { x: 0.0, y: 1.0 };
    pub const NEG_UNIT_X: Vector2 = Vector2 { x: -1.0, y: 0.0 };
    pub const NEG_UNIT_Y: Vector2 = Vector2 { x: 0.0, y: -1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn transform(vec: Vector2, mat: Matrix3, w: f32) -> Vector2 {
        Vector2 {
            x: vec.x * mat.mat[0][0] + vec.y * mat.mat[1][0] + w * mat.mat[2][0],
            y: vec.x * mat.mat[0][1] + vec.y * mat.mat[1][1] + w * mat.mat[2][1],
        }
    }

    pub fn zero() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    pub fn neg_unit_x() -> Self {
        Self { x: -1.0, y: 0.0 }
    }

    pub fn neg_unit_y() -> Self {
        Self { x: 0.0, y: -1.0 }
    }

    // Set both components in one line
    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    // 長さの二乗
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    // ベクトルの長さ
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    // 正規化
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
    }

    // 与えられたベクトルを正規化
    pub fn normalized(vec: Vector2) -> Vector2 {
        let mut temp = vec;
        temp.normalize();
        temp
    }

    // ドット積
    pub fn dot(a: Vector2, b: Vector2) -> f32 {
        a.x * b.x + a.y * b.y
    }

    // 線形補間
    pub fn lerp(a: Vector2, b: Vector2, f: f32) -> Vector2 {
        a + (b - a) * f
    }

    // 反射
    pub fn reflect(v: Vector2, n: Vector2) -> Vector2 {
        v - 2.0 * Vector2::dot(v, n) * n
    }

    // ベクトルと行列の変換 (未実装)
    // pub fn transform(vec: Vector2, mat: Matrix3, w: f32) -> Vector2 {
    //     // Matrix3の実装に依存します
    // }
}

// ベクトル演算
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// f32 * Vector2 の実装
impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, vec: Vector2) -> Vector2 {
        Vector2 {
            x: vec.x * self,
            y: vec.y * self,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const UNIT_X: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const NEG_UNIT_X: Vector3 = Vector3 { x: -1.0, y: 0.0, z: 0.0 };
    pub const NEG_UNIT_Y: Vector3 = Vector3 { x: 0.0, y: -1.0, z: 0.0 };
    pub const NEG_UNIT_Z: Vector3 = Vector3 { x: 0.0, y: 0.0, z: -1.0 };
    pub const INFINITY: Vector3 = Vector3 { x: f32::INFINITY, y: f32::INFINITY, z: f32::INFINITY };
    pub const NEG_INFINITY: Vector3 = Vector3 { x: f32::NEG_INFINITY, y: f32::NEG_INFINITY, z: f32::NEG_INFINITY };

    pub fn transform(vec: Vector3, mat: Matrix4, w: f32) -> Vector3 {
        Vector3 {
            x: vec.x * mat.mat[0][0] + vec.y * mat.mat[1][0] + vec.z * mat.mat[2][0] + w * mat.mat[3][0],
            y: vec.x * mat.mat[0][1] + vec.y * mat.mat[1][1] + vec.z * mat.mat[2][1] + w * mat.mat[3][1],
            z: vec.x * mat.mat[0][2] + vec.y * mat.mat[1][2] + vec.z * mat.mat[2][2] + w * mat.mat[3][2],
        }
    }

    pub fn transform_with_persp_div(vec: Vector3, mat: Matrix4, w: f32) -> Vector3 {
        let mut ret_val = Vector3::transform(vec, mat, w);
        let transformed_w = vec.x * mat.mat[0][3]
            + vec.y * mat.mat[1][3]
            + vec.z * mat.mat[2][3]
            + w * mat.mat[3][3];
        if transformed_w.abs() > f32::EPSILON {
            ret_val = Vector3 {
                x: ret_val.x / transformed_w,
                y: ret_val.y / transformed_w,
                z: ret_val.z / transformed_w,
            };
        }
        ret_val
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Zero Vector
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    // Set all three components
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    // ベクトルの長さの二乗
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // ベクトルの長さ
    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    // 正規化
    pub fn normalize(&mut self) {
        let length = self.length();
        if length > 0.0 {
            self.x /= length;
            self.y /= length;
            self.z /= length;
        }
    }

    // 与えられたベクトルを正規化
    pub fn normalized(vec: Vector3) -> Vector3 {
        let mut temp = vec;
        temp.normalize();
        temp
    }

    // ドット積
    pub fn dot(a: Vector3, b: Vector3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    // クロス積
    pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    // 線形補間
    pub fn lerp(a: Vector3, b: Vector3, f: f32) -> Vector3 {
        a + (b - a) * f
    }

    // 反射
    pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - 2.0 * Vector3::dot(v, n) * n
    }

    // 行列での変換 (未実装)
    // pub fn transform(vec: Vector3, mat: Matrix4, w: f32) -> Vector3 {
    //     // Matrix4の実装に依存します
    // }

    // クォータニオンでの変換 (未実装)
    // pub fn transform_quaternion(vec: Vector3, q: Quaternion) -> Vector3 {
    //     // Quaternionの実装に依存します
    // }
}


// Vector3 - Vector3
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Vector3 + Vector3
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Vector3 * Scalar
impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Scalar * Vector3
impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vec: Vector3) -> Vector3 {
        Vector3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Vector3 *= Scalar
impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

// Vector3 += Vector3
impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// Vector3 -= Vector3
impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub mat: [[f32; 3]; 3],
}

impl Matrix3 {
    pub const IDENTITY: Matrix3 = Matrix3 {
        mat: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ],
    };

    pub fn new(mat: [[f32; 3]; 3]) -> Self {
        Self { mat }
    }

    // Identity Matrixの初期化
    pub fn identity() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    // const float*へのキャストに相当する機能
    pub fn as_ptr(&self) -> *const f32 {
        &self.mat[0][0] as *const f32
    }

    // 行列の乗算
    pub fn mul(self, other: Matrix3) -> Matrix3 {
        let mut result = Matrix3::identity();

        for row in 0..3 {
            for col in 0..3 {
                result.mat[row][col] =
                    self.mat[row][0] * other.mat[0][col] +
                        self.mat[row][1] * other.mat[1][col] +
                        self.mat[row][2] * other.mat[2][col];
            }
        }

        result
    }

    // *= 演算子の実装
    pub fn mul_assign(&mut self, other: Matrix3) {
        *self = self.mul(other);
    }

    // スケール行列の作成
    pub fn create_scale(x_scale: f32, y_scale: f32) -> Self {
        Self {
            mat: [
                [x_scale, 0.0, 0.0],
                [0.0, y_scale, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn create_uniform_scale(scale: f32) -> Self {
        Self::create_scale(scale, scale)
    }

    // 回転行列の作成 (Z軸周り)
    pub fn create_rotation(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        Self {
            mat: [
                [cos_theta, sin_theta, 0.0],
                [-sin_theta, cos_theta, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    // 平行移動行列の作成
    pub fn create_translation(translation: Vector2) -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [translation.x, translation.y, 1.0],
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub mat: [[f32; 4]; 4],
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        mat: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn new(mat: [[f32; 4]; 4]) -> Self {
        Self { mat }
    }

    pub fn invert(&mut self) {
        // Implement inversion logic here
    }

    pub fn create_from_quaternion(q: Quaternion) -> Matrix4 {
        Matrix4 {
            mat: [
                [
                    1.0 - 2.0 * q.y * q.y - 2.0 * q.z * q.z,
                    2.0 * q.x * q.y + 2.0 * q.w * q.z,
                    2.0 * q.x * q.z - 2.0 * q.w * q.y,
                    0.0,
                ],
                [
                    2.0 * q.x * q.y - 2.0 * q.w * q.z,
                    1.0 - 2.0 * q.x * q.x - 2.0 * q.z * q.z,
                    2.0 * q.y * q.z + 2.0 * q.w * q.x,
                    0.0,
                ],
                [
                    2.0 * q.x * q.z + 2.0 * q.w * q.y,
                    2.0 * q.y * q.z - 2.0 * q.w * q.x,
                    1.0 - 2.0 * q.x * q.x - 2.0 * q.y * q.y,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Identity Matrixの初期化
    pub fn identity() -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // const float*へのキャストに相当する機能
    pub fn as_ptr(&self) -> *const f32 {
        &self.mat[0][0] as *const f32
    }

    // 行列の乗算
    pub fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Matrix4::identity();

        for row in 0..4 {
            for col in 0..4 {
                result.mat[row][col] =
                    self.mat[row][0] * other.mat[0][col] +
                        self.mat[row][1] * other.mat[1][col] +
                        self.mat[row][2] * other.mat[2][col] +
                        self.mat[row][3] * other.mat[3][col];
            }
        }

        result
    }

    // *= 演算子の実装
    pub fn mul_assign(&mut self, other: Matrix4) {
        *self = self.mul(other);
    }

    // スケール行列の作成
    pub fn create_scale(x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        Self {
            mat: [
                [x_scale, 0.0, 0.0, 0.0],
                [0.0, y_scale, 0.0, 0.0],
                [0.0, 0.0, z_scale, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // 一様スケール行列の作成
    pub fn create_uniform_scale(scale: f32) -> Self {
        Self::create_scale(scale, scale, scale)
    }

    // X軸回転行列の作成
    pub fn create_rotation_x(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos_theta, sin_theta, 0.0],
                [0.0, -sin_theta, cos_theta, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Y軸回転行列の作成
    pub fn create_rotation_y(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        Self {
            mat: [
                [cos_theta, 0.0, -sin_theta, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [sin_theta, 0.0, cos_theta, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Z軸回転行列の作成
    pub fn create_rotation_z(theta: f32) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        Self {
            mat: [
                [cos_theta, sin_theta, 0.0, 0.0],
                [-sin_theta, cos_theta, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // 平行移動行列の作成
    pub fn create_translation(translation: Vector3) -> Self {
        Self {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [translation.x, translation.y, translation.z, 1.0],
            ],
        }
    }

    // クォータニオンからの回転行列の作成 (未実装)
    // pub fn create_from_quaternion(q: Quaternion) -> Self {
    //     // Quaternionの実装に依存します
    // }

    // 視点行列の作成
    pub fn create_look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let zaxis = Vector3::normalized(target - eye);
        let xaxis = Vector3::normalized(Vector3::cross(up, zaxis));
        let yaxis = Vector3::normalized(Vector3::cross(zaxis, xaxis));

        let trans = Vector3 {
            x: -Vector3::dot(xaxis, eye),
            y: -Vector3::dot(yaxis, eye),
            z: -Vector3::dot(zaxis, eye),
        };

        Self {
            mat: [
                [xaxis.x, yaxis.x, zaxis.x, 0.0],
                [xaxis.y, yaxis.y, zaxis.y, 0.0],
                [xaxis.z, yaxis.z, zaxis.z, 0.0],
                [trans.x, trans.y, trans.z, 1.0],
            ],
        }
    }

    // 正射影行列の作成
    pub fn create_ortho(width: f32, height: f32, near: f32, far: f32) -> Self {
        Self {
            mat: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, 1.0 / (far - near), 0.0],
                [0.0, 0.0, near / (near - far), 1.0],
            ],
        }
    }

    // FOVベースの透視投影行列の作成
    pub fn create_perspective_fov(fov_y: f32, width: f32, height: f32, near: f32, far: f32) -> Self {
        let y_scale = (fov_y / 2.0).tan().recip();
        let x_scale = y_scale * height / width;

        Self {
            mat: [
                [x_scale, 0.0, 0.0, 0.0],
                [0.0, y_scale, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), 1.0],
                [0.0, 0.0, -near * far / (far - near), 0.0],
            ],
        }
    }

    // Simple View-Projection行列の作成
    pub fn create_simple_view_proj(width: f32, height: f32) -> Self {
        Self {
            mat: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0, 1.0],
            ],
        }
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix4::identity();

        for row in 0..4 {
            for col in 0..4 {
                result.mat[row][col] =
                    self.mat[row][0] * rhs.mat[0][col] +
                        self.mat[row][1] * rhs.mat[1][col] +
                        self.mat[row][2] * rhs.mat[2][col] +
                        self.mat[row][3] * rhs.mat[3][col];
            }
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    // 軸と角度からクォータニオンを生成
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let scalar = (angle / 2.0).sin();
        Self {
            x: axis.x * scalar,
            y: axis.y * scalar,
            z: axis.z * scalar,
            w: (angle / 2.0).cos(),
        }
    }

    // 内部コンポーネントの設定
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn conjugate(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
        self.z *= -1.0;
    }

    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    // クォータニオンの正規化
    pub fn normalized(mut q: Quaternion) -> Quaternion {
        q.normalize();
        q
    }

    // 線形補間
    pub fn lerp(a: Quaternion, b: Quaternion, f: f32) -> Quaternion {
        let mut ret_val = Quaternion {
            x: Self::lerp_f32(a.x, b.x, f),
            y: Self::lerp_f32(a.y, b.y, f),
            z: Self::lerp_f32(a.z, b.z, f),
            w: Self::lerp_f32(a.w, b.w, f),
        };
        ret_val.normalize();
        ret_val
    }

    pub fn lerp_f32(a: f32, b: f32, f: f32) -> f32 {
        a + f * (b - a)
    }

    // ドット積
    pub fn dot(a: Quaternion, b: Quaternion) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    // 球面線形補間
    pub fn slerp(a: Quaternion, b: Quaternion, f: f32) -> Quaternion {
        let raw_cosom = Quaternion::dot(a, b);
        let cosom = if raw_cosom >= 0.0 { raw_cosom } else { -raw_cosom };

        let (scale0, scale1) = if cosom < 0.9999 {
            let omega = cosom.acos();
            let inv_sin = 1.0 / omega.sin();
            (
                ((1.0 - f) * omega).sin() * inv_sin,
                (f * omega).sin() * inv_sin,
            )
        } else {
            (1.0 - f, f)
        };

        let scale1 = if raw_cosom < 0.0 { -scale1 } else { scale1 };

        let mut ret_val = Quaternion {
            x: scale0 * a.x + scale1 * b.x,
            y: scale0 * a.y + scale1 * b.y,
            z: scale0 * a.z + scale1 * b.z,
            w: scale0 * a.w + scale1 * b.w,
        };
        ret_val.normalize();
        ret_val
    }

    // クォータニオンの連結
    pub fn concatenate(q: Quaternion, p: Quaternion) -> Quaternion {
        let qv = Vector3::new(q.x, q.y, q.z);
        let pv = Vector3::new(p.x, p.y, p.z);
        let new_vec = p.w * qv + q.w * pv + Vector3::cross(pv, qv);
        Quaternion {
            x: new_vec.x,
            y: new_vec.y,
            z: new_vec.z,
            w: p.w * q.w - Vector3::dot(pv, qv),
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

pub mod color {
    use crate::math::Vector3;

    pub const BLACK: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const WHITE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const RED: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const GREEN: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const BLUE: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const YELLOW: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 0.0 };
    pub const LIGHT_YELLOW: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 0.88 };
    pub const LIGHT_BLUE: Vector3 = Vector3 { x: 0.68, y: 0.85, z: 0.9 };
    pub const LIGHT_PINK: Vector3 = Vector3 { x: 1.0, y: 0.71, z: 0.76 };
    pub const LIGHT_GREEN: Vector3 = Vector3 { x: 0.56, y: 0.93, z: 0.56 };
}

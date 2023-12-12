#![allow(dead_code)]

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const UP: Vec3 = vec3(0.0, 1.0, 0.0);
    pub const ZERO: Vec3 = vec3(0.0, 0.0, 0.0);
}

#[inline]
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = vec2(0.0, 0.0);
}

#[inline]
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

pub type Mat4 = [[f32; 4]; 4];

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        sub_vec3(self, rhs)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        add_vec3(self, rhs)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        mul_vec3(self, rhs)
    }
}

pub fn sub_vec3(v0: Vec3, v1: Vec3) -> Vec3 {
    vec3(v0.x - v1.x, v0.y - v1.y, v0.z - v1.z)
}

pub fn add_vec3(v0: Vec3, v1: Vec3) -> Vec3 {
    vec3(v0.x + v1.x, v0.y + v1.y, v0.z + v1.z)
}

pub fn mul_vec3(v: Vec3, val: f32) -> Vec3 {
    vec3(v.x * val, v.y * val, v.z * val)
}

pub fn dot_vec3(v0: Vec3, v1: Vec3) -> f32 {
    v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
}

#[inline]
pub fn len_vec3(v: Vec3) -> f32 {
    dot_vec3(v, v).sqrt()
}

pub fn norm_vec3(v: Vec3) -> Vec3 {
    let l = len_vec3(v);

    if l != 0.0 {
        vec3(v.x / l, v.y / l, v.z / l)
    } else {
        Vec3::ZERO
    }
}

pub fn cross_vec3(v0: Vec3, v1: Vec3) -> Vec3 {
    vec3((v0.y * v1.z) - (v0.z * v1.y), (v0.z * v1.x) - (v0.x * v1.z), (v0.x * v1.y) - (v0.y * v1.x))
}

pub const fn identity_mat4() -> Mat4 {
    let mut m = [[0.0; 4]; 4];

    m[0][0] = 1.0;
    m[1][1] = 1.0;
    m[2][2] = 1.0;
    m[3][3] = 1.0;

    m
}

pub fn rotate_mat4(angle: f32, axis_unorm: Vec3) -> Mat4 {
    let mut m = identity_mat4();

    let axis = norm_vec3(axis_unorm);
    let sin_theta = angle.to_radians().sin();
    let cos_theta = angle.to_radians().cos();
    let cos_value = 1.0 - cos_theta;

    m[0][0] = (axis.x * axis.x * cos_value) + cos_theta;
    m[0][1] = (axis.x * axis.y * cos_value) + (axis.z * sin_theta);
    m[0][2] = (axis.x * axis.z * cos_value) - (axis.y * sin_theta);
    m[1][0] = (axis.y * axis.x * cos_value) - (axis.z * sin_theta);
    m[1][1] = (axis.y * axis.y * cos_value) + cos_theta;
    m[1][2] = (axis.y * axis.z * cos_value) + (axis.x * sin_theta);
    m[2][0] = (axis.z * axis.x * cos_value) + (axis.y * sin_theta);
    m[2][1] = (axis.z * axis.y * cos_value) - (axis.x * sin_theta);
    m[2][2] = (axis.z * axis.z * cos_value) + cos_theta;

    m
}

pub fn persp_mat4(fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let mut m = identity_mat4();
    let t = f32::tan(fov * (std::f32::consts::PI / 360.0));

    m[0][0] = 1.0 / t;
    m[1][1] = aspect / t;
    m[2][3] = -1.0;
    m[2][2] = (near + far) / (near - far);
    m[3][2] = (2.0 * near * far) / (near - far);
    m[3][3] = 0.0;

    m
}

pub fn lookat_mat4(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let mut m = [[0.0; 4]; 4];

    let f = norm_vec3(center - eye);
    let s = norm_vec3(cross_vec3(f, up));
    let u = cross_vec3(s, f);

    m[0][0] = s.x;
    m[0][1] = u.x;
    m[0][2] = -f.x;

    m[1][0] = s.y;
    m[1][1] = u.y;
    m[1][2] = -f.y;

    m[2][0] = s.z;
    m[2][1] = u.z;
    m[2][2] = -f.z;

    m[3][0] = -dot_vec3(s, eye);
    m[3][1] = -dot_vec3(u, eye);
    m[3][2] = dot_vec3(f, eye);
    m[3][3] = 1.0;

    m
}

pub fn translate_mat4(translation: Vec3) -> Mat4 {
    let mut m = identity_mat4();

    m[3][0] = translation.x;
    m[3][1] = translation.y;
    m[3][2] = translation.z;

    m
}

pub fn mul_mat4(left: Mat4, right: Mat4) -> Mat4 {
    let mut m = [[0.0; 4]; 4];

    for col in 0..4 {
        for row in 0..4 {
            m[col][row] = left[0][row] * right[col][0]
                + left[1][row] * right[col][1]
                + left[2][row] * right[col][2]
                + left[3][row] * right[col][3];
        }
    }

    m
}

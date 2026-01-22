// vec1 + vec2 -> vec3
pub fn add_vec(matrix: &[i32], v1: usize, v2: usize, n: usize) -> Vec<i32> {
    (0..n).map(|i| matrix[i + v1] + matrix[i + v2]).collect()
}

// vec1 + vec2 -> vec1
pub fn add_assign_vec(matrix: &mut [i32], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] + matrix[v2 + i];
    });
}

// vec1 - vec2 -> vec3
pub fn sub_vec(matrix: &[i32], v1: usize, v2: usize, n: usize) -> Vec<i32> {
    (0..n).map(|i| matrix[i + v1] - matrix[i + v2]).collect()
}

// vec1 - vec2 -> vec1
pub fn sub_assign_vec(matrix: &mut [i32], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] - matrix[v2 + i];
    });
}

// vec1 * vec2 -> vec3, not that kind of mult 
pub fn mul_vec(matrix: &[i32], v1: usize, v2: usize, n: usize) -> Vec<i32> {
    (0..n).map(|i| matrix[i + v1] * matrix[i + v2]).collect()
}

// vec1 * vec2 -> vec1
pub fn mul_assign_vec(matrix: &mut [i32], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] * matrix[v2 + i];
    });
}

// vec1 / vec2 -> vec3
pub fn div_vec(matrix: &[i32], v1: usize, v2: usize, n: usize) -> Vec<i32> {
    (0..n).map(|i| matrix[i + v1] / matrix[i + v2]).collect()
}

// vec1 / vec2 -> vec1
pub fn div_assign_vec(matrix: &mut [i32], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] / matrix[v2 + i];
    });
}

pub fn dot_vec(matrix: &[i32], v1: usize, v2: usize, n: usize) -> i32 {
    (0..n).fold(0, |acc, i| acc + matrix[v1 + i] * matrix[v2 + i])
}

pub fn unit_vector(matrix: &[i32], v: usize, n: usize) -> Vec<i32> {
    (0..n).map(|i| matrix[i + v] / n as i32).collect()
}

// vec1 + vec2 -> vec3
pub fn add_vec(matrix: &[f64], v1: usize, v2: usize, n: usize) -> Vec<f64> {
    (0..n).map(|i| matrix[i + v1] + matrix[i + v2]).collect()
}

// vec1 + vec2 -> vec1
pub fn add_assign_vec(matrix: &mut [f64], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] + matrix[v2 + i];
    });
}

// vec1 - vec2 -> vec3
pub fn sub_vec(matrix: &[f64], v1: usize, v2: usize, n: usize) -> Vec<f64> {
    (0..n).map(|i| matrix[i + v1] - matrix[i + v2]).collect()
}

// vec1 - vec2 -> vec1
pub fn sub_assign_vec(matrix: &mut [f64], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] - matrix[v2 + i];
    });
}

// vec1 * vec2 -> vec3, not that kind of mult 
pub fn mul_vec(matrix: &[f64], v1: usize, v2: usize, n: usize) -> Vec<f64> {
    (0..n).map(|i| matrix[i + v1] * matrix[i + v2]).collect()
}

// vec1 * vec2 -> vec1
pub fn mul_assign_vec(matrix: &mut [f64], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] * matrix[v2 + i];
    });
}

// vec1 / vec2 -> vec3
pub fn div_vec(matrix: &[f64], v1: usize, v2: usize, n: usize) -> Vec<f64> {
    (0..n).map(|i| matrix[i + v1] / matrix[i + v2]).collect()
}

// vec1 / vec2 -> vec1
pub fn div_assign_vec(matrix: &mut [f64], v1: usize, v2: usize, n: usize) {
    (0..n).for_each(|i| {
        matrix[v1 + i] = matrix[v1 + i] / matrix[v2 + i];
    });
}

pub fn dot_vec(matrix: &[f64], v1: usize, v2: usize, n: usize) -> f64 {
    (0..n).fold(0.0, |acc, i| acc + matrix[v1 + i] * matrix[v2 + i])
}

pub fn unit_vector(matrix: &[f64], v: usize, n: usize) -> Vec<f64> {
    (0..n).map(|i| matrix[i + v] / n as f64).collect()
}

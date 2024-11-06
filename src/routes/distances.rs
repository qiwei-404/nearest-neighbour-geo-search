use std::arch::asm;


pub fn dist(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    // Ensure the vectors are of the same length
    assert!(vec1.len() == vec2.len(), "Vectors must be of equal length");

    let mut fl_dist: f32 = 0.0;
    for vec_index in 0..vec1.len() {
        let i = vec1[vec_index];
        let j = vec2[vec_index];
        let mut result: f32;

        unsafe {
            asm!(
                "movss xmm0, {0}",      // Load i into xmm0
                "movss xmm1, {1}",      // Load j into xmm1
                "subss xmm0, xmm1",     // xmm0 = xmm0 - xmm1 (i - j)
                "mulss xmm0, xmm0",     // xmm0 = xmm0 * xmm0 ((i - j) ^ 2)
                "movss {2}, xmm0",      // Store result back into result
                in(xmm_reg) i,
                in(xmm_reg) j,
                out(xmm_reg) result,
            );
        }

        fl_dist += result;
    }
    fl_dist.sqrt()
}

// pub fn manhattan(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
//     // Manhattan distance
//     let mut fl_dist: f32 = 0.0;
//     for vec_index in 0..(vec1.len()) {
//         fl_dist += (&vec1[vec_index] - vec2[vec_index]).abs();
//     }
//     return fl_dist;
// }

pub fn haversine(geo1: &Vec<f32>, geo2: &Vec<f32>) -> f32 {
    // geo distance in KMs
    let earth_radius: f32 = 6372.8;
    let conversion_factor: f32 = std::f32::consts::PI / 180.0;
    let lat1: f32 = geo1[0] * conversion_factor;
    let lat2: f32 = geo2[0] * conversion_factor;
    let lon1: f32 = geo1[1] * conversion_factor;
    let lon2: f32 = geo2[1] * conversion_factor;
    let d_lat: f32 = lat2 - lat1;
    let d_lon: f32 = lon2 - lon1;

    let a: f32 = ((d_lat / 2.0) * (d_lat / 2.0)).sin()
        + lat1.cos() * lat2.cos() * ((d_lon / 2.0) * d_lon / 2.0).sin();
    let c: f32 = 2.0 * a.sqrt().asin();
    let output = earth_radius * c;
    return output;
}

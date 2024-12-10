use profiler_macro::time_profiler;
use rand::Rng;
use std::ops::Mul;
use std::ops::Add;

pub fn setup() {
    // 0. setup
    // public parameters after setup: [a_ij^(k), a_ij^(l), phi^(k), phi^(l), b^(k), b0(l)']

    // 1. setup constraints
    // 1.1 get s_i and do norm check
    // 1.1.1 get s_i = s_1 - s_r; r is the number of witness s
    // each s_i is a vector of ring elements

    // 1.1.2 get beta norm bound, refer paper page 26, theorem 6.3

    // 1.1.3 do check: sum of s_i norm <= beta_square

    // 1.2 calculate b^(k)
    // 1.2.1 calculate dot product ss = a_ij * <s_i, s_j> for all i, j
    // a_ij is the quadratic coefficient, phi^(k) is the linear coefficient
    // 1.2.2 calculate phi_s = <phi^(k), s_i> for all i
    // 1.2.3 calculate b^(k) = sum(ss) + sum(phi_s)

    // 1.3 calculate b'^(l)
    // 1.3.1 calculate dot product ss = a_ij' * <s_i, s_j> for all i, j
    // a_ij' is the quadratic coefficient, phi^(l)' is the linear coefficient
    // 1.3.2 calculate phi_s = <phi^(l)', s_i> for all i
    // 1.3.3 calculate b'^(l) = sum(ss) + sum(phi_s)

    // L = |F'| = ceiling(128 / logQ)
}

#[time_profiler()]
pub fn prove() {
    println!("Proving something...");
    // 2. GOAL: calculate ajtai commitment (1st outer commitment)
    // 2.1 split t to t_i for all i
    // 2.1.1 get basis b1, refer to paper page 16, labrador c code line 142
    // refer to note: line => xxx

    // 2.2 split g = <s_i, s_j> for all i, j
    // 2.2.1 get basis b2 same as 2.1.1

    // 2.3 calculate u1
    // 2.3.1 B & C is randomly chosen
    // 2.3.2 calculate u1 = sum(B_ik * t_i^(k)) + sum(C_ijk * g_ij^(k))

    // ================================================

    // 3. GOAL: JL projection
    // 3.1 PI_i is randomly chosen from \Chi { -1, 0, 1 }^{256 * nd}
    //      (Using Guassian Distribution)
    // 3.2 caculate p_j = sum(<pi_i^(j)>, s_i) for all i-r
    // 3.3 Verifier have to check: || p || <= \sqrt{128} * beta

    // ================================================

    // 4. GOAL: Aggregation
    // 4.1 psi^(k) is randomly chosen from Z_q^{L}
    // 4.2 omega^(k) is randomly chosen from Z_q^{256}
    //      (Both using Guassian Distribution)
    // 4.3 caculate b^{''(k)}
    // 4.3.1 calculate a_ij^{''(k)} = sum(psi_l^(k) * a_ij^{'(l)}) for all l = 1..L
    // 4.3.2 calculate phi_i^{''(k)} =
    //       sum(psi_l^(k) * phi_i^{'(l)}) for all l = 1..L
    //       + sum(omega_j^(k) * sigma_{-1} * pi_i^{j)) for all j = 1..256
    // 4.3.3 calculate b^{''(k)} = sum(a_ij^{''(k)} * <s_i, s_j>) + sum(<phi_i^{''(k)}, s_i>)

    // Send b^{''(k)} to verifier
    // Verifier check: b_0^{''(k)} ?= <⟨omega^(k),p⟩> + sum(psi_l^(k) * b_0^{'(l)}) for all l = 1..L

    // ================================================

    // 5. GOAL: Calculate u2 (2nd outer commitment)
    // 5.1 vec<alpha> and vec<beta> are randomly chosen from R_q^{128/logQ} // why is this not `L`
    // 5.2 phi_i = sum(alpha_k * phi_i) + beta_k * phi_i^{''(k)}
    // 5.3 h_ij = 1/2 * (<phi_i, s_j> + <phi_j, s_i>)
    // 5.4 u2 = sum D_ij * h_ij^(k) for all k = 1..(t1-1)

    // Send u2 to verifier

    // ================================================

    // 6. GOAL: calculate z (Amortized Opening)
    // 6.1 c_i is randomly chosen from C
    // 6.2 calculate z = sum(c_i * s_i) for all i = 1..r
    // Send z, t_i, g_ij, h_ij to verifier
}

// Assuming you have a PolynomialRing type defined
#[derive(Debug, Clone)]
struct PolynomialRing {
    coefficients: Vec<usize>, // Example field, adjust as necessary
}

impl PolynomialRing {
    // Add this method to enable multiplication by PolynomialRing
    fn multiply_by_polynomial_ring(&self, other: &PolynomialRing) -> PolynomialRing {
        let mut result_coefficients =
            vec![0; self.coefficients.len() + other.coefficients.len() - 1];
        for (i, &coeff1) in self.coefficients.iter().enumerate() {
            for (j, &coeff2) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coeff1 * coeff2;
            }
        }
        PolynomialRing {
            coefficients: result_coefficients,
        }
    }

    fn add_polynomial_ring(&self, other: &PolynomialRing) -> PolynomialRing {
        let max_len = std::cmp::max(self.coefficients.len(), other.coefficients.len());
        let mut result_coefficients = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = if i < self.coefficients.len() {
                self.coefficients[i]
            } else {
                0
            };
            let b = if i < other.coefficients.len() {
                other.coefficients[i]
            } else {
                0
            };
            result_coefficients.push(a + b);
        }
        PolynomialRing {
            coefficients: result_coefficients,
        }
    }
}

impl Mul for PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(&other)
    }
}

impl Mul<&PolynomialRing> for PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: &PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(other)
    }
}

impl Mul<PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(&other)
    }
}

impl Mul<&PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn mul(self, other: &PolynomialRing) -> PolynomialRing {
        self.multiply_by_polynomial_ring(other)
    }
}

    // Start Generation Here
    impl Mul<usize> for PolynomialRing {
        type Output = PolynomialRing;

        fn mul(self, other: usize) -> PolynomialRing {
            let new_coefficients = self
                .coefficients
                .iter()
                .map(|c| c * other)
                .collect();
            PolynomialRing {
                coefficients: new_coefficients,
            }
        }
    }

    impl Mul<usize> for &PolynomialRing {
        type Output = PolynomialRing;

        fn mul(self, other: usize) -> PolynomialRing {
            let new_coefficients = self
                .coefficients
                .iter()
                .map(|c| c * other)
                .collect();
            PolynomialRing {
                coefficients: new_coefficients,
            }
        }
    }


impl Add for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(&other)
    }
}

impl Add<&PolynomialRing> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(other)
    }
}

impl Add<PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(&other)
    }
}

impl Add<&PolynomialRing> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &PolynomialRing) -> PolynomialRing {
        self.add_polynomial_ring(other)
    }
}

impl Add<usize> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: usize) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += other;
        } else {
            new_coefficients.push(other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<&usize> for PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &usize) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += *other;
        } else {
            new_coefficients.push(*other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<usize> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: usize) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += other;
        } else {
            new_coefficients.push(other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}

impl Add<&usize> for &PolynomialRing {
    type Output = PolynomialRing;

    fn add(self, other: &usize) -> PolynomialRing {
        let mut new_coefficients = self.coefficients.clone();
        if let Some(first) = new_coefficients.get_mut(0) {
            *first += *other;
        } else {
            new_coefficients.push(*other);
        }
        PolynomialRing {
            coefficients: new_coefficients,
        }
    }
}



// inner product of 2 vectors of PolynomialRing
// Start of Selection
fn inner_product_polynomial_ring(
    a: &Vec<PolynomialRing>,
    b: &Vec<PolynomialRing>,
) -> PolynomialRing {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a.multiply_by_polynomial_ring(b))
        .collect::<Vec<PolynomialRing>>()
        .into_iter()
        .reduce(|acc, x| acc.add_polynomial_ring(&x))
        .unwrap()
}

// Function to calculate b^(k)
fn calculate_b_constraint(
    s: &Vec<Vec<PolynomialRing>>,
    a_constraint: &Vec<Vec<PolynomialRing>>,
    phi_constraint: &Vec<PolynomialRing>,
) -> PolynomialRing {
    let mut b: PolynomialRing = PolynomialRing {
        coefficients: vec![0],
    };
    let s_len = s.len();
    // Calculate b^(k)
    for i in 0..s_len {
        for j in 0..s_len {
            // calculate inner product of s[i] and s[j], will retturn a single PolynomialRing
            let elem_s_i = &s[i];
            let elem_s_j = &s[j];
            // Calculate inner product and update b
            let inner_product_si_sj = inner_product_polynomial_ring(&elem_s_i, &elem_s_j);
            b = b.add_polynomial_ring(&inner_product_si_sj.multiply_by_polynomial_ring(&a_constraint[i][j]));
        }
        // calculate inner product of s[i] and phi
        s[i].iter()
            .map(|elem| elem)
            .zip(phi_constraint.iter())
            .for_each(|(x, y)| b = b.add_polynomial_ring(&x.multiply_by_polynomial_ring(&y)));
    }

    b
}

#[derive(Debug)]
struct RqMatrix {
    values: Vec<Vec<PolynomialRing>>, // matrix of PolynomialRing values
}

impl RqMatrix {
    fn new(size_kappa: usize, size_n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let values = (0..size_kappa)
            .map(|_| {
                (0..size_n)
                    .map(|_| PolynomialRing {
                        coefficients: (0..size_n).map(|_| rng.gen_range(1..10)).collect(),
                    })
                    .collect()
            })
            .collect();
        RqMatrix { values }
    }
}

// Ajtai commitment: calculate A matrix times s_i
fn calculate_a_times_s_i(a: &RqMatrix, s_i: &Vec<PolynomialRing>) -> Vec<PolynomialRing> {
    a.values
        .iter()
        .map(|row| {
            row.iter()
                .zip(s_i.iter())
                .map(|(a, b)| a.multiply_by_polynomial_ring(b))
                .collect::<Vec<PolynomialRing>>()
        })
        .collect::<Vec<Vec<PolynomialRing>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<PolynomialRing>>()
}

// convert number to basis
// 42 = 0 * 2^7 + 1 * 2^6 + 0 * 2^5 + 1 * 2^4 + 0 * 2^3 + 1 * 2^2 + 0 * 2^1 + 0 * 2^0
// first digit: 42 / 2 = 21, result_i = 0
// second digit: 21 / 2 = 10, result_i = 1
// third digit: 10 / 2 = 5, result_i = 0
// forth digit: 5 / 2 = 2, result_i = 1
// fifth digit: 2 / 2 = 1, result_i = 0
// sixth digit: 1 / 2 = 0, result_i = 1

fn num_to_basis(num: usize, basis: usize, digits: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut remainder = num;

    while remainder > 0 {
        result.push(remainder % basis);
        remainder /= basis;
    }

    while result.len() < digits {
        // push 0 to the highest position
        result.push(0);
    }

    result
}

// convert ring polynomial to basis
fn ring_polynomial_to_basis(poly: &PolynomialRing, basis: usize, digits: usize) -> Vec<Vec<usize>> {
    poly.coefficients
        .iter()
        .map(|coeff| num_to_basis(*coeff, basis, digits))
        .collect()
}

fn generate_gaussian_distribution(nd: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0; nd]; 256]; // Initialize a 256 x nd matrix

    for i in 0..256 {
        for j in 0..nd {
            let random_value: f32 = rng.gen(); // Generate a random float between 0 and 1
            matrix[i][j] = if random_value < 0.25 {
                -1 // 1/4 probability
            } else if random_value < 0.75 {
                0 // 1/2 probability
            } else {
                1 // 1/4 probability
            };
        }
    }

    matrix
}

// create test case for setup
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_prover() {
        let lambda: usize = 128;
        let double_lambda = lambda * 2;
        let q = 2usize.pow(32 as u32);
        let log_q = 32;
        let deg_bound_d = 3; // todo: should be 64
        // s is a vector of size r. each s_i is a PolynomialRing(Rq) with n coefficients
        let size_r: usize = 3; // r: Number of witness elements
        let size_n: usize = 5; // n
        let beta: usize = 50; // Example value for beta
        let witness_s: Vec<Vec<PolynomialRing>> = (1..=size_r)
            .map(|i| {
                (1..=size_n)
                    .map(|j| PolynomialRing {
                        coefficients: vec![i * 3 + j, i * 3 + j + 1, i * 3 + j + 2],
                    })
                    .collect()
            })
            .collect();
        println!("s: {:?}", witness_s);
        // Calculate the sum of squared norms
        let mut sum_squared_norms = 0;
        for vector in &witness_s {
            let norm_squared: usize = vector
                .iter()
                .map(|elem| elem.coefficients[0].pow(2)) // Calculate the square of each element
                .sum();
            sum_squared_norms += norm_squared; // Accumulate the squared norms
        }
        println!("sum_squared_norms: {}", sum_squared_norms);
        println!("beta^2: {}", beta.pow(2));
        // Check the condition
        assert!(
            sum_squared_norms <= beta.pow(2),
            "The condition is not satisfied: sum of squared norms exceeds beta^2"
        );

        let mut rng = rand::thread_rng();
        let k: usize = 6;
        // In DPCS(dot product constraint system), there are k constraints, each constraint has a, phi and b
        // Generate random a^(k)_{i,j}: k length vector of matrix, matrix length is r x r, each element in matrix is a R_q
        // todo: aij == aji
        let a_constraint: Vec<Vec<Vec<PolynomialRing>>> = (0..k)
            .map(|_| {
                (0..size_r)
                    .map(|_| {
                        (0..size_r)
                            .map(|_| PolynomialRing {
                                coefficients: (0..size_n).map(|_| rng.gen_range(0..10)).collect(),
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        // Generate random phi^(k)_{i}: vector(k length) of vector(n length), each element in vector is a R_q
        let phi_constraint: Vec<Vec<PolynomialRing>> = (0..k)
            .map(|_| (0..size_n)
                .map(|_| PolynomialRing {
                    coefficients: (0..size_n).map(|_| rng.gen_range(0..10)).collect(),
                })
                .collect()
            )
            .collect();
        println!("a_constraint: {:?}", a_constraint);
        println!("phi_constraint: {:?}", phi_constraint);

        // calculate b^(k)
        let b_constraint: Vec<PolynomialRing> = (0..k)
            .map(|k_i| calculate_b_constraint(&witness_s, &a_constraint[k_i], &phi_constraint[k_i]))
            .collect();
        println!("b_constraint: {:?}", b_constraint);

        // In DPCS(dot product constraint system) for constant terms(ct), there are k constraints, each constraint has a, phi and b.
        // Generate random a^(l)_{i,j}: l length vector of matrix, matrix length is r x r, each element in matrix is a R_q
        // todo: aij == aji
        let size_l: usize = 5; // Define L as usize
        let a_constraint_ct: Vec<Vec<Vec<PolynomialRing>>> = (0..size_l)
            .map(|_| {
                (0..size_r)
                    .map(|_| {
                        (0..size_r)
                            .map(|_| PolynomialRing {
                                coefficients: (0..size_n).map(|_| rng.gen_range(0..10)).collect(),
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        println!("a_constraint_ct: {:?}", a_constraint_ct);
        // Generate random phi^(k)_{i}, each element is a R_q^{n}
        let phi_constraint_ct: Vec<Vec<PolynomialRing>> = (0..size_l)
            .map(|_| (0..size_n)
                .map(|_| PolynomialRing {
                    coefficients: vec![rng.gen_range(1..5)],
                })
                .collect()
            )
            .collect();
        // calculate b^(l)
        // todo: only need to keep constant term?
        let b_constraint_ct: Vec<PolynomialRing> = (0..size_l)
            .map(|l_i| calculate_b_constraint(&witness_s, &a_constraint_ct[l_i], &phi_constraint_ct[l_i]))
            .collect();
        println!("b_constraint_ct: {:?}", b_constraint_ct);

        let size_kappa = 3; // Example size
        // let size_n = 5;
        // A: matrix size: kappa * n, each element is PolynomialRing(Rq)
        // calculate t_i = A * s_i for all i = 1..r
        // size of t_i = (kappa * n)Rq * 1Rq = kappa * n
        let a_matrix = RqMatrix::new(size_kappa, size_n);
        println!("A: {:?}", a_matrix);
        // print size of A
        println!(
            "size of A: {:?} x {:?}",
            a_matrix.values.len(),
            a_matrix.values[0].len()
        );
        assert!(a_matrix.values.len() == size_kappa);
        assert!(a_matrix.values[0].len() == size_n);
        let mut all_t_i = Vec::new();
        for s_i in &witness_s {
            let t_i = calculate_a_times_s_i(&a_matrix, &s_i);
            println!("size of t_i: {:?}", t_i.len());
            all_t_i.push(t_i);
        }
        println!("Calculated all t_i: {:?}", all_t_i);
        // print size of all_t_i
        println!("size of all_t_i: {:?}", all_t_i.len());
        // check size of all_t_i is kappa
        assert!(all_t_i.len() == size_kappa);

        // ================================================
        // prover
        // 2. GOAL: calculate ajtai commitment (1st outer commitment)
        // 2.1 split t to t_i for all i
        // 2.1.1 get basis b1, refer to paper page 16, labrador c code line 142
        // s = β / sqrt(rnd)
        // r: s_len, n
        // for example:
        // t_0: [
        //  [8, 46, 61, 71, 33, 33, 18], -> t[0][0]
        //  [20, 54, 94, 93, 70, 33, 14], -> t[0][1]
        //  [24, 40, 100, 85, 121, 57, 56],  -> t[0][2]
        //  [14, 37, 91, 118, 159, 109, 72],  -> t[0][3]
        // ]
        // such as basis = 10
        // t1: length of t[i][j][k], such as length of t[0][0][0] = length of [8, 0, 0] = 3
        // Then:
        // t_0_basis_form: [
            // [[8, 0, 0], [6, 4, 0], [1, 6, 0], [1, 7, 0], [3, 3, 0], [3, 3, 0], [8, 1, 0]]
            // [[0, 2, 0], [4, 5, 0], [4, 9, 0], [3, 9, 0], [0, 7, 0], [3, 3, 0], [4, 1, 0]]
            // [[4, 2, 0], [0, 4, 0], [0, 0, 1], [5, 8, 0], [1, 2, 1], [7, 5, 0], [6, 5, 0]]
            // [[4, 1, 0], [7, 3, 0], [1, 9, 0], [8, 1, 1], [9, 5, 1], [9, 0, 1], [2, 7, 0]]
        // ]
        let basis = 10;
        let digits = 3; // t1
        let all_t_i_basis_form: Vec<Vec<Vec<Vec<usize>>>> = all_t_i
            .iter()
            .map(|t_i| {
                t_i.iter()
                    .map(|t_i_j| ring_polynomial_to_basis(t_i_j, basis, digits))
                    .collect::<Vec<Vec<Vec<usize>>>>()
            })
            .collect::<Vec<Vec<Vec<Vec<usize>>>>>();
        println!("all_t_i_basis_form: {:?}", all_t_i_basis_form);
        // print t_0
        println!("t_0: {:?}", all_t_i[0]);
        println!("t_0_basis_form: {:?}", all_t_i_basis_form[0]);
        // pick elements at each position across all inner vectors, put them into a vector
        let mut all_t_i_basis_form_aggregated: Vec<Vec<Vec<PolynomialRing>>> = Vec::new();
        for (i, t_i_basis_form) in all_t_i_basis_form.iter().enumerate() {
            let mut row_results: Vec<Vec<PolynomialRing>> = Vec::new();
            for (j, t_i_j_basis_form) in t_i_basis_form.iter().enumerate() {
                let mut row_results_j: Vec<PolynomialRing> = Vec::new();
                // Get the number of columns from the first inner vector
                // t_i_j_basis_form: [[6, 1, 0], [6, 2, 0], [3, 9, 0], [8, 9, 0], [8, 3, 1], [5, 6, 0], [0, 5, 0]]
                let num_basis_needed = t_i_j_basis_form.len();
                let num_loop_needed = t_i_j_basis_form[0].len();
                for k in 0..num_loop_needed {
                    // println!("t_i_j_basis_form[{}][{}] = {:?}", i, j, t_i_j_basis_form[k]);
                    let mut row_k: Vec<usize> = Vec::new();
                    for basis_needed in 0..num_basis_needed {
                        let num_to_be_pushed = t_i_j_basis_form[basis_needed][k];
                        // println!("t_i_j_basis_form_k[{}][{}]: {:?}", basis_needed, k, num_to_be_pushed);
                        row_k.push(num_to_be_pushed);
                    }
                    row_results_j.push(PolynomialRing {
                        coefficients: row_k,
                    });
                } // finish t_i_j_basis_form calculation
                row_results.push(row_results_j);
            }
            all_t_i_basis_form_aggregated.push(row_results);
        }
        println!(
            "all_t_i_basis_form_aggregated: {:?}",
            all_t_i_basis_form_aggregated
        );
        // 2
        // 2.2.1 get basis b2 same as 2.1.1
        // Start of Selection
        // Calculate g_ij = <s_i, s_j>
        let num_s: usize = witness_s.len();
        let mut g_matrix: Vec<Vec<PolynomialRing>> = vec![
            vec![
                PolynomialRing {
                    coefficients: vec![0; size_n]
                };
                num_s
            ];
            num_s
        ];
        // Calculate b^(k)
        for i in 0..num_s {
            for j in 0..num_s {
                // calculate inner product of s[i] and s[j]
                let elem_s_i = &witness_s[i];
                let elem_s_j = &witness_s[j];
                // Calculate inner product and update b
                let inner_product_si_sj = inner_product_polynomial_ring(&elem_s_i, &elem_s_j);
                g_matrix[i][j] = inner_product_si_sj;
            }
        }
        println!("g_matrix: {:?}", g_matrix);

        // let basis = 10;
        // let digits = 3; // t1
        let g_matrix_basis_form: Vec<Vec<Vec<Vec<usize>>>> = g_matrix
            .iter()
            .map(|g_i| {
                g_i.iter()
                    .map(|g_i_j| ring_polynomial_to_basis(g_i_j, basis, digits))
                    .collect::<Vec<Vec<Vec<usize>>>>()
            })
            .collect::<Vec<Vec<Vec<Vec<usize>>>>>();
        println!("g_matrix_basis_form: {:?}", g_matrix_basis_form);
        // Sum elements at each position across all inner vectors, get t_i and put them into a matrix
        // g is a matrix, each element is a Vec<Rq>(Vec<PolynomialRing>)
        let mut g_matrix_aggregated: Vec<Vec<Vec<PolynomialRing>>> = Vec::new();
        for (i, g_i_basis_form) in g_matrix_basis_form.iter().enumerate() {
            let mut row_results: Vec<Vec<PolynomialRing>> = Vec::new();
            for (j, g_i_j_basis_form) in g_i_basis_form.iter().enumerate() {
                let mut row_results_j: Vec<PolynomialRing> = Vec::new();
                // Get the number of columns from the first inner vector
                // t_i_j_basis_form: [[6, 1, 0], [6, 2, 0], [3, 9, 0], [8, 9, 0], [8, 3, 1], [5, 6, 0], [0, 5, 0]]
                let num_basis_needed = g_i_j_basis_form.len();
                let num_loop_needed = g_i_j_basis_form[0].len();
                for k in 0..num_loop_needed {
                    // println!("t_i_j_basis_form[{}][{}] = {:?}", i, j, t_i_j_basis_form[k]);
                    let mut row_k: Vec<usize> = Vec::new();
                    for basis_needed in 0..num_basis_needed {
                        let num_to_be_pushed = g_i_j_basis_form[basis_needed][k];
                        // println!("t_i_j_basis_form_k[{}][{}]: {:?}", basis_needed, k, num_to_be_pushed);
                        row_k.push(num_to_be_pushed);
                    }
                    row_results_j.push(PolynomialRing {
                        coefficients: row_k,
                    });
                } // finish t_i_j_basis_form calculation
                row_results.push(row_results_j);
            }
            g_matrix_aggregated.push(row_results);
        }

        println!("g_matrix_aggregated: {:?}", g_matrix_aggregated);
        // 2.3 calculate u1
        // 2.3.1 B & C is randomly chosen similar to A
        let size_b = [3, 5];
        let size_c = [3, 5];
        let b_matrix = RqMatrix::new(size_b[0], size_b[1]);
        let c_matrix = RqMatrix::new(size_c[0], size_c[1]);
        // 2.3.2 calculate u1 = sum(B_ik * t_i^(k)) + sum(C_ijk * g_ij^(k))
        // Start of Selection
        // Define necessary variables
        let t1 = digits;
        let t2 = digits;
        let r = size_r;
        let B = &b_matrix.values;
        let C = &c_matrix.values;
        let t = &all_t_i_basis_form_aggregated;
        let kappa = size_r;
        let kappa1 = 5;
        let kappa2 = 5;
        // Initialize u1 with zeros with size kappa1, each element is a polynomial ring
        let mut u1 = vec![
            PolynomialRing {
                coefficients: vec![0; size_n]
            };
            kappa1
        ];
        // B_ik: Rq^{kappa1 x kappa}, t_i: Rq^{kappa}, t_i^(k): Rq^{kappa}
        // B_ik * t_i^(k): Rq^{kappa1}
        // First summation: ∑ B_ik * t_i^(k), 1 ≤ i ≤ r, 0 ≤ k ≤ t1−1
        for i in 0..r {
            for k in 0..t1 {
                let b_i_k = RqMatrix::new(kappa1, kappa).values;
                let t_i_k = &t[i][k];
                // matrix<Rq> * vector<Rq> -> vector<Rq>
                let b_ik_times_t_ik = b_i_k
                    .iter()
                    .map(|row| {
                        row.iter()
                            .zip(t_i_k.iter())
                            .map(|(b, t)| b.multiply_by_polynomial_ring(t))
                            .fold(
                                PolynomialRing {
                                    coefficients: vec![0; size_n],
                                },
                                |acc, val| acc.add_polynomial_ring(&val),
                            )
                    })
                    .collect::<Vec<PolynomialRing>>();
                u1 = u1
                    .iter()
                    .zip(b_ik_times_t_ik.iter())
                    .map(|(a, b)| a.add_polynomial_ring(b))
                    .collect();
            }
        }
        println!("u1: {:?}", u1);

        // Second summation: ∑ C_ijk * g_ij^(k)
        for i in 0..r {
            for j in i..r {
                // i ≤ j
                for k in 0..t2 {
                    let c_i_j_k = RqMatrix::new(kappa2, 1).values;
                    let g_i_j = &g_matrix_aggregated[i][j];
                    let c_i_j_k_times_g_i_j = c_i_j_k
                        .iter()
                        .map(|row| {
                            row.iter()
                                .zip(g_i_j.iter())
                                .map(|(c, g)| c.multiply_by_polynomial_ring(g))
                                .fold(
                                    PolynomialRing {
                                        coefficients: vec![0; size_n],
                                    },
                                    |acc, val| acc.add_polynomial_ring(&val),
                                )
                        })
                        .collect::<Vec<PolynomialRing>>();
                    u1 = u1
                        .iter()
                        .zip(c_i_j_k_times_g_i_j.iter())
                        .map(|(a, b)| a.add_polynomial_ring(b))
                        .collect();
                }
            }
        }
        println!("u1: {:?}", u1);

        // ================================================

        // 3. GOAL: JL projection
        let nd = size_n * deg_bound_d;
        // generate gaussian distribution matrix
        // TODO: should from verifier
        let gaussian_distribution_matrix = generate_gaussian_distribution(nd);
        println!("gaussian_distribution_matrix: {:?}", gaussian_distribution_matrix);
        // 3.1 PI_i is randomly chosen from \Chi { -1, 0, 1 }^{256 * nd}
        //      (Using Guassian Distribution)

        // 3.2 caculate p_j = sum(<pi_i^(j), s_i>) for all i-r
        /*
            - pi^(j) is the j-th row of gaussian_distribution_matrix, j = 1..256
            - concat s_i's coefficients, output a vector with length nd
            - <pi_i^(j), s_i> = pi_i^(j)[0] * s_i[0] + pi_i^(j)[1] * s_i[1] + ... + pi_i^(j)[nd-1] * s_i[nd-1], is the inner product of pi_i^(j) and s_i
            - p_j = sum(<pi_i^(j), s_i>) for all i = 1..r, is a Zq
            - vector p = [p_1, p_2, ..., p_256], is a vector with length 256(2λ), type Vec<Zq>
        */

        // concat all s_i's coefficients into a vector
        // such as: s = [s_0, s_1]
        // s_0: [PolynomialRing{coefficients: [1, 2, 3]}, PolynomialRing{coefficients: [4, 5, 6]}, PolynomialRing{coefficients: [7, 8, 9]}], output: ss_0 = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        // s_1: [PolynomialRing{coefficients: [1, 2, 3]}, PolynomialRing{coefficients: [4, 5, 6]}, PolynomialRing{coefficients: [7, 8, 9]}], output: ss_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        // so ss = [ss_0, ss_1]
        let s_coeffs: Vec<Vec<usize>> = witness_s.iter()
            .map(|s_i| s_i.iter().map(|s_i_poly| s_i_poly.coefficients.clone()).flatten().collect())
            .collect();

        println!("s_coeffs: {:?}", s_coeffs);
        // implement p calculation, inner product each element of gaussian_distribution_matrix and each element of s_coeffs
        let mut p: Vec<usize> = Vec::with_capacity(256);
        for g_row in gaussian_distribution_matrix.iter() {
            println!("g_row: {:?}", g_row);
            let mut sum = 0;
            for s_row in s_coeffs.iter() {
                sum += g_row.iter().zip(s_row.iter()).map(|(a, b)| *a * *b as i32).sum::<i32>();
            }
            p.push(sum as usize);
        }
        println!("p: {:?}", p);
        assert_eq!(p.len(), 256);
        // todo: send p to verifier(put in transcript)

        // 3.3 Verifier have to check: || p || <= \sqrt{128} * beta

        // ================================================

        // 4. GOAL: Aggregation
        // 4.1 psi^(k) is randomly chosen from Z_q^{L}
        // k = 1..λ/log2^q
        let size_k = lambda / log_q;
        let psi_challenge = (0..size_k)
            .map(|_| (0..size_l).map(|_| rng.gen_range(0..10)).collect())
            .collect::<Vec<Vec<usize>>>();
        assert_eq!(psi_challenge.len(), size_k);
        assert_eq!(psi_challenge[0].len(), size_l);

        // 4.2 omega^(k) is randomly chosen from Z_q^{256}
        //      (Both using Guassian Distribution)
        let omega_challenge = (0..size_k).map(|_| (0..double_lambda).map(|_| rng.gen_range(0..10)).collect()).collect::<Vec<Vec<usize>>>();
        assert_eq!(omega_challenge.len(), size_k);
        assert_eq!(omega_challenge[0].len(), double_lambda);

        // 4.3 caculate b^{''(k)}
        // 4.3.1 calculate a_ij^{''(k)} = sum(psi_l^(k) * a_ij^{'(l)}) for all l = 1..L
        // a_l, phi_l
        // for k from 1 to L
        // calculate a_ij^{''(k)} = sum(psi_l^(k) * a_ij^{'(l)}) for all l = 1..L
        let aggregated_a_l: Vec<usize> = (0..k)
            .map(|k_i| {
                let psi_ki = &psi_k[k_i];
                    // Start of Selection
                    let mut sum = 0;
                    for l_i in 0..l {
                        let psi_ki_l = psi_ki[l_i];
                        for i in 0..r {
                            for j in i..r {
                                sum += a_l[i][j] * psi_ki_l;
                            }
                }
            }
            sum
        }).collect();
        println!("aggregated_a_l: {:?}", aggregated_a_l);
        assert_eq!(aggregated_a_l.len(), k);
        // 4.3.2 calculate phi_i^{''(k)} =
        //       sum(psi_l^(k) * phi_i^{'(l)}) for all l = 1..L
        //       + sum(omega_j^(k) * sigma_{-1} * pi_i^{j)) for all j = 1..256
        // 4.3.3 calculate b^{''(k)} = sum(a_ij^{''(k)} * <s_i, s_j>) + sum(<phi_i^{''(k)}, s_i>)

        // Send b_0^{''(k)} to verifier
        // Verifier check: b_0^{''(k)} ?= <⟨omega^(k),p⟩> + sum(psi_l^(k) * b_0^{'(l)}) for all l = 1..L

        // ================================================

        // 5. GOAL: Calculate u2 (2nd outer commitment)
        // 5.1 vec<alpha> and vec<beta> are randomly chosen from R_q^{128/logQ} // why is this not `L`
        // 5.2 phi_i = sum(alpha_k * phi_i) + beta_k * phi_i^{''(k)}
        // 5.3 h_ij = 1/2 * (<phi_i, s_j> + <phi_j, s_i>)
        // 5.4 u2 = sum D_ij * h_ij^(k) for all k = 1..(t1-1)

        // Send u2 to verifier

        // ================================================

        // 6. GOAL: calculate z (Amortized Opening)
        // 6.1 c_i is randomly chosen from C
        // 6.2 calculate z = sum(c_i * s_i) for all i = 1..r
        // Send z, t_i, g_ij, h_ij to verifier
    }

    #[test]
    fn test_multiply_by_polynomial_ring() {
        let poly1 = PolynomialRing {
            coefficients: vec![1, 2],
        };
        let poly2 = PolynomialRing {
            coefficients: vec![3, 4],
        };
        let result = poly1.multiply_by_polynomial_ring(&poly2);
        assert_eq!(result.coefficients, vec![3, 10, 8]); // 1*3, 1*4 + 2*3, 2*4
    }

    #[test]
    fn test_calculate_b_k() {
        let r: usize = 3;
        let s: Vec<Vec<PolynomialRing>> = vec![
            vec![
                PolynomialRing {
                    coefficients: vec![1, 2, 3],
                },
                PolynomialRing {
                    coefficients: vec![4, 5, 6],
                },
            ],
            vec![
                PolynomialRing {
                    coefficients: vec![7, 8, 9],
                },
                PolynomialRing {
                    coefficients: vec![10, 11, 12],
                },
            ],
            vec![
                PolynomialRing {
                    coefficients: vec![13, 14, 15],
                },
                PolynomialRing {
                    coefficients: vec![16, 17, 18],
                },
            ],
        ];
        let k: usize = 6;
        let a_constraint: Vec<Vec<PolynomialRing>> = (0..k).map(|_| {
            (0..r).map(|r_i| PolynomialRing {
                coefficients: vec![r_i],
            }).collect()
        }).collect();
        let phi_constraint: Vec<PolynomialRing> = (0..r).map(|r_i| PolynomialRing {
            coefficients: vec![r_i],
        }).collect();
        let b_k = calculate_b_constraint(&s, &a_constraint, &phi_constraint);
        println!("b_k: {:?}", b_k);
        // assert_eq!(b_k, 1983);
    }

    #[test]
    fn test_a_new() {
        let size: usize = 3;
        let a = RqMatrix::new(size, size);
        assert_eq!(a.values.len(), size);
        assert_eq!(a.values[0].len(), size);
    }

    #[test]
    fn test_calculate_a_times_s_i() {
        let a = RqMatrix::new(2, 2);
        let s_i = vec![
            PolynomialRing {
                coefficients: vec![1, 2],
            },
            PolynomialRing {
                coefficients: vec![3, 4],
            },
        ];
        let result = calculate_a_times_s_i(&a, &s_i);
        assert_eq!(result.len(), a.values.len() * s_i.len()); // Check that the result length is correct
    }

    #[test]
    fn test_num_to_basis() {
        let num = 42;
        let basis = 2;
        let digits = 6;
        let binary = num_to_basis(num, basis, digits);
        assert_eq!(binary, vec![0, 1, 0, 1, 0, 1]);

        let num = 100;
        let basis = 3;
        let digits = 6;
        let binary = num_to_basis(num, basis, digits);
        assert_eq!(binary, vec![1, 0, 2, 0, 1, 0]);

        let num = 100;
        let basis = 6;
        let digits = 6;
        let binary = num_to_basis(num, basis, digits);
        assert_eq!(binary, vec![4, 4, 2, 0, 0, 0]);

        let num = 100;
        let basis = 10;
        let digits = 6;
        let binary = num_to_basis(num, basis, digits);
        assert_eq!(binary, vec![0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_basis_to_num_vector() {
        let basis = 10;
        let digits = 3;
        let vec1 = [8, 46, 61, 71, 33, 33, 18];
        let vec2 = [20, 54, 94, 93, 70, 33, 14];
        let vec3 = [24, 40, 100, 85, 121, 57, 56];
        let vec4 = [14, 37, 91, 118, 159, 109, 72];
        for vec in [vec1, vec2, vec3, vec4] {
            let mut temp_vec: Vec<Vec<usize>> = Vec::new();
            for i in vec {
                let num = num_to_basis(i, basis, digits);
                println!("num: {:?}", num);
                temp_vec.push(num);
            }
            println!("temp_vec for vec {:?}: {:?}", vec, temp_vec);
        }
    }

    #[test]
    fn test_ring_polynomial_to_basis() {
        let poly = PolynomialRing {
            coefficients: vec![42, 100, 100],
        };
        let basis = 2;
        let digits = 8;
        let expected_result = vec![
            vec![0, 1, 0, 1, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0],
        ];
        let result = ring_polynomial_to_basis(&poly, basis, digits);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_inner_product_polynomial_ring() {
        let a = vec![
            PolynomialRing {
                coefficients: vec![1, 2, 3],
            },
            PolynomialRing {
                coefficients: vec![4, 5, 6],
            },
        ];
        let b = vec![
            PolynomialRing {
                coefficients: vec![7, 8, 9],
            },
            PolynomialRing {
                coefficients: vec![10, 11, 12],
            },
        ];

        let result = inner_product_polynomial_ring(&a, &b);

        // Expected result calculation:
        // (1 + 2x + 3x^2) * (7 + 8x + 9x^2) = 7 + 22x + 46x^2 + 42x^3 + 27x^4
        // (4 + 5x + 6x^2) * (10 + 11x + 12x^2) = 40 +  96x + 163x^2 + 126x^3 + 72x^4
        // Sum: 47 + 116x + 209x^2 + 168x^3 + 99x^4

        let expected = PolynomialRing {
            coefficients: vec![47, 116, 209, 168, 99],
        };

        assert_eq!(result.coefficients, expected.coefficients);
    }

    #[test]
    fn test_generate_gaussian_distribution() {
        let nd = 10;
        let matrix = generate_gaussian_distribution(nd);
        println!("matrix: {:?}", matrix);
        assert_eq!(matrix.len(), 256);
        assert_eq!(matrix[0].len(), nd);
        assert_eq!(matrix[1].len(), nd);
        assert!(matrix.iter().all(|row| row.iter().all(|&val| val == -1 || val == 0 || val == 1)));

    }

    // add test for polynomial addition and multiplication with overload
    #[test]
    fn test_polynomial_addition_and_multiplication() {
        let a = PolynomialRing {
            coefficients: vec![1, 2, 3],
        };
        let b = PolynomialRing {
            coefficients: vec![4, 5, 6],
        };
        let c = &a + &b;
        assert_eq!(c.coefficients, vec![5, 7, 9]);
        let d = &a * &b;
        assert_eq!(d.coefficients, vec![4, 13, 28, 27, 18]);
    }
}

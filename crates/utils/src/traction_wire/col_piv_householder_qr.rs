//! Replacement for Eigen's `ColPivHouseholderQR` (`A.colPivHouseholderQr().solve(b)`), which
//! Circuit.cpp uses for every dense solve. There is no C++ file for this one: Eigen is not a
//! dependency of this port, so Eigen 3.4.0 is written out here with its exact evaluation order
//! (sources: ColPivHouseholderQR.h computeInPlace/_solve_impl, Householder.h, Redux.h,
//! products/GeneralMatrixVector.h, products/TriangularSolverVector.h).
//! The official sumo binary evaluates every vectorised reduction with SSE2 packets of two doubles and
//! fuses the explicit `pmadd` of Eigen's gemv kernels into one FMA (measured: this order reproduces its
//! rank-deficient circuit solutions bit for bit, packets of four doubles or no FMA do not; fusing the
//! outer-product update or other plain expressions does not either, so only `pmadd` is fused). On well-conditioned systems the result is the same to
//! rounding; on the rank-deficient overhead-wire circuits only this order gives its solutions.

use std::ops::Range;

/// Dense row-major matrix, stand-in for `Eigen::MatrixXd`.
#[derive(Clone, Debug, PartialEq)]
pub struct DenseMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl DenseMatrix {
    pub fn from_row_major(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), rows * cols, "matrix data does not match its shape");
        Self { rows, cols, data }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.cols + j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.data[i * self.cols + j]
    }

    /// Circuit.cpp `removeColumn`: drops column `col`, the other columns keep their order.
    pub fn remove_column(&mut self, col: usize) {
        let mut data = Vec::with_capacity(self.rows * (self.cols - 1));
        for i in 0..self.rows {
            for j in (0..self.cols).filter(|&j| j != col) {
                data.push(self.get(i, j));
            }
        }
        self.cols -= 1;
        self.data = data;
    }

    /// `A * x` as Eigen's column-major gemv: every row is summed over the columns left to right.
    pub fn mul_vec(&self, x: &[f64]) -> Vec<f64> {
        (0..self.rows)
            .map(|i| {
                let mut s = 0.0;
                for j in 0..self.cols {
                    s += self.get(i, j) * x[j];
                }
                s
            })
            .collect()
    }

    fn swap_columns(&mut self, a: usize, b: usize) {
        for i in 0..self.rows {
            self.data.swap(i * self.cols + a, i * self.cols + b);
        }
    }
}

/// Eigen `redux_impl<scalar_sum_op, ..., LinearVectorizedTraversal, NoUnrolling>` (Redux.h) with
/// SSE2 packets of two doubles. Expressions such as `cwiseAbs2()` have no direct access, so the
/// aligned start is always 0: two packet accumulators over the first `size & !1` terms, the lanes
/// added, then the remaining term (if any) added one by one.
fn redux_sum(terms: &[f64]) -> f64 {
    let size = terms.len();
    if size == 0 {
        return 0.0;
    }
    let aligned = (size / 2) * 2;
    let aligned2 = (size / 4) * 4;
    if aligned == 0 {
        let mut res = terms[0];
        for &t in &terms[1..] {
            res += t;
        }
        return res;
    }
    let mut r0 = [terms[0], terms[1]];
    if aligned > 2 {
        let mut r1 = [terms[2], terms[3]];
        let mut idx = 4;
        while idx < aligned2 {
            r0 = [r0[0] + terms[idx], r0[1] + terms[idx + 1]];
            r1 = [r1[0] + terms[idx + 2], r1[1] + terms[idx + 3]];
            idx += 4;
        }
        r0 = [r0[0] + r1[0], r0[1] + r1[1]];
        if aligned > aligned2 {
            r0 = [r0[0] + terms[aligned2], r0[1] + terms[aligned2 + 1]];
        }
    }
    let mut res = r0[0] + r0[1];
    for &t in &terms[aligned..] {
        res += t;
    }
    res
}

/// Eigen `squaredNorm()`: sum of the squares through `redux_sum`.
fn sq_norm(v: &[f64]) -> f64 {
    let squares: Vec<f64> = v.iter().map(|t| t * t).collect();
    redux_sum(&squares)
}

/// Eigen `norm()` of a vector: `sqrt(squaredNorm())`. Circuit.cpp uses it for `(A * x - b).norm()`.
pub fn eigen_norm(v: &[f64]) -> f64 {
    sq_norm(v).sqrt()
}

/// Euclidean norm of column `j` from row `start` down (Eigen `col(j).tail(..).norm()`).
fn col_norm(m: &DenseMatrix, j: usize, start: usize) -> f64 {
    let v: Vec<f64> = (start..m.rows()).map(|i| m.get(i, j)).collect();
    eigen_norm(&v)
}

/// One element of Eigen's row-major gemv kernel (`essential^T * column`, GeneralMatrixVector.h):
/// two packet lanes accumulate the even/odd products, the lanes are added, then the remaining
/// product is added; `res = 0 + alpha * cc` with alpha = 1.
fn gemv_dot(e: &[f64], col: &[f64]) -> f64 {
    let m = e.len();
    let mut lanes = [0.0f64; 2];
    let mut j = 0;
    while j + 2 <= m {
        // pmadd(a, b, c) of the packet kernel: one fused multiply-add per lane
        lanes[0] = e[j].mul_add(col[j], lanes[0]);
        lanes[1] = e[j + 1].mul_add(col[j + 1], lanes[1]);
        j += 2;
    }
    let mut cc = lanes[0] + lanes[1];
    while j < m {
        cc += e[j] * col[j];
        j += 1;
    }
    0.0 + 1.0 * cc
}

/// Eigen `makeHouseholderInPlace` on the vector (`c0`, `tail`) (Householder.h).
/// Returns (tau, beta, essential), where `essential` replaces `tail` in the factored matrix.
fn make_householder(c0: f64, tail: &[f64]) -> (f64, f64, Vec<f64>) {
    let tail_sq_norm = if tail.is_empty() { 0.0 } else { sq_norm(tail) };
    if tail_sq_norm <= f64::MIN_POSITIVE {
        return (0.0, c0, vec![0.0; tail.len()]);
    }
    let mut beta = (c0 * c0 + tail_sq_norm).sqrt();
    if c0 >= 0.0 {
        beta = -beta;
    }
    let essential = tail.iter().map(|t| t / (c0 - beta)).collect();
    let tau = (beta - c0) / beta;
    (tau, beta, essential)
}

/// Eigen `applyHouseholderOnTheLeft` of H = I - tau v v^T, v = (1, essential), applied to the
/// rows k.. of the columns `cols` of `m` (Householder.h):
/// `tmp = essential^T * bottom; tmp += row(0); row(0) -= tau * tmp; bottom -= (tau * essential) * tmp`.
fn apply_householder_on_left(m: &mut DenseMatrix, k: usize, essential: &[f64], tau: f64, cols: Range<usize>) {
    if tau == 0.0 || essential.is_empty() {
        return;
    }
    for j in cols {
        let colv: Vec<f64> = (0..essential.len()).map(|i| m.get(k + 1 + i, j)).collect();
        let mut tmp = gemv_dot(essential, &colv);
        tmp += m.get(k, j);
        let top = m.get(k, j);
        *m.get_mut(k, j) = top - tau * tmp;
        for (i, e) in essential.iter().enumerate() {
            let cur = m.get(k + 1 + i, j);
            *m.get_mut(k + 1 + i, j) = cur - (tau * e) * tmp;
        }
    }
}

/// Solves `a * x = b` for square `a` as Eigen's `ColPivHouseholderQR::solve` does: components
/// beyond the numerical rank (a column whose remaining norm is below
/// `abs2(maxColNorm * eps) / rows`, scaled by `rows - k`) are zero.
pub fn col_piv_householder_qr_solve(a: &DenseMatrix, b: &[f64]) -> Vec<f64> {
    let rows = a.rows();
    let cols = a.cols();
    let size = rows.min(cols);
    let eps = f64::EPSILON;
    assert_eq!(b.len(), rows, "right-hand side does not match the matrix rows");

    let mut qr = a.clone();
    let mut hcoeffs = vec![0.0; size];
    // perm[i] is the original column at position i (Eigen m_colsPermutation.indices())
    let mut perm: Vec<usize> = (0..cols).collect();

    let mut col_norms_updated: Vec<f64> = (0..cols).map(|j| col_norm(&qr, j, 0)).collect();
    let mut col_norms_direct = col_norms_updated.clone();
    let max_norm = col_norms_updated.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    // Eigen 3.4.0 ColPivHouseholderQR.h:511 computeInPlace: abs2(maxCoeff * epsilon) / rows
    let threshold_helper = (max_norm * eps) * (max_norm * eps) / rows as f64;
    let norm_downdate_threshold = eps.sqrt();
    // number of meaningful pivots (m_nonzero_pivots), fixed at the first column whose norm is too small
    let mut nonzero_pivots = size;

    for k in 0..size {
        // column with the largest remaining norm, first one on ties (Eigen maxCoeff)
        let mut biggest = k;
        for j in k + 1..cols {
            if col_norms_updated[j] > col_norms_updated[biggest] {
                biggest = j;
            }
        }
        if nonzero_pivots == size && col_norms_updated[biggest] * col_norms_updated[biggest] < threshold_helper * (rows - k) as f64 {
            nonzero_pivots = k;
        }
        if biggest != k {
            qr.swap_columns(k, biggest);
            col_norms_updated.swap(k, biggest);
            col_norms_direct.swap(k, biggest);
            perm.swap(k, biggest);
        }

        let tail: Vec<f64> = (k + 1..rows).map(|i| qr.get(i, k)).collect();
        let (tau, beta, essential) = make_householder(qr.get(k, k), &tail);
        hcoeffs[k] = tau;
        for (idx, i) in (k + 1..rows).enumerate() {
            *qr.get_mut(i, k) = essential[idx];
        }
        *qr.get_mut(k, k) = beta;
        apply_householder_on_left(&mut qr, k, &essential, tau, k + 1..cols);

        // downdate the remaining column norms, recomputing them when cancellation makes them unreliable
        for j in k + 1..cols {
            if col_norms_updated[j] != 0.0 {
                let mut temp = qr.get(k, j).abs() / col_norms_updated[j];
                temp = (1.0 + temp) * (1.0 - temp);
                if temp < 0.0 {
                    temp = 0.0;
                }
                let ratio = col_norms_updated[j] / col_norms_direct[j];
                let temp2 = temp * (ratio * ratio);
                if temp2 <= norm_downdate_threshold {
                    col_norms_direct[j] = col_norm(&qr, j, k + 1);
                    col_norms_updated[j] = col_norms_direct[j];
                } else {
                    col_norms_updated[j] *= temp.sqrt();
                }
            }
        }
    }

    let rank = nonzero_pivots;
    if rank == 0 {
        return vec![0.0; cols];
    }

    // c = Q^T b with householderQ().setLength(nonzero_pivots): only the first `rank` reflectors, each
    // through applyHouseholderOnTheLeft on the vector (inner product through redux)
    let mut c = b.to_vec();
    for k in 0..rank {
        let tau = hcoeffs[k];
        let ess: Vec<f64> = (k + 1..rows).map(|i| qr.get(i, k)).collect();
        if ess.is_empty() {
            c[k] *= 1.0 - tau;
            continue;
        }
        if tau == 0.0 {
            continue;
        }
        let products: Vec<f64> = ess.iter().enumerate().map(|(idx, e)| e * c[k + 1 + idx]).collect();
        let mut tmp = redux_sum(&products);
        tmp += c[k];
        c[k] -= tau * tmp;
        for (idx, e) in ess.iter().enumerate() {
            c[k + 1 + idx] -= (tau * e) * tmp;
        }
    }

    // back substitution with the leading rank x rank block of R: Eigen triangular_solve_vector
    // (column-major, Upper): panels of 8, column oriented inside a panel, a column-major gemv for the rows above
    const PANEL_WIDTH: usize = 8;
    let mut pi = rank;
    while pi > 0 {
        let w = pi.min(PANEL_WIDTH);
        let start = pi - w;
        for kk in 0..w {
            let i = pi - kk - 1;
            if c[i] != 0.0 {
                c[i] /= qr.get(i, i);
                let rem = w - kk - 1;
                let s = i - rem;
                for t in 0..rem {
                    c[s + t] -= c[i] * qr.get(s + t, i);
                }
            }
        }
        // general_matrix_vector_product<ColMajor>: rows are processed in packets of two (pmadd, fused);
        // a remaining odd last row is scalar (`c0 += a * b`, not fused)
        let packet_rows = (start / 2) * 2;
        for r in 0..start {
            let mut acc = 0.0;
            for jcol in start..pi {
                acc = if r < packet_rows {
                    qr.get(r, jcol).mul_add(c[jcol], acc)
                } else {
                    qr.get(r, jcol) * c[jcol] + acc
                };
            }
            c[r] += -1.0 * acc;
        }
        pi -= w;
    }

    let mut x = vec![0.0; cols];
    for i in 0..rank {
        x[perm[i]] = c[i];
    }
    if let Ok(t) = std::env::var("OC95_DUMP_TARGET") {
        // probe only (removed before the final commit): solver calls whose solution contains the target value
        if let Ok(t) = t.parse::<f64>() {
            if x.iter().any(|v| (v - t).abs() <= 5.1e-5) {
                eprintln!("OC95Q {} rank={} {:?} {:?} {:?}", rows, rank, a.data, b, x);
            }
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_small_dense_system() {
        // 2x + y = 3, x + 3y = 5
        let a = DenseMatrix::from_row_major(2, 2, vec![2.0, 1.0, 1.0, 3.0]);
        let x = col_piv_householder_qr_solve(&a, &[3.0, 5.0]);
        assert!((x[0] - 0.8).abs() < 1e-12, "x0 = {}", x[0]);
        assert!((x[1] - 1.4).abs() < 1e-12, "x1 = {}", x[1]);
    }

    #[test]
    fn pivots_when_leading_entry_is_zero() {
        // y = 2, x = 3
        let a = DenseMatrix::from_row_major(2, 2, vec![0.0, 1.0, 1.0, 0.0]);
        let x = col_piv_householder_qr_solve(&a, &[2.0, 3.0]);
        assert!((x[0] - 3.0).abs() < 1e-12);
        assert!((x[1] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn rank_deficient_columns_get_zero() {
        // x + y = 2 with a rank-1 matrix: the first pivot column is used, the second gets 0
        let a = DenseMatrix::from_row_major(2, 2, vec![1.0, 1.0, 1.0, 1.0]);
        let x = col_piv_householder_qr_solve(&a, &[2.0, 2.0]);
        assert!((x[0] - 2.0).abs() < 1e-12, "x0 = {}", x[0]);
        assert_eq!(x[1], 0.0);
    }

    #[test]
    fn rank_threshold_has_no_diag_size_factor() {
        // Eigen 3.4.0: threshold_helper = abs2(1 * eps) / 2 = 2.465e-32, so the column norm 2.5e-16
        // (squared 6.25e-32) is above it and counts as a nonzero pivot: x = [1, 1 / 2.5e-16].
        let a = DenseMatrix::from_row_major(2, 2, vec![1.0, 0.0, 0.0, 2.5e-16]);
        let x = col_piv_householder_qr_solve(&a, &[1.0, 1.0]);
        assert_eq!(x[0], 1.0);
        assert!((x[1] / 4.0e15 - 1.0).abs() < 1e-12, "x1 = {}", x[1]);
    }

    #[test]
    fn redux_sum_uses_two_packet_lanes() {
        // size 4: lane0 = t0 + t2, lane1 = t1 + t3, then the lanes are added (Redux.h, two packets of two)
        let t = [1.0, 1.5e-16, 1.0, 1.5e-16];
        let sequential = ((1.0 + 1.5e-16) + 1.0) + 1.5e-16; // rounds to 2.0
        let lanes = (1.0 + 1.0) + (1.5e-16 + 1.5e-16); // 2.0000000000000004
        assert_eq!(redux_sum(&t), lanes);
        assert_ne!(redux_sum(&t), sequential);
    }

    /// Captured solver call of the elecHybrid_overhead circuit (rank-deficient, 8 x 8). The official
    /// sumo binary prints circuitVoltage -5.3387 for x[6]; the evaluation order of Eigen gives it.
    #[test]
    fn overhead_wire_circuit_matches_eigen() {
        #[rustfmt::skip]
        let a = vec![
            0.0, 85.81345854017509, 0.0, 0.0, -22.11283738661643, -21.775486028848164, 0.0, -41.9251351247105,
            0.0, 0.0, 79.63964652739284, 0.0, 0.0, 0.0, -79.63964652739284, 0.0,
            0.0, 0.0, 0.0, 18.10169678064943, 0.0, 0.0, -18.10169678064943, 0.0,
            0.0, -22.11283738661643, 0.0, 0.0, 22.11283738661643, 0.0, 0.0, 0.0,
            0.0, -21.775486028848164, 0.0, 0.0, 0.0, 21.775486028848164, 0.0, 0.0,
            0.0, 0.0, -79.63964652739284, -18.10169678064943, 0.0, 0.0, 97.74134330804227, 0.0,
            -22.754184722112246, -41.9251351247105, 0.0, 0.0, 0.0, 0.0, 0.0, 64.67931984682275,
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let b = [0.0, 0.0, 0.0, 0.0, 0.0, -289.9208221058896, -289.9208221058896, 600.0];
        let x = col_piv_householder_qr_solve(&DenseMatrix::from_row_major(8, 8, a), &b);
        // values from the offline emulation of Eigen 3.4.0 with SSE2 packets and fused pmadd (oc95 probe)
        assert_eq!(x[3], 0.0);
        assert_eq!(x[6], -5.338741180252343);
        assert_eq!(x[2], -4.1252717809116835);
        assert_eq!(x[0], 600.0000000000007);
    }

    /// Captured initial solve of the same circuit a few steps later: nearly singular, the solution is dominated
    /// by rounding (~1.56e16). The official sumo binary prints circuitVoltage 15627729533009616 (test1) and
    /// 15627729533009614 (test3) for x[6] and x[7]; only SSE2 packets with fused pmadd give those digits.
    #[test]
    fn near_singular_overhead_wire_circuit_matches_eigen_digits() {
        #[rustfmt::skip]
        let a = vec![
            -14.749262536873157, 58.63758595233775, 0.0, 0.0, -22.11283738661643, -21.775486028848164, 0.0, 0.0,
            0.0, 0.0, 53.09309768492857, 0.0, 0.0, 0.0, 0.0, -53.09309768492857,
            0.0, 0.0, 0.0, 56.903019046578486, 0.0, 0.0, -56.903019046578486, 0.0,
            0.0, -22.11283738661643, 0.0, 0.0, 22.11283738661643, 0.0, 0.0, 0.0,
            0.0, -21.775486028848164, 0.0, 0.0, 0.0, 21.775486028848164, 0.0, 0.0,
            0.0, 0.0, 0.0, -56.903019046578486, 0.0, 0.0, 88.75887765753563, -31.855858610957156,
            0.0, 0.0, -53.09309768492857, 0.0, 0.0, 0.0, -31.855858610957156, 84.94895629588572,
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let b = [0.0, 0.0, 0.0, 0.0, 0.0, -289.9208221058896, -289.9208221058896, 600.0];
        let x = col_piv_householder_qr_solve(&DenseMatrix::from_row_major(8, 8, a), &b);
        assert_eq!(x[6], 15627729533009616.0);
        assert_eq!(x[7], 15627729533009614.0);
        assert_eq!(x[2], 15627729533009612.0);
        assert_eq!(x[3], 15627729533009618.0);
    }

    #[test]
    fn remove_column_keeps_order() {
        let mut a = DenseMatrix::from_row_major(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.remove_column(1);
        assert_eq!(a, DenseMatrix::from_row_major(2, 2, vec![1.0, 3.0, 4.0, 6.0]));
    }
}

//! Replacement for Eigen's `ColPivHouseholderQR` (`A.colPivHouseholderQr().solve(b)`), which
//! Circuit.cpp uses for every dense solve. There is no C++ file for this one: Eigen is not a
//! dependency of this port, so the Eigen 3.4 unblocked algorithm is written out here:
//! Householder QR with column pivoting, rank threshold `diagSize * eps` relative to the largest
//! pivot, and a solve that sets the components beyond the numerical rank to zero.
//! Not bit-exact with Eigen's vectorised/blocked kernels, only the same algorithm.

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

    /// `A * x`, every row summed left to right.
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

/// Euclidean norm of column `j` from row `start` down (Eigen `col(j).tail(..).norm()`).
fn col_norm(m: &DenseMatrix, j: usize, start: usize) -> f64 {
    let mut sq = 0.0;
    for i in start..m.rows() {
        sq += m.get(i, j) * m.get(i, j);
    }
    sq.sqrt()
}

/// Eigen `makeHouseholderInPlace` on the vector (`c0`, `tail`).
/// Returns (tau, beta, essential), where `essential` replaces `tail` in the factored matrix.
fn make_householder(c0: f64, tail: &[f64]) -> (f64, f64, Vec<f64>) {
    let tail_sq_norm: f64 = tail.iter().map(|t| t * t).sum();
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
/// rows k.. of the columns `cols` of `m`.
fn apply_householder_on_left(m: &mut DenseMatrix, k: usize, essential: &[f64], tau: f64, cols: Range<usize>) {
    if tau == 0.0 {
        return;
    }
    for j in cols {
        let mut s = 0.0;
        for (i, e) in essential.iter().enumerate() {
            s += e * m.get(k + 1 + i, j);
        }
        let w = s + m.get(k, j);
        *m.get_mut(k, j) -= tau * w;
        let tw = tau * w;
        for (i, e) in essential.iter().enumerate() {
            *m.get_mut(k + 1 + i, j) -= e * tw;
        }
    }
}

/// Solves `a * x = b` for square `a` as Eigen's `ColPivHouseholderQR::solve` does: components
/// beyond the numerical rank (pivot below `diagSize * eps` times the largest pivot) are zero.
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
    let threshold_helper = (max_norm * eps) * (max_norm * eps) / rows as f64;

    for k in 0..size {
        // column with the largest remaining norm, first one on ties (Eigen maxCoeff)
        let mut biggest = k;
        for j in k + 1..cols {
            if col_norms_updated[j] > col_norms_updated[biggest] {
                biggest = j;
            }
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
                temp = 1.0 - temp * temp;
                if temp < 0.0 {
                    temp = 0.0;
                }
                let ratio = col_norms_updated[j] / col_norms_direct[j];
                let temp2 = temp * (ratio * ratio);
                if temp2 <= threshold_helper {
                    col_norms_direct[j] = col_norm(&qr, j, k + 1);
                    col_norms_updated[j] = col_norms_direct[j];
                } else {
                    col_norms_updated[j] *= temp.sqrt();
                }
            }
        }
    }

    // numerical rank: first diagonal entry at or below diagSize * eps times the largest one
    let threshold = size as f64 * eps;
    let mut max_pivot = 0.0_f64;
    for k in 0..size {
        max_pivot = max_pivot.max(qr.get(k, k).abs());
    }
    let mut rank = size;
    for k in 0..size {
        if qr.get(k, k).abs() <= threshold * max_pivot {
            rank = k;
            break;
        }
    }

    // c = Q^T b, applying the reflectors in order H_0, H_1, ...
    let mut c = b.to_vec();
    for k in 0..size {
        let tau = hcoeffs[k];
        if tau == 0.0 {
            continue;
        }
        let mut s = 0.0;
        for i in k + 1..rows {
            s += qr.get(i, k) * c[i];
        }
        let w = s + c[k];
        c[k] -= tau * w;
        let tw = tau * w;
        for i in k + 1..rows {
            c[i] -= qr.get(i, k) * tw;
        }
    }

    // back substitution with the leading rank x rank block of R
    for i in (0..rank).rev() {
        let mut s = c[i];
        for j in i + 1..rank {
            s -= qr.get(i, j) * c[j];
        }
        c[i] = s / qr.get(i, i);
    }

    let mut x = vec![0.0; cols];
    for i in 0..rank {
        x[perm[i]] = c[i];
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
    fn remove_column_keeps_order() {
        let mut a = DenseMatrix::from_row_major(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        a.remove_column(1);
        assert_eq!(a, DenseMatrix::from_row_major(2, 2, vec![1.0, 3.0, 4.0, 6.0]));
    }
}

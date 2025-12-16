use super::{data::Data, Matrix};

fn assert_f32_near(actual: f32, expected: f32, eps: f32) {
	assert!(
		(actual - expected).abs() <= eps,
		"expected {expected}, got {actual} (eps={eps})"
	);
}

#[test]
fn test_new_from_cols_and_indexing() {
	// Internal storage is column-major: cols[c][r]
	let m: Matrix<2, 3> = Matrix::new([
		[1.0, 2.0], // col 0
		[3.0, 4.0], // col 1
		[5.0, 6.0], // col 2
	]);
	assert_f32_near(m[(0, 0)], 1.0, 1e-6);
	assert_f32_near(m[(1, 0)], 2.0, 1e-6);
	assert_f32_near(m[(0, 1)], 3.0, 1e-6);
	assert_f32_near(m[(1, 1)], 4.0, 1e-6);
	assert_f32_near(m[(0, 2)], 5.0, 1e-6);
	assert_f32_near(m[(1, 2)], 6.0, 1e-6);
}

#[test]
fn test_from_rows_layout() {
	// Rows are row-major: rows[r][c]
	let m: Matrix<2, 3> = Matrix::from_rows([
		[1.0, 3.0, 5.0],
		[2.0, 4.0, 6.0],
	]);
	assert_f32_near(m[(0, 0)], 1.0, 1e-6);
	assert_f32_near(m[(1, 0)], 2.0, 1e-6);
	assert_f32_near(m[(0, 1)], 3.0, 1e-6);
	assert_f32_near(m[(1, 1)], 4.0, 1e-6);
	assert_f32_near(m[(0, 2)], 5.0, 1e-6);
	assert_f32_near(m[(1, 2)], 6.0, 1e-6);
}

#[test]
fn test_zeros_and_index_mut() {
	let mut m: Matrix<2, 2> = Matrix::zeros();
	assert_f32_near(m[(0, 0)], 0.0, 1e-6);
	assert_f32_near(m[(1, 1)], 0.0, 1e-6);

	m[(0, 1)] = 7.5;
	assert_f32_near(m[(0, 1)], 7.5, 1e-6);
}

#[test]
fn test_stack_vs_heap_storage_flags() {
	let a: Matrix<2, 2> = Matrix::new_stack([[1.0, 2.0], [3.0, 4.0]]);
	assert!(a.is_stack());
	assert!(!a.is_heap());
	match &a.cols {
		Data::STACK(..) => {}
		Data::HEAP(..) => panic!("expected STACK storage"),
	}

	let b: Matrix<2, 2> = Matrix::new_heap([[1.0, 2.0], [3.0, 4.0]]);
	assert!(b.is_heap());
	assert!(!b.is_stack());
	match &b.cols {
		Data::HEAP(..) => {}
		Data::STACK(..) => panic!("expected HEAP storage"),
	}
}

#[test]
fn test_mul_2x2() {
	// A = [[1, 2], [3, 4]]
	let a: Matrix<2, 2> = Matrix::from_rows([[1.0, 2.0], [3.0, 4.0]]);
	// B = [[5, 6], [7, 8]]
	let b: Matrix<2, 2> = Matrix::from_rows([[5.0, 6.0], [7.0, 8.0]]);
	// A*B = [[19, 22], [43, 50]]
	let c = a * b;
	assert_f32_near(c[(0, 0)], 19.0, 1e-6);
	assert_f32_near(c[(0, 1)], 22.0, 1e-6);
	assert_f32_near(c[(1, 0)], 43.0, 1e-6);
	assert_f32_near(c[(1, 1)], 50.0, 1e-6);
}

#[test]
fn test_mul_rectangular() {
	// A is 2x3, B is 3x1, so A*B is 2x1
	// A = [[1,2,3], [4,5,6]] (rows)
	let a: Matrix<2, 3> = Matrix::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
	// B = [[7], [8], [9]]
	let b: Matrix<3, 1> = Matrix::from_rows([[7.0], [8.0], [9.0]]);
	// A*B = [[50], [122]]
	let c: Matrix<2, 1> = a * b;
	assert_f32_near(c[(0, 0)], 50.0, 1e-6);
	assert_f32_near(c[(1, 0)], 122.0, 1e-6);
}
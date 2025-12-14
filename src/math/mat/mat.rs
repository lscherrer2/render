use super::Matrix;




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let mat = Matrix::new([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]);
    }

    #[test]
    fn test_mat_mul() {
        let mat1 = Matrix::from_rows([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);
        let mat2 = Matrix::from_rows([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        let mat3 = mat1 * mat2;
    }

    #[test]
    fn test_vec_mul() {
        let mat1 = Matrix::from_rows([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
        ]);
        let mat2 = Matrix::from_rows([
            [1.0], 
            [2.0], 
            [3.0],
        ]);
    }

    #[test]
    fn test_display() {
        let mat: Matrix<3, 3> = Matrix::new([
            [1.0, 4.0, 7.0],
            [2.0, 5.0, 8.0],
            [3.0, 6.0, 9.0],
        ]);
        mat.print();
    }

    #[test]
    fn test_heap() {
        let heap_mat: Matrix<3, 3> = Matrix::new_heap([
            [1.0, 4.0, 7.0],
            [2.0, 5.0, 8.0],
            [3.0, 6.0, 9.0],
        ]);
        let stack_mat: Matrix<3, 3> = Matrix::new_stack([
            [1.0, 4.0, 7.0],
            [2.0, 5.0, 8.0],
            [3.0, 6.0, 9.0],
        ]);
        assert!(heap_mat.is_heap());
        assert!(stack_mat.is_stack());

    }
}



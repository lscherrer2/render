use super::{Matrix, data::Data};
impl<const R: usize, const C: usize> 
Matrix<R, C> {
    pub fn print(&self) -> () {
        print!("[");
        (0..R).for_each(|r| {
            if r > 0 {print!(" ");};
            print!("[");
            (0..C).for_each(|c| {
                print!("{:5.2}", self[(r, c)]);
                if (c+1) != C {print!(",");};
            });
            if (r+1) == R {print!("]");}
            else {println!("]");};
        });
        println!("]");
    }
    pub fn is_heap(&self) -> bool {
        match &self.cols {
            Data::HEAP(..) => true,
            Data::STACK(..) => false,
        }
    }
    pub fn is_stack(&self) -> bool {
        match &self.cols {
            Data::HEAP(..) => false,
            Data::STACK(..) => true,
        }
    }
}
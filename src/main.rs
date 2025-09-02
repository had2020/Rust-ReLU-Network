fn main() {
    struct Pos {
        x: usize, // Rows
        y: usize, // Cols
    }

    struct Tensor {
        data: Vec<Vec<f32>>,
        pos: Pos,
    }

    impl Tensor {
        fn new(self, n: f32, s: Pos) -> Tensor {
            let mut r_data: Vec<Vec<f32>> = vec![Vec::new()];
            let mut r_y: Vec<f32> = Vec::new();
            for i in 0..s.y {
                r_y.push(n);
            }
            for i in 0..s.x {
                r_data.push(r_y);
            }
        }
    }
}

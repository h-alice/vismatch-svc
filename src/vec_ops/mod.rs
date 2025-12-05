use num_traits::Float;
use ndarray::Array1;

pub trait L2Norm<B: Float> {
    /// Implement the l2-norm.
    fn norm(&self) -> B;
}

impl L2Norm<f32> for Array1<f32> {
    fn norm(&self) -> f32 {
        let x = self.dot(self);
        x.sqrt()
    }
}

impl L2Norm<f64> for Array1<f64> {
    fn norm(&self) -> f64 {
        let x = self.dot(self);
        x.sqrt()
    }
}

pub trait UnitVector<B: Float> {
    fn unit(&self) -> Array1<B>; 
}


impl UnitVector<f32> for Array1<f32> {
    fn unit(&self) -> Array1<f32> {
        self.clone() / self.norm()
    }
}

impl UnitVector<f64> for Array1<f64> {
    fn unit(&self) -> Array1<f64> {
        self.clone() / self.norm()
    }
}
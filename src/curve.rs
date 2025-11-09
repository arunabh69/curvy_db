use num_complex::Complex;

pub struct Curve<Y> {
    data: Vec<Y>,
    x_axis: Vec<f64>,
    tags: Vec<String>
}

impl Curve<Complex<f64>> {

    pub fn set_x_axis(&mut self, mut x_axis: Vec<f64>) {
        x_axis.sort_by(|a,b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
        self.x_axis = x_axis;
           
    }
    pub fn max_element(&self) -> Complex<f64> {
        self.data.iter().max_by(|a, b| 
            a.norm_sqr().partial_cmp(&b.norm_sqr()).unwrap()).cloned().unwrap()
    }
    pub fn range_start(&self) -> f64 {
        self.x_axis.first().unwrap_or(&0.).clone()
    }
    pub fn range_end(&self) -> f64 { 
        self.x_axis.last().unwrap_or(&0.).clone() 
    }
    
}
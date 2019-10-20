fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
//合計
fn sum(v: &Vec<f64>) -> f64{
    let mut _sum = 0.;
    for num in v {
        _sum += num;
    }
    _sum
}
//平均
fn mean(v: &Vec<f64>) -> f64{
    let _sum    = sum(&v);
    let _length = v.len() as f64;
    let _mean = _sum/_length;
    _mean
}
//標準偏差
fn var(v: &Vec<f64>) -> f64{
    let _var = cov(&v,&v);
    _var.sqrt()
}
//共分散
fn cov(x: &Vec<f64>,y: &Vec<f64>) -> f64{
    let mut sxy = 0.;
    for i in 0..x.len() {
        sxy += (x[i] - mean(&x))*(y[i] - mean(&y));
    }
    sxy/x.len() as f64
}
//相関係数　
fn cor(x: &Vec<f64>,y: &Vec<f64>) -> f64{
    let _cor  = cov(&x,&y) / (var(&x) * var(&y) );
    _cor
}
//単回帰分析
struct Paramator {
    beta0: f64,
    beta1: f64,
}
fn singleregression(x: &Vec<f64>,y: &Vec<f64>) -> Paramator{
    let param = Paramator {
        beta0: mean(&y) - mean(&x)*cov(&x,&y)/cov(&x,&x),
        beta1: cov(&x,&y)/cov(&x,&x)
    };
    param
}

impl Paramator {
    fn aniticipation(&self,z:f64) -> f64 {
        self.beta0 + self.beta1 * z
    }

}

fn main() {
    let x = vec![2.2,4.1,5.5,1.9,3.4];
    let y = vec![71.,81.,86.,72.,77.];
    let res = singleregression(&x,&y);
    println!("{}",res.aniticipation(3.0)) 
}

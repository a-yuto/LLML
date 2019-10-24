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

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
//単回帰分析
fn singleregression(x: &Vec<f64>,y: &Vec<f64>) -> Box<dyn Fn(f64) -> f64>{
    let b0 = mean(&y) - mean(&x)*cov(&x,&y)/cov(&x,&x);
    let b1 = cov(&x,&y)/cov(&x,&x);
    Box::new(move |x| b0 + b1 * x)
}
fn multiple_regression_analysis(x1: &Vec<f64>,x2: &Vec<f64>,y: &Vec<f64>) -> Box<dyn Fn(f64) -> f64>{
    
}
fn main() {
    let x1 = vec![51.,38.,57.,51.,53.,77.,63.,69.,72.,73.];
    let x2 = vec![16.,4.,16., 11.,4., 22.,5., 5.,  2.,1.];
    let y  = vec![3.0,3.2,3.3,3.9,4.4,4.5,4.5,5.4,5.4,6.0]
    let f = singleregression(&x,&y);
    println!("{}",f(2.5).to_string());   
}

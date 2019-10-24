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


use std::ops::Div;

pub struct KdjData {
    pub day: String,
    pub open: f32,
    pub low: f32,
    pub high: f32,
    pub close: f32,
    pub vol: f32,
    pub amount: f32,
    pub change: f32,
    pub pct_chg: f32,
    pub amp: f32,
    pub turnover: f32,
}

fn calculate_closeness_percentage(numbers: &[f32]) -> f32 {
    let n = numbers.len();
    if n < 2 {
        return 999.0;
    }

    let mean = numbers.iter().sum::<f32>() / n as f32;

    let variance = numbers.iter().map(|&x| (x - mean).powi(2)).sum::<f32>();

    let std_dev_squared: <f32 as Div<f32>>::Output = variance / n as f32;

    std_dev_squared.sqrt()
}

//GERNATED BY CHAT-GPT-3.5
pub fn calculate_kdj(
    prices: &[KdjData],
    n: usize,
    m1: usize,
    m2: usize,
) -> Option<
    (Vec<(
        String, /*Day */
        f32,    /*K*/
        f32,    /*D */
        f32,    /*J wrong*/
        f32,    /*distance */
    )>),
> {
    if prices.len() < n {
        return None;
    }

    let mut high_prices = Vec::new();
    let mut low_prices = Vec::new();
    for i in 0..n {
        high_prices.push(prices[i].high);
        low_prices.push(prices[i].low);
    }
    let highest_high = *high_prices
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let lowest_low = *low_prices
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let rsv = (prices[n - 1].close - lowest_low) / (highest_high - lowest_low) * 100.0;
    let mut k_value = rsv;
    let mut d_value = k_value;

    let mut result: Vec<(String, f32, f32, f32, f32)> = vec![];

    for i in n..prices.len() {
        high_prices.remove(0);
        low_prices.remove(0);
        high_prices.push(prices[i].high);
        low_prices.push(prices[i].low);
        let highest_high = *high_prices
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let lowest_low = *low_prices
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let rsv = (prices[i].close - lowest_low) / (highest_high - lowest_low) * 100.0;
        k_value = (k_value * (m1 - 1) as f32 + rsv) / m1 as f32;
        d_value = (d_value * (m2 - 1) as f32 + k_value) / m2 as f32;
        let j_value = 3.0 * k_value - 2.0 * d_value;
        let distance = calculate_closeness_percentage(&[j_value, k_value, d_value]);

        result.push((
            prices.get(i).unwrap().day.to_string(),
            k_value,
            d_value,
            j_value,
            distance,
        ));
    }

    Some(result)
}

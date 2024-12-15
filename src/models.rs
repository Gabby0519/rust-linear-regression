use nalgebra::{DMatrix, DVector};

pub fn calculate_r_squared(actual: &[f64], predicted: &[f64]) -> f64 {
    let mean_actual = actual.iter().sum::<f64>() / actual.len() as f64;
    let ss_total = actual.iter().map(|&y| (y - mean_actual).powi(2)).sum::<f64>();
    let ss_residual = actual
        .iter()
        .zip(predicted.iter())
        .map(|(&y, &y_pred)| (y - y_pred).powi(2))
        .sum::<f64>();
    1.0 - (ss_residual / ss_total)
}

pub fn fit_linear(imdb_ratings: &[f64], meta_scores: &[f64]) -> (f64, f64) {
    let (mean_x, mean_y) = (
        imdb_ratings.iter().sum::<f64>() / imdb_ratings.len() as f64,
        meta_scores.iter().sum::<f64>() / meta_scores.len() as f64,
    );
    let slope = imdb_ratings
        .iter()
        .zip(meta_scores.iter())
        .map(|(&x, &y)| (x - mean_x) * (y - mean_y))
        .sum::<f64>()
        / imdb_ratings.iter().map(|&x| (x - mean_x).powi(2)).sum::<f64>();
    (slope, mean_y - slope * mean_x)
}

pub fn predict_linear(imdb_ratings: &[f64], slope: f64, intercept: f64) -> Vec<f64> {
    imdb_ratings.iter().map(|&x| slope * x + intercept).collect()
}

pub fn predict_polynomial(imdb_ratings: &[f64], meta_scores: &[f64], degree: usize) -> Vec<f64> {
    let x_matrix: Vec<Vec<f64>> = imdb_ratings
        .iter()
        .map(|&x| (0..=degree).map(|i| x.powi(i as i32)).collect())
        .collect();

    let x_matrix = DMatrix::from_row_slice(imdb_ratings.len(), degree + 1, &x_matrix.concat());
    let y_vector = DVector::from_column_slice(meta_scores);
    let x_t = x_matrix.transpose();
    let coefficients = (x_t.clone() * &x_matrix)
        .try_inverse()
        .unwrap()
        * x_t
        * y_vector;
    (x_matrix * coefficients).as_slice().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_linear() {
        let imdb_ratings = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let meta_scores = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope, intercept) = fit_linear(&imdb_ratings, &meta_scores);
        assert!((slope - 2.0).abs() < 1e-6, "Incorrect slope");
        assert!((intercept - 0.0).abs() < 1e-6, "Incorrect intercept");
    }

    #[test]
    fn test_predict_linear() {
        let imdb_ratings = vec![1.0, 2.0, 3.0];
        let slope = 2.0;
        let intercept = 1.0;
        let predictions = predict_linear(&imdb_ratings, slope, intercept);
        assert_eq!(predictions, vec![3.0, 5.0, 7.0], "Linear predictions are incorrect");
    }

    #[test]
    fn test_predict_polynomial() {
        let imdb_ratings = vec![1.0, 2.0, 3.0];
        let meta_scores = vec![1.0, 4.0, 9.0];
        let predictions = predict_polynomial(&imdb_ratings, &meta_scores, 2);
        assert!((predictions[0] - 1.0).abs() < 1e-6, "Polynomial prediction is incorrect");
    }

    #[test]
    fn test_calculate_r_squared() {
        let actual = vec![2.0, 4.0, 6.0];
        let predicted = vec![2.0, 4.0, 6.0];
        let r2 = calculate_r_squared(&actual, &predicted);
        assert!((r2 - 1.0).abs() < 1e-6, "R² should be 1.0 for perfect predictions");
    }
}

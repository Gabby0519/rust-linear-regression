mod data;
mod models;
mod visualization;

use data::load_dataset;
use models::{fit_linear, predict_linear, predict_polynomial, calculate_r_squared};
use visualization::plot_predictions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the dataset
    let (imdb_ratings, meta_scores) = load_dataset("imdb_top_1000.csv")?;
    println!("Loaded dataset with {} rows.", imdb_ratings.len());

    // Perform Linear Regression
    let (slope, intercept) = fit_linear(&imdb_ratings, &meta_scores);
    println!("Linear Regression Equation: y = {:.4}x + {:.4}", slope, intercept);

    // Predict using Linear Regression
    let linear_predictions = predict_linear(&imdb_ratings, slope, intercept);

    // Calculate R² for Linear Regression
    let linear_r2 = calculate_r_squared(&meta_scores, &linear_predictions);
    println!("Linear Regression R²: {:.4}", linear_r2);

    // Perform Polynomial Regression (Degree 2)
    let polynomial_predictions = predict_polynomial(&imdb_ratings, &meta_scores, 2);

    // Calculate R² for Polynomial Regression
    let polynomial_r2 = calculate_r_squared(&meta_scores, &polynomial_predictions);
    println!("Polynomial Regression (Degree 2) R²: {:.4}", polynomial_r2);

    // Plot the Predictions
    plot_predictions(
        &imdb_ratings,
        &meta_scores,
        &[linear_predictions, polynomial_predictions],
        &["Linear Regression", "Polynomial Regression"],
        "predictions.png",
    )?;
    println!("Plot saved as predictions.png");

    Ok(())
}

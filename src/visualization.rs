use plotters::prelude::*;
use std::error::Error;

pub fn plot_predictions(
    imdb_ratings: &[f64],
    actual_meta_scores: &[f64],
    predictions: &[Vec<f64>],
    labels: &[&str],
    file_name: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Predictions vs Actual Meta Scores", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, 0.0..100.0)?;

    chart.configure_mesh().x_desc("IMDB Rating").y_desc("Meta Score").draw()?;

    chart.draw_series(
        imdb_ratings
            .iter()
            .zip(actual_meta_scores.iter())
            .map(|(&x, &y)| Circle::new((x, y), 5, ShapeStyle::from(&BLUE).filled())),
    )?;

    for (pred, label) in predictions.iter().zip(labels.iter()) {
        chart
            .draw_series(
                imdb_ratings
                    .iter()
                    .zip(pred.iter())
                    .map(|(&x, &y)| Circle::new((x, y), 5, ShapeStyle::from(&RED).filled())),
            )?
            .label(*label)
            .legend(|(x, y)| Circle::new((x, y), 5, ShapeStyle::from(&RED).filled()));
    }

    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_predictions() {
        let imdb_ratings = vec![1.0, 2.0, 3.0];
        let meta_scores = vec![10.0, 20.0, 30.0];
        let predictions = vec![vec![10.0, 20.0, 30.0], vec![12.0, 22.0, 32.0]];
        let labels = vec!["Model 1", "Model 2"];
        let result = plot_predictions(&imdb_ratings, &meta_scores, &predictions, &labels, "test_plot.png");
        assert!(result.is_ok(), "Plotting function should not panic");
    }
}

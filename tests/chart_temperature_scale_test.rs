use pi_inky_weather_epd::dashboard::chart::{CurveType, GraphData, HourlyForecastGraph};

#[test]
fn chart_keeps_zero_to_twenty_celsius_visible_when_data_is_inside_that_range() {
    let mut graph = HourlyForecastGraph {
        curves: vec![
            CurveType::ActualTemp(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: 8.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 12.0 },
                ],
                smooth: true,
            }),
            CurveType::TempFeelLike(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: 9.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 11.0 },
                ],
                smooth: true,
            }),
            CurveType::RainChance(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: 10.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 20.0 },
                ],
                smooth: false,
            }),
        ],
        ..Default::default()
    };

    let _ = graph.draw_graph().unwrap();

    assert_eq!(graph.min_y, 0.0);
    assert_eq!(graph.max_y, 20.0);
}

#[test]
fn chart_expands_beyond_zero_to_twenty_when_forecast_exceeds_it() {
    let mut graph = HourlyForecastGraph {
        curves: vec![
            CurveType::ActualTemp(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: -4.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 26.0 },
                ],
                smooth: true,
            }),
            CurveType::TempFeelLike(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: -2.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 24.0 },
                ],
                smooth: true,
            }),
            CurveType::RainChance(GraphData {
                points: vec![
                    pi_inky_weather_epd::dashboard::chart::Point { x: 0.0, y: 10.0 },
                    pi_inky_weather_epd::dashboard::chart::Point { x: 1.0, y: 20.0 },
                ],
                smooth: false,
            }),
        ],
        ..Default::default()
    };

    let _ = graph.draw_graph().unwrap();

    assert_eq!(graph.min_y, -4.0);
    assert_eq!(graph.max_y, 26.0);
}

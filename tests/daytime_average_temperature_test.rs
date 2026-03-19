use chrono::{TimeZone, Utc};
use pi_inky_weather_epd::{
    clock::FixedClock,
    configs::settings::TemperatureUnit,
    dashboard::context::ContextBuilder,
    domain::models::{HourlyForecast, Precipitation, Temperature, Wind},
};
use serial_test::serial;

fn temp_c(value: f32) -> Temperature {
    Temperature::new(value, TemperatureUnit::C)
}

fn hourly_forecast(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    temp_value_c: f32,
) -> HourlyForecast {
    HourlyForecast {
        time: Utc
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .unwrap(),
        temperature: temp_c(temp_value_c),
        apparent_temperature: temp_c(temp_value_c),
        wind: Wind::new(0, 0),
        precipitation: Precipitation::new(Some(0), Some(0), Some(0)),
        uv_index: 0,
        relative_humidity: 50,
        is_night: false,
        cloud_cover: Some(0),
        weather_code: Some(0),
    }
}

#[test]
#[serial]
fn uses_time_weighted_daytime_average_for_hourly_data() {
    let original_tz = std::env::var("TZ").ok();
    unsafe { std::env::set_var("TZ", "UTC") };

    let clock = FixedClock::new(Utc.with_ymd_and_hms(2025, 10, 9, 0, 0, 0).unwrap());
    let hourly_forecasts = vec![
        hourly_forecast(2025, 10, 9, 8, 0, 10.0),
        hourly_forecast(2025, 10, 9, 9, 0, 10.0),
        hourly_forecast(2025, 10, 9, 10, 0, 12.0),
        hourly_forecast(2025, 10, 9, 11, 0, 14.0),
        hourly_forecast(2025, 10, 9, 12, 0, 16.0),
        hourly_forecast(2025, 10, 9, 13, 0, 18.0),
        hourly_forecast(2025, 10, 9, 14, 0, 20.0),
        hourly_forecast(2025, 10, 9, 15, 0, 22.0),
        hourly_forecast(2025, 10, 9, 16, 0, 24.0),
        hourly_forecast(2025, 10, 9, 17, 0, 26.0),
        hourly_forecast(2025, 10, 9, 18, 0, 28.0),
        hourly_forecast(2025, 10, 9, 19, 0, 30.0),
        hourly_forecast(2025, 10, 9, 20, 0, 32.0),
        hourly_forecast(2025, 10, 9, 21, 0, 34.0),
        hourly_forecast(2025, 10, 9, 22, 0, 36.0),
    ];

    let mut builder = ContextBuilder::new();
    builder.with_hourly_forecast_data(hourly_forecasts, &clock);

    assert_eq!(builder.context.daytime_average_temp, "21");
    assert_eq!(builder.context.daytime_average_temp_dual, "21°C / 70°F");

    unsafe {
        match original_tz {
            Some(tz) => std::env::set_var("TZ", tz),
            None => std::env::remove_var("TZ"),
        }
    }
}

#[test]
#[serial]
fn uses_weighted_average_for_irregular_time_spacing() {
    let original_tz = std::env::var("TZ").ok();
    unsafe { std::env::set_var("TZ", "UTC") };

    let clock = FixedClock::new(Utc.with_ymd_and_hms(2025, 10, 9, 0, 0, 0).unwrap());
    let hourly_forecasts = vec![
        hourly_forecast(2025, 10, 9, 8, 30, 10.0),
        hourly_forecast(2025, 10, 9, 9, 15, 12.0),
        hourly_forecast(2025, 10, 9, 10, 30, 18.0),
        hourly_forecast(2025, 10, 9, 12, 0, 20.0),
        hourly_forecast(2025, 10, 9, 13, 1, 14.0),
        hourly_forecast(2025, 10, 9, 20, 30, 8.0),
        hourly_forecast(2025, 10, 9, 21, 30, 6.0),
    ];

    let mut builder = ContextBuilder::new();
    builder.with_hourly_forecast_data(hourly_forecasts, &clock);

    assert_eq!(builder.context.daytime_average_temp, "14");
    assert_eq!(builder.context.daytime_average_temp_dual, "14°C / 58°F");

    unsafe {
        match original_tz {
            Some(tz) => std::env::set_var("TZ", tz),
            None => std::env::remove_var("TZ"),
        }
    }
}

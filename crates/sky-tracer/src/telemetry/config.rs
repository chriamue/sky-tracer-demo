use opentelemetry::{
    global,
    sdk::{propagation::TraceContextPropagator, trace::Config, Resource},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use std::env;
use std::error::Error;
use tracing::info;
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub otel_endpoint: String,
    pub service_name: String,
}

impl TelemetryConfig {
    pub fn from_env() -> Self {
        Self {
            enabled: env::var("ENABLE_TELEMETRY")
                .map(|v| v.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
            otel_endpoint: env::var("OTEL_ENDPOINT")
                .unwrap_or_else(|_| "http://jaeger:4317".to_string()),
            service_name: env::var("SERVICE_NAME")
                .unwrap_or_else(|_| "unknown-service".to_string()),
        }
    }
}

pub fn init_tracer(config: &TelemetryConfig) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    if !config.enabled {
        return Ok(());
    }

    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&config.otel_endpoint),
        )
        .with_trace_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                config.service_name.clone(),
            )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .with(telemetry)
        .init();

    Ok(())
}

pub fn setup_telemetry() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let config = TelemetryConfig::from_env();

    if !config.enabled {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_thread_names(true)
            .with_target(true)
            .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339())
            .init();

        info!("Telemetry is disabled, using standard logging");
        return Ok(());
    }

    info!("Initializing telemetry with config: {:?}", config);
    init_tracer(&config)?;
    info!("Telemetry initialized successfully");

    Ok(())
}

pub fn shutdown_telemetry() {
    if env::var("ENABLE_TELEMETRY")
        .map(|v| v.parse::<bool>().unwrap_or(false))
        .unwrap_or(false)
    {
        global::shutdown_tracer_provider();
    }
}

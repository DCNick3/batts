// 1. Run `cargo add opentelemetry opentelemetry-otlp tracing-opentelemetry tracing-subscriber --features=opentelemetry/rt-tokio,tracing-subscriber/env-filter`
// 2. add `init_tracing::init_tracing().context("Setting up the opentelemetry exporter")?;` to main.rs

use anyhow::{Context, Result};
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::sdk::propagation::{
    BaggagePropagator, TextMapCompositePropagator, TraceContextPropagator,
};
use opentelemetry::sdk::resource::{EnvResourceDetector, SdkProvidedResourceDetector};
use opentelemetry::sdk::{trace as sdktrace, Resource};
use opentelemetry::trace::TraceError;
use opentelemetry_otlp::{HasExportConfig, WithExportConfig};
use std::time::Duration;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;

/// Configure the global propagator based on content of the env variable [OTEL_PROPAGATORS](https://opentelemetry.io/docs/concepts/sdk-configuration/general-sdk-configuration/#otel_propagators)
/// Specifies Propagators to be used in a comma-separated list.
/// Default value: `"tracecontext,baggage"`
/// Example: `export OTEL_PROPAGATORS="b3"`
/// Accepted values for `OTEL_PROPAGATORS` are:
///
/// - "tracecontext": W3C Trace Context
/// - "baggage": W3C Baggage
/// - "b3": B3 Single (require feature "zipkin")
/// - "b3multi": B3 Multi (require feature "zipkin")
/// - "jaeger": Jaeger (require feature "jaeger")
/// - "xray": AWS X-Ray (require feature "xray")
/// - "ottrace": OT Trace (third party) (not supported)
/// - "none": No automatically configured propagator.
///
/// # Errors
///
/// Will return `TraceError` if issue in reading or instanciate propagator.
pub fn init_propagator() -> Result<(), TraceError> {
    let value_from_env =
        std::env::var("OTEL_PROPAGATORS").unwrap_or_else(|_| "tracecontext,baggage".to_string());
    let propagators: Vec<(Box<dyn TextMapPropagator + Send + Sync>, String)> = value_from_env
        .split(',')
        .map(|s| {
            let name = s.trim().to_lowercase();
            propagator_from_string(&name).map(|o| o.map(|b| (b, name)))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    if !propagators.is_empty() {
        let (propagators_impl, propagators_name): (Vec<_>, Vec<_>) =
            propagators.into_iter().unzip();
        tracing::debug!(OTEL_PROPAGATORS = propagators_name.join(","));
        let composite_propagator = TextMapCompositePropagator::new(propagators_impl);
        opentelemetry::global::set_text_map_propagator(composite_propagator);
    }
    Ok(())
}

#[allow(clippy::box_default)]
fn propagator_from_string(
    v: &str,
) -> Result<Option<Box<dyn TextMapPropagator + Send + Sync>>, TraceError> {
    match v {
        "tracecontext" => Ok(Some(Box::new(TraceContextPropagator::new()))),
        "baggage" => Ok(Some(Box::new(BaggagePropagator::new()))),
        "none" => Ok(None),
        unknown => Err(opentelemetry::trace::TraceError::from(format!(
            "unsupported propagators form env OTEL_PROPAGATORS: '{unknown}'"
        ))),
    }
}

fn init_tracer() -> Result<sdktrace::Tracer> {
    let mut exporter = opentelemetry_otlp::new_exporter().tonic().with_env();

    println!(
        "Using opentelemetry endpoint {}",
        exporter.export_config().endpoint
    );

    // overwrite the service name because k8s service name does not always matches what we want
    std::env::set_var("OTEL_SERVICE_NAME", env!("CARGO_PKG_NAME"));

    let resource = Resource::from_detectors(
        Duration::from_secs(0),
        vec![
            Box::new(EnvResourceDetector::new()),
            Box::new(SdkProvidedResourceDetector),
        ],
    );

    println!("Using opentelemetry resources {:?}", resource);

    Ok(opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(sdktrace::config().with_resource(resource))
        .install_batch(opentelemetry::runtime::Tokio)?)
}

pub fn init_tracing() -> Result<()> {
    let tracer = init_tracer().context("Setting up the opentelemetry exporter")?;

    let default = concat!(env!("CARGO_CRATE_NAME"), "=trace")
        .parse()
        .expect("hard-coded default directive should be valid");

    Registry::default()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(default)
                .from_env_lossy()
                .add_directive("otel::tracing=trace".parse().unwrap()),
        )
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .event_format(tracing_subscriber::fmt::format::Format::default().compact()),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    init_propagator().context("Setting up the opentelemetry propagator")?;

    Ok(())
}

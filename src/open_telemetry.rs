use opentelemetry::sdk::{trace, Resource};
use opentelemetry::trace::TraceError;
use opentelemetry::{runtime, KeyValue};
use opentelemetry_otlp;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions;

pub fn init_trace() -> Result<trace::Tracer, TraceError> {

    let url = "http://localhost:4317";
    println!("hello init trace is starting");
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(url),
        )
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otel-service",
            )])),
        )
        .install_batch(runtime::Tokio);

    println!("hello init trace is done");
    tracer
}

use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::trace::{self, Sampler};
use opentelemetry_jaeger::new_agent_pipeline;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use tracing_opentelemetry::layer;

/// Initialize OpenTelemetry with Jaeger exporter
pub fn init_tracer() -> Result<opentelemetry::sdk::trace::Tracer, opentelemetry::global::Error> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    
    // Get Jaeger configuration from environment variables or use defaults
    let jaeger_host = env::var("OTEL_EXPORTER_JAEGER_AGENT_HOST").unwrap_or_else(|_| "jaeger".to_string());
    let jaeger_port = env::var("OTEL_EXPORTER_JAEGER_AGENT_PORT")
        .unwrap_or_else(|_| "6831".to_string())
        .parse::<u16>()
        .unwrap_or(6831);
    let service_name = env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "rust-otel-service".to_string());
    
    println!("Configuring Jaeger exporter with host: {}, port: {}, service: {}", jaeger_host, jaeger_port, service_name);
    
    // Configure Jaeger exporter with simpler configuration
    let tracer = new_agent_pipeline()
        .with_service_name(service_name)
        .with_endpoint(format!("{}:{}", jaeger_host, jaeger_port))
        .with_trace_config(trace::config().with_sampler(Sampler::AlwaysOn))
        .install_simple()?;
    
    println!("Jaeger tracer initialized successfully");
    
    Ok(tracer)
}

/// Set up tracing with OpenTelemetry
pub fn init_tracing() {
    println!("Initializing tracing with OpenTelemetry");
    
    // Set default logging level to INFO
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::default()
            .add_directive(Level::INFO.into())
            .add_directive("actix_web=info".parse().unwrap())
            .add_directive("rust_otel=debug".parse().unwrap())
    });
    
    // Initialize the OpenTelemetry tracer
    let tracer = match init_tracer() {
        Ok(tracer) => {
            println!("OpenTelemetry tracer initialized successfully");
            tracer
        },
        Err(e) => {
            eprintln!("Failed to initialize OpenTelemetry tracer: {}", e);
            panic!("Failed to initialize tracer");
        }
    };
    
    // Initialize tracing subscriber with OpenTelemetry
    let telemetry = layer().with_tracer(tracer);
    
    Registry::default()
        .with(env_filter)
        .with(telemetry)
        .init();
    
    println!("Tracing initialization complete");
} 
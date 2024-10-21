use opentelemetry::global;
use opentelemetry::trace::Tracer;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

fn main() {
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let tracer_provider = TracerProvider::builder()
        .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    global::set_tracer_provider(tracer_provider);

    let tracing_layer = tracing_opentelemetry::layer();

    let log_provider = LoggerProvider::builder()
        .with_simple_exporter(stdout::LogExporter::default())
        .build();

    //let log_layer = layer::OpenTelemetryTracingBridge::new(&log_provider);
    let log_layer = layer::OpenTelemetryTracingBridge::new(&log_provider);

    let subscriber = Registry::default().with(tracing_layer).with(log_layer);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let tracer = global::tracer("dash0");
        tracer.in_span("app_init", |_ctx| {
            let number_shaved = 3;
            info!(
                number_shaved,
                "yak shaving completed."
            );
        });
    });

    global::shutdown_tracer_provider();
}
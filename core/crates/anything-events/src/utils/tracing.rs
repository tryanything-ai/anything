use std::fmt::{Debug, Result as FmtResult};

use chrono::SecondsFormat;
use opentelemetry::{global, sdk::propagation::TraceContextPropagator};

use crate::config::AnythingEventsConfig;
use colored::Colorize;
use tracing::{
    field::{Field, Visit},
    Event, Level, Subscriber,
};
use tracing_log::NormalizeEvent;
use tracing_subscriber::{
    field::RecordFields,
    fmt::{self, format::Writer, FmtContext, FormatEvent, FormatFields, FormattedFields},
    prelude::*,
    registry::LookupSpan,
    EnvFilter,
};

pub fn setup_tracing(_service_name: String, _config: &AnythingEventsConfig) {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer()
        .event_format(SemiCompact)
        .fmt_fields(SemiCompact);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

fn level_color(level: Level, msg: String) -> impl std::fmt::Display {
    match level {
        Level::ERROR => msg.bright_red(),
        Level::WARN => msg.bright_yellow(),
        Level::INFO => msg.bright_green(),
        Level::DEBUG => msg.bright_blue(),
        Level::TRACE => msg.bright_purple(),
    }
}

struct SemiCompactVisitor {
    fields: String,
    message: String,
}

impl Visit for SemiCompactVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => self.message = format!("{value:?}\n"),
            name if name.starts_with("log.") => (),
            name => {
                self.fields
                    .push_str(&format!("    {}: {:?}\n", name.cyan(), value));
            }
        };
    }
}

struct SemiCompact;

impl<C, N> FormatEvent<C, N> for SemiCompact
where
    C: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, C, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> FmtResult {
        let normalized_meta = event.normalized_metadata();
        let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());

        let header = format!(
            "[{} {} {}]",
            chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false),
            meta.level(),
            meta.target(),
        );

        writeln!(writer, "{}", level_color(*meta.level(), header))?;

        ctx.field_format().format_fields(writer.by_ref(), event)?;

        ctx.visit_spans(|span| {
            //write!(writer, "    -> {}\n", span.name().bold())?;
            let ext = span.extensions();
            let data = ext.get::<FormattedFields<SemiCompact>>().unwrap();
            write!(writer, "{data}")
        })?;

        Ok(())
    }
}

impl<'w> FormatFields<'w> for SemiCompact {
    fn format_fields<R: RecordFields>(&self, mut writer: Writer<'w>, fields: R) -> FmtResult {
        let mut visitor = SemiCompactVisitor {
            fields: String::new(),
            message: String::new(),
        };
        fields.record(&mut visitor);
        write!(writer, "{}", visitor.message.bright_white())?;
        write!(writer, "{}", visitor.fields)?;
        Ok(())
    }
}

/// Macro for instrumenting spans
#[macro_export]
macro_rules! instrumented {
    ($span:expr, $block:tt) => {{
        use tracing::Instrument;
        async {
            {
                $block
            };
            Ok::<(), anyhow::Error>(())
        }
        .instrument($span)
        .await
    }};
}

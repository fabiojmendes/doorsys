use std::{env, fmt};

use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{format::Writer, FmtContext, FormatEvent, FormatFields, FormattedFields},
    registry::LookupSpan,
    EnvFilter,
};

struct PriorityFormatter;

impl<S, N> FormatEvent<S, N> for PriorityFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        let priority = match metadata.level() {
            &Level::TRACE => 7,
            &Level::DEBUG => 6,
            &Level::INFO => 5,
            &Level::WARN => 4,
            &Level::ERROR => 3,
        };
        write!(
            &mut writer,
            "<{}>{} {}: ",
            priority,
            metadata.level(),
            metadata.target()
        )?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

pub fn init() {
    if let "1" = env::var("SYSLOG_FORMAT")
        .unwrap_or(String::from("0"))
        .as_str()
    {
        tracing_subscriber::fmt()
            .event_format(PriorityFormatter)
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt::init();
    };
}

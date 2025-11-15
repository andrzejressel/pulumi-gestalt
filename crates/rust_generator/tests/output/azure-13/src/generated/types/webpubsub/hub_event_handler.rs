#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HubEventHandler {
    /// An `auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "auth")]
    pub r#auth: Option<Box<super::super::types::webpubsub::HubEventHandlerAuth>>,
    /// Specifies the list of system events. Supported values are `connect`, `connected` and `disconnected`.
    #[builder(into)]
    #[serde(rename = "systemEvents")]
    pub r#system_events: Option<Vec<String>>,
    /// The Event Handler URL Template. Two predefined parameters `{hub}` and `{event}` are available to use in the template. The value of the EventHandler URL is dynamically calculated when the client request comes in. Example: `http://example.com/api/{hub}/{event}`.
    #[builder(into)]
    #[serde(rename = "urlTemplate")]
    pub r#url_template: String,
    /// Specifies the matching event names. There are 3 kind of patterns supported: * `*` matches any event name * `,` Combine multiple events with `,` for example `event1,event2`, it matches event `event1` and `event2` * The single event name, for example `event1`, it matches `event1`.
    #[builder(into)]
    #[serde(rename = "userEventPattern")]
    pub r#user_event_pattern: Option<String>,
}

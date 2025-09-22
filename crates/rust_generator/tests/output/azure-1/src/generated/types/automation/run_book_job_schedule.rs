#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RunBookJobSchedule {
    /// The UUID of automation runbook job schedule ID.
    #[builder(into)]
    #[serde(rename = "jobScheduleId")]
    pub r#job_schedule_id: Option<String>,
    /// A map of key/value pairs corresponding to the arguments that can be passed to the Runbook.
    /// 
    /// > **NOTE:** The parameter keys/names must strictly be in lowercase, even if this is not the case in the runbook. This is due to a limitation in Azure Automation where the parameter names are normalized. The values specified don't have this limitation.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Name of a Hybrid Worker Group the Runbook will be executed on.
    #[builder(into)]
    #[serde(rename = "runOn")]
    pub r#run_on: Option<String>,
    /// The name of the Schedule.
    #[builder(into)]
    #[serde(rename = "scheduleName")]
    pub r#schedule_name: String,
}

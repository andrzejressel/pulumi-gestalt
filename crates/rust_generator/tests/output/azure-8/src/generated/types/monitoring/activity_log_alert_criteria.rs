#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActivityLogAlertCriteria {
    /// The email address or Azure Active Directory identifier of the user who performed the operation.
    #[builder(into)]
    #[serde(rename = "caller")]
    pub r#caller: Option<String>,
    /// The category of the operation. Possible values are `Administrative`, `Autoscale`, `Policy`, `Recommendation`, `ResourceHealth`, `Security` and `ServiceHealth`.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    /// The severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    /// A list of severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    /// 
    /// > **NOTE:** `level` and `levels` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "levels")]
    pub r#levels: Option<Vec<String>>,
    /// The Resource Manager Role-Based Access Control operation name. Supported operation should be of the form: `<resourceProvider>/<resourceType>/<operation>`.
    #[builder(into)]
    #[serde(rename = "operationName")]
    pub r#operation_name: Option<String>,
    /// The recommendation category of the event. Possible values are `Cost`, `Reliability`, `OperationalExcellence`, `HighAvailability` and `Performance`. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    #[serde(rename = "recommendationCategory")]
    pub r#recommendation_category: Option<String>,
    /// The recommendation impact of the event. Possible values are `High`, `Medium` and `Low`. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    #[serde(rename = "recommendationImpact")]
    pub r#recommendation_impact: Option<String>,
    /// The recommendation type of the event. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    #[serde(rename = "recommendationType")]
    pub r#recommendation_type: Option<String>,
    /// The name of resource group monitored by the activity log alert.
    #[builder(into)]
    #[serde(rename = "resourceGroup")]
    pub r#resource_group: Option<String>,
    /// A list of names of resource groups monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_group` and `resource_groups` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Option<Vec<String>>,
    /// A block to define fine grain resource health settings.
    #[builder(into)]
    #[serde(rename = "resourceHealth")]
    pub r#resource_health: Option<Box<super::super::types::monitoring::ActivityLogAlertCriteriaResourceHealth>>,
    /// The specific resource monitored by the activity log alert. It should be within one of the `scopes`.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Option<String>,
    /// A list of specific resources monitored by the activity log alert. It should be within one of the `scopes`.
    /// 
    /// > **NOTE:** `resource_id` and `resource_ids` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "resourceIds")]
    pub r#resource_ids: Option<Vec<String>>,
    /// The name of the resource provider monitored by the activity log alert.
    #[builder(into)]
    #[serde(rename = "resourceProvider")]
    pub r#resource_provider: Option<String>,
    /// A list of names of resource providers monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_provider` and `resource_providers` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "resourceProviders")]
    pub r#resource_providers: Option<Vec<String>>,
    /// The resource type monitored by the activity log alert.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
    /// A list of resource types monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_type` and `resource_types` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<String>>,
    /// A block to define fine grain service health settings.
    #[builder(into)]
    #[serde(rename = "serviceHealth")]
    pub r#service_health: Option<Box<super::super::types::monitoring::ActivityLogAlertCriteriaServiceHealth>>,
    /// The status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// A list of status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    /// 
    /// > **NOTE:** `status` and `statuses` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<String>>,
    /// The sub status of the event.
    #[builder(into)]
    #[serde(rename = "subStatus")]
    pub r#sub_status: Option<String>,
    /// A list of sub status of the event.
    /// 
    /// > **NOTE:** `sub_status` and `sub_statuses` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "subStatuses")]
    pub r#sub_statuses: Option<Vec<String>>,
}

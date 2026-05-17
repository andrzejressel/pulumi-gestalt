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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActivityLogAlertCriteria {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "caller",
                    &self.r#caller,
                ),
                to_pulumi_object_field(
                    "category",
                    &self.r#category,
                ),
                to_pulumi_object_field(
                    "level",
                    &self.r#level,
                ),
                to_pulumi_object_field(
                    "levels",
                    &self.r#levels,
                ),
                to_pulumi_object_field(
                    "operation_name",
                    &self.r#operation_name,
                ),
                to_pulumi_object_field(
                    "recommendation_category",
                    &self.r#recommendation_category,
                ),
                to_pulumi_object_field(
                    "recommendation_impact",
                    &self.r#recommendation_impact,
                ),
                to_pulumi_object_field(
                    "recommendation_type",
                    &self.r#recommendation_type,
                ),
                to_pulumi_object_field(
                    "resource_group",
                    &self.r#resource_group,
                ),
                to_pulumi_object_field(
                    "resource_groups",
                    &self.r#resource_groups,
                ),
                to_pulumi_object_field(
                    "resource_health",
                    &self.r#resource_health,
                ),
                to_pulumi_object_field(
                    "resource_id",
                    &self.r#resource_id,
                ),
                to_pulumi_object_field(
                    "resource_ids",
                    &self.r#resource_ids,
                ),
                to_pulumi_object_field(
                    "resource_provider",
                    &self.r#resource_provider,
                ),
                to_pulumi_object_field(
                    "resource_providers",
                    &self.r#resource_providers,
                ),
                to_pulumi_object_field(
                    "resource_type",
                    &self.r#resource_type,
                ),
                to_pulumi_object_field(
                    "resource_types",
                    &self.r#resource_types,
                ),
                to_pulumi_object_field(
                    "service_health",
                    &self.r#service_health,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "statuses",
                    &self.r#statuses,
                ),
                to_pulumi_object_field(
                    "sub_status",
                    &self.r#sub_status,
                ),
                to_pulumi_object_field(
                    "sub_statuses",
                    &self.r#sub_statuses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActivityLogAlertCriteria {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#caller: {
                        let field_value = match fields_map.get("caller") {
                            Some(value) => value,
                            None => bail!("Missing field 'caller' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#category: {
                        let field_value = match fields_map.get("category") {
                            Some(value) => value,
                            None => bail!("Missing field 'category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#level: {
                        let field_value = match fields_map.get("level") {
                            Some(value) => value,
                            None => bail!("Missing field 'level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#levels: {
                        let field_value = match fields_map.get("levels") {
                            Some(value) => value,
                            None => bail!("Missing field 'levels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operation_name: {
                        let field_value = match fields_map.get("operation_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'operation_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_category: {
                        let field_value = match fields_map.get("recommendation_category") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_impact: {
                        let field_value = match fields_map.get("recommendation_impact") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_impact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_type: {
                        let field_value = match fields_map.get("recommendation_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendation_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group: {
                        let field_value = match fields_map.get("resource_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_groups: {
                        let field_value = match fields_map.get("resource_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_health: {
                        let field_value = match fields_map.get("resource_health") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_health' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_ids: {
                        let field_value = match fields_map.get("resource_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_provider: {
                        let field_value = match fields_map.get("resource_provider") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_provider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_providers: {
                        let field_value = match fields_map.get("resource_providers") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_providers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resource_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_health: {
                        let field_value = match fields_map.get("service_health") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_health' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statuses: {
                        let field_value = match fields_map.get("statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_status: {
                        let field_value = match fields_map.get("sub_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'sub_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_statuses: {
                        let field_value = match fields_map.get("sub_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'sub_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActivityLogAlertCriteria {
    /// The email address or Azure Active Directory identifier of the user who performed the operation.
    #[builder(into)]
    pub r#caller: Option<String>,
    /// The category of the operation. Possible values are `Administrative`, `Autoscale`, `Policy`, `Recommendation`, `ResourceHealth`, `Security` and `ServiceHealth`.
    #[builder(into)]
    pub r#category: String,
    /// The severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    #[builder(into)]
    pub r#level: Option<String>,
    /// A list of severity level of the event. Possible values are `Verbose`, `Informational`, `Warning`, `Error`, and `Critical`.
    /// 
    /// > **NOTE:** `level` and `levels` are mutually exclusive.
    #[builder(into)]
    pub r#levels: Option<Vec<String>>,
    /// The Resource Manager Role-Based Access Control operation name. Supported operation should be of the form: `<resourceProvider>/<resourceType>/<operation>`.
    #[builder(into)]
    pub r#operation_name: Option<String>,
    /// The recommendation category of the event. Possible values are `Cost`, `Reliability`, `OperationalExcellence`, `HighAvailability` and `Performance`. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    pub r#recommendation_category: Option<String>,
    /// The recommendation impact of the event. Possible values are `High`, `Medium` and `Low`. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    pub r#recommendation_impact: Option<String>,
    /// The recommendation type of the event. It is only allowed when `category` is `Recommendation`.
    #[builder(into)]
    pub r#recommendation_type: Option<String>,
    /// The name of resource group monitored by the activity log alert.
    #[builder(into)]
    pub r#resource_group: Option<String>,
    /// A list of names of resource groups monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_group` and `resource_groups` are mutually exclusive.
    #[builder(into)]
    pub r#resource_groups: Option<Vec<String>>,
    /// A block to define fine grain resource health settings.
    #[builder(into)]
    pub r#resource_health: Option<Box<super::super::types::monitoring::ActivityLogAlertCriteriaResourceHealth>>,
    /// The specific resource monitored by the activity log alert. It should be within one of the `scopes`.
    #[builder(into)]
    pub r#resource_id: Option<String>,
    /// A list of specific resources monitored by the activity log alert. It should be within one of the `scopes`.
    /// 
    /// > **NOTE:** `resource_id` and `resource_ids` are mutually exclusive.
    #[builder(into)]
    pub r#resource_ids: Option<Vec<String>>,
    /// The name of the resource provider monitored by the activity log alert.
    #[builder(into)]
    pub r#resource_provider: Option<String>,
    /// A list of names of resource providers monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_provider` and `resource_providers` are mutually exclusive.
    #[builder(into)]
    pub r#resource_providers: Option<Vec<String>>,
    /// The resource type monitored by the activity log alert.
    #[builder(into)]
    pub r#resource_type: Option<String>,
    /// A list of resource types monitored by the activity log alert.
    /// 
    /// > **NOTE:** `resource_type` and `resource_types` are mutually exclusive.
    #[builder(into)]
    pub r#resource_types: Option<Vec<String>>,
    /// A block to define fine grain service health settings.
    #[builder(into)]
    pub r#service_health: Option<Box<super::super::types::monitoring::ActivityLogAlertCriteriaServiceHealth>>,
    /// The status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    #[builder(into)]
    pub r#status: Option<String>,
    /// A list of status of the event. For example, `Started`, `Failed`, or `Succeeded`.
    /// 
    /// > **NOTE:** `status` and `statuses` are mutually exclusive.
    #[builder(into)]
    pub r#statuses: Option<Vec<String>>,
    /// The sub status of the event.
    #[builder(into)]
    pub r#sub_status: Option<String>,
    /// A list of sub status of the event.
    /// 
    /// > **NOTE:** `sub_status` and `sub_statuses` are mutually exclusive.
    #[builder(into)]
    pub r#sub_statuses: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActivityLogAlertCriteria {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
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
                    "operationName",
                    &self.r#operation_name,
                ),
                to_pulumi_object_field(
                    "recommendationCategory",
                    &self.r#recommendation_category,
                ),
                to_pulumi_object_field(
                    "recommendationImpact",
                    &self.r#recommendation_impact,
                ),
                to_pulumi_object_field(
                    "recommendationType",
                    &self.r#recommendation_type,
                ),
                to_pulumi_object_field(
                    "resourceGroup",
                    &self.r#resource_group,
                ),
                to_pulumi_object_field(
                    "resourceGroups",
                    &self.r#resource_groups,
                ),
                to_pulumi_object_field(
                    "resourceHealth",
                    &self.r#resource_health,
                ),
                to_pulumi_object_field(
                    "resourceId",
                    &self.r#resource_id,
                ),
                to_pulumi_object_field(
                    "resourceIds",
                    &self.r#resource_ids,
                ),
                to_pulumi_object_field(
                    "resourceProvider",
                    &self.r#resource_provider,
                ),
                to_pulumi_object_field(
                    "resourceProviders",
                    &self.r#resource_providers,
                ),
                to_pulumi_object_field(
                    "resourceType",
                    &self.r#resource_type,
                ),
                to_pulumi_object_field(
                    "resourceTypes",
                    &self.r#resource_types,
                ),
                to_pulumi_object_field(
                    "serviceHealth",
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
                    "subStatus",
                    &self.r#sub_status,
                ),
                to_pulumi_object_field(
                    "subStatuses",
                    &self.r#sub_statuses,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("operationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'operationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_category: {
                        let field_value = match fields_map.get("recommendationCategory") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendationCategory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_impact: {
                        let field_value = match fields_map.get("recommendationImpact") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendationImpact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#recommendation_type: {
                        let field_value = match fields_map.get("recommendationType") {
                            Some(value) => value,
                            None => bail!("Missing field 'recommendationType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_group: {
                        let field_value = match fields_map.get("resourceGroup") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceGroup' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_groups: {
                        let field_value = match fields_map.get("resourceGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_health: {
                        let field_value = match fields_map.get("resourceHealth") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceHealth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id: {
                        let field_value = match fields_map.get("resourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_ids: {
                        let field_value = match fields_map.get("resourceIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_provider: {
                        let field_value = match fields_map.get("resourceProvider") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceProvider' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_providers: {
                        let field_value = match fields_map.get("resourceProviders") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceProviders' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_type: {
                        let field_value = match fields_map.get("resourceType") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resourceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_health: {
                        let field_value = match fields_map.get("serviceHealth") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceHealth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("subStatus") {
                            Some(value) => value,
                            None => bail!("Missing field 'subStatus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_statuses: {
                        let field_value = match fields_map.get("subStatuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'subStatuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

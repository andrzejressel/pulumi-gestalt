#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StackInstancesStackInstanceSummary {
    /// Account ID in which the instance is deployed.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// Detailed status of the stack instance. Values include `PENDING`, `RUNNING`, `SUCCEEDED`, `FAILED`, `CANCELLED`, `INOPERABLE`, `SKIPPED_SUSPENDED_ACCOUNT`, `FAILED_IMPORT`.
    #[builder(into)]
    #[serde(rename = "detailedStatus")]
    pub r#detailed_status: Option<String>,
    /// Status of the stack instance's actual configuration compared to the expected template and parameter configuration of the stack set to which it belongs. Values include `DRIFTED`, `IN_SYNC`, `UNKNOWN`, `NOT_CHECKED`.
    #[builder(into)]
    #[serde(rename = "driftStatus")]
    pub r#drift_status: Option<String>,
    /// Organization root ID or organizational unit (OU) IDs that you specified for `deployment_targets`.
    #[builder(into)]
    #[serde(rename = "organizationalUnitId")]
    pub r#organizational_unit_id: Option<String>,
    /// Region that the stack instance is associated with.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// ID of the stack instance.
    #[builder(into)]
    #[serde(rename = "stackId")]
    pub r#stack_id: Option<String>,
    /// Name or unique ID of the stack set that the stack instance is associated with.
    #[builder(into)]
    #[serde(rename = "stackSetId")]
    pub r#stack_set_id: Option<String>,
    /// Status of the stack instance, in terms of its synchronization with its associated stack set. Values include `CURRENT`, `OUTDATED`, `INOPERABLE`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Explanation for the specific status code assigned to this stack instance.
    #[builder(into)]
    #[serde(rename = "statusReason")]
    pub r#status_reason: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StackInstancesStackInstanceSummary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_id,
                )
                .await,
            );
            map.insert(
                "detailed_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detailed_status,
                )
                .await,
            );
            map.insert(
                "drift_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#drift_status,
                )
                .await,
            );
            map.insert(
                "organizational_unit_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organizational_unit_id,
                )
                .await,
            );
            map.insert(
                "region".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#region,
                )
                .await,
            );
            map.insert(
                "stack_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stack_id,
                )
                .await,
            );
            map.insert(
                "stack_set_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stack_set_id,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "status_reason".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status_reason,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StackInstancesStackInstanceSummary {
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
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_status: {
                        let field_value = match fields_map.get("detailed_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drift_status: {
                        let field_value = match fields_map.get("drift_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'drift_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit_id: {
                        let field_value = match fields_map.get("organizational_unit_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_id: {
                        let field_value = match fields_map.get("stack_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'stack_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stack_set_id: {
                        let field_value = match fields_map.get("stack_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'stack_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#status_reason: {
                        let field_value = match fields_map.get("status_reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'status_reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

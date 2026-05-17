#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudVmClusterIormConfigCach {
    /// A `db_plans` block as defined above.
    #[builder(into)]
    #[serde(rename = "dbPlans")]
    pub r#db_plans: Vec<super::super::types::oracle::GetCloudVmClusterIormConfigCachDbPlan>,
    /// Additional information about the current `lifecycleState`.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: String,
    /// The current state of IORM configuration for the Exadata DB system.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: String,
    /// The current value for the IORM objective. The default is `AUTO`.
    #[builder(into)]
    #[serde(rename = "objective")]
    pub r#objective: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCloudVmClusterIormConfigCach {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "db_plans".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#db_plans,
                )
                .await,
            );
            map.insert(
                "lifecycle_details".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_details,
                )
                .await,
            );
            map.insert(
                "lifecycle_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_state,
                )
                .await,
            );
            map.insert(
                "objective".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#objective,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCloudVmClusterIormConfigCach {
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
                    r#db_plans: {
                        let field_value = match fields_map.get("db_plans") {
                            Some(value) => value,
                            None => bail!("Missing field 'db_plans' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_details: {
                        let field_value = match fields_map.get("lifecycle_details") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_state: {
                        let field_value = match fields_map.get("lifecycle_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#objective: {
                        let field_value = match fields_map.get("objective") {
                            Some(value) => value,
                            None => bail!("Missing field 'objective' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

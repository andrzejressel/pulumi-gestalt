#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterMaintenancePolicyMaintenanceExclusion {
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: String,
    #[builder(into)]
    #[serde(rename = "exclusionName")]
    pub r#exclusion_name: String,
    /// Maintenance exclusion related options.
    #[builder(into)]
    #[serde(rename = "exclusionOptions")]
    pub r#exclusion_options: Vec<super::super::types::container::GetClusterMaintenancePolicyMaintenanceExclusionExclusionOption>,
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterMaintenancePolicyMaintenanceExclusion {
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
                "end_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_time,
                )
                .await,
            );
            map.insert(
                "exclusion_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusion_name,
                )
                .await,
            );
            map.insert(
                "exclusion_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusion_options,
                )
                .await,
            );
            map.insert(
                "start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterMaintenancePolicyMaintenanceExclusion {
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
                    r#end_time: {
                        let field_value = match fields_map.get("end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusion_name: {
                        let field_value = match fields_map.get("exclusion_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusion_options: {
                        let field_value = match fields_map.get("exclusion_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

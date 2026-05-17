#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAcceleratorAttribute {
    #[builder(into)]
    #[serde(rename = "flowLogsEnabled")]
    pub r#flow_logs_enabled: bool,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Bucket")]
    pub r#flow_logs_s_3_bucket: String,
    #[builder(into)]
    #[serde(rename = "flowLogsS3Prefix")]
    pub r#flow_logs_s_3_prefix: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAcceleratorAttribute {
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
                "flow_logs_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flow_logs_enabled,
                )
                .await,
            );
            map.insert(
                "flow_logs_s_3_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flow_logs_s_3_bucket,
                )
                .await,
            );
            map.insert(
                "flow_logs_s_3_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flow_logs_s_3_prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAcceleratorAttribute {
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
                    r#flow_logs_enabled: {
                        let field_value = match fields_map.get("flow_logs_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'flow_logs_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flow_logs_s_3_bucket: {
                        let field_value = match fields_map.get("flow_logs_s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'flow_logs_s_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flow_logs_s_3_prefix: {
                        let field_value = match fields_map.get("flow_logs_s_3_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'flow_logs_s_3_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

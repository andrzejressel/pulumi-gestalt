#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ObjectLambdaAccessPointConfiguration {
    /// Allowed features. Valid values: `GetObject-Range`, `GetObject-PartNumber`.
    #[builder(into)]
    #[serde(rename = "allowedFeatures")]
    pub r#allowed_features: Option<Vec<String>>,
    /// Whether or not the CloudWatch metrics configuration is enabled.
    #[builder(into)]
    #[serde(rename = "cloudWatchMetricsEnabled")]
    pub r#cloud_watch_metrics_enabled: Option<bool>,
    /// Standard access point associated with the Object Lambda Access Point.
    #[builder(into)]
    #[serde(rename = "supportingAccessPoint")]
    pub r#supporting_access_point: String,
    /// List of transformation configurations for the Object Lambda Access Point. See Transformation Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "transformationConfigurations")]
    pub r#transformation_configurations: Vec<super::super::types::s3control::ObjectLambdaAccessPointConfigurationTransformationConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ObjectLambdaAccessPointConfiguration {
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
                "allowed_features".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_features,
                )
                .await,
            );
            map.insert(
                "cloud_watch_metrics_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_watch_metrics_enabled,
                )
                .await,
            );
            map.insert(
                "supporting_access_point".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#supporting_access_point,
                )
                .await,
            );
            map.insert(
                "transformation_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transformation_configurations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ObjectLambdaAccessPointConfiguration {
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
                    r#allowed_features: {
                        let field_value = match fields_map.get("allowed_features") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_features' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_watch_metrics_enabled: {
                        let field_value = match fields_map.get("cloud_watch_metrics_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_watch_metrics_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supporting_access_point: {
                        let field_value = match fields_map.get("supporting_access_point") {
                            Some(value) => value,
                            None => bail!("Missing field 'supporting_access_point' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transformation_configurations: {
                        let field_value = match fields_map.get("transformation_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'transformation_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

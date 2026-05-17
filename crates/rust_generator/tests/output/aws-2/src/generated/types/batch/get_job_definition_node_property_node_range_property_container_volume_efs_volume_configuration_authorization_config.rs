#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig {
    /// The Amazon EFS access point ID to use.
    #[builder(into)]
    #[serde(rename = "accessPointId")]
    pub r#access_point_id: String,
    /// Whether or not to use the AWS Batch job IAM role defined in a job definition when mounting the Amazon EFS file system.
    #[builder(into)]
    #[serde(rename = "iam")]
    pub r#iam: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig {
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
                "access_point_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_point_id,
                )
                .await,
            );
            map.insert(
                "iam".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iam,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig {
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
                    r#access_point_id: {
                        let field_value = match fields_map.get("access_point_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_point_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam: {
                        let field_value = match fields_map.get("iam") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

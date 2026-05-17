#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskSparkInfrastructureSpecVpcNetwork {
    /// The Cloud VPC network in which the job is run. By default, the Cloud VPC network named Default within the project is used.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// List of network tags to apply to the job.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Option<Vec<String>>,
    /// The Cloud VPC sub-network in which the job is run.
    #[builder(into)]
    #[serde(rename = "subNetwork")]
    pub r#sub_network: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskSparkInfrastructureSpecVpcNetwork {
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
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "network_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_tags,
                )
                .await,
            );
            map.insert(
                "sub_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sub_network,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskSparkInfrastructureSpecVpcNetwork {
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
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_tags: {
                        let field_value = match fields_map.get("network_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_network: {
                        let field_value = match fields_map.get("sub_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'sub_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOrigin {
    /// The name of the origin resource. Your origin can be an instance with an attached static IP, a bucket, or a load balancer that has at least one instance attached to it.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The protocol that your Amazon Lightsail distribution uses when establishing a connection with your origin to pull content.
    #[builder(into)]
    #[serde(rename = "protocolPolicy")]
    pub r#protocol_policy: Option<String>,
    /// The AWS Region name of the origin resource.
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: String,
    /// The resource type of the origin resource (e.g., Instance).
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOrigin {
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
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "protocol_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol_policy,
                )
                .await,
            );
            map.insert(
                "region_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#region_name,
                )
                .await,
            );
            map.insert(
                "resource_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOrigin {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol_policy: {
                        let field_value = match fields_map.get("protocol_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_name: {
                        let field_value = match fields_map.get("region_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'region_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

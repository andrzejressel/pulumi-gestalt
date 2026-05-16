#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOriginVpcOriginConfig {
    #[builder(into)]
    #[serde(rename = "originKeepaliveTimeout")]
    pub r#origin_keepalive_timeout: Option<i32>,
    #[builder(into)]
    #[serde(rename = "originReadTimeout")]
    pub r#origin_read_timeout: Option<i32>,
    /// The VPC origin ID.
    #[builder(into)]
    #[serde(rename = "vpcOriginId")]
    pub r#vpc_origin_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOriginVpcOriginConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("origin_keepalive_timeout".to_string(), self.r#origin_keepalive_timeout.to_pulumi_value().await);
            map.insert("origin_read_timeout".to_string(), self.r#origin_read_timeout.to_pulumi_value().await);
            map.insert("vpc_origin_id".to_string(), self.r#vpc_origin_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOriginVpcOriginConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#origin_keepalive_timeout: {
                        let field_value = match fields_map.get("origin_keepalive_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_keepalive_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_read_timeout: {
                        let field_value = match fields_map.get("origin_read_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_read_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vpc_origin_id: {
                        let field_value = match fields_map.get("vpc_origin_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_origin_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

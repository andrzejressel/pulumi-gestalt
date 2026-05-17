#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetNodeGroupScalingConfig {
    /// Desired number of worker nodes.
    #[builder(into)]
    #[serde(rename = "desiredSize")]
    pub r#desired_size: i32,
    /// Maximum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "maxSize")]
    pub r#max_size: i32,
    /// Minimum number of worker nodes.
    #[builder(into)]
    #[serde(rename = "minSize")]
    pub r#min_size: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetNodeGroupScalingConfig {
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
                "desired_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#desired_size,
                )
                .await,
            );
            map.insert(
                "max_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_size,
                )
                .await,
            );
            map.insert(
                "min_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_size,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetNodeGroupScalingConfig {
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
                    r#desired_size: {
                        let field_value = match fields_map.get("desired_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_size: {
                        let field_value = match fields_map.get("max_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_size: {
                        let field_value = match fields_map.get("min_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

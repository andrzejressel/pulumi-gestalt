#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterDataplaneV2 {
    /// Enable advanced networking which requires dataplane_v2_enabled to be set true.
    #[builder(into)]
    #[serde(rename = "advancedNetworking")]
    pub r#advanced_networking: Option<bool>,
    /// Enables Dataplane V2.
    #[builder(into)]
    #[serde(rename = "dataplaneV2Enabled")]
    pub r#dataplane_v_2_enabled: Option<bool>,
    /// Enable Dataplane V2 for clusters with Windows nodes.
    #[builder(into)]
    #[serde(rename = "windowsDataplaneV2Enabled")]
    pub r#windows_dataplane_v_2_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterDataplaneV2 {
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
                "advanced_networking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#advanced_networking,
                )
                .await,
            );
            map.insert(
                "dataplane_v_2_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dataplane_v_2_enabled,
                )
                .await,
            );
            map.insert(
                "windows_dataplane_v_2_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows_dataplane_v_2_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterDataplaneV2 {
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
                    r#advanced_networking: {
                        let field_value = match fields_map.get("advanced_networking") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_networking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dataplane_v_2_enabled: {
                        let field_value = match fields_map.get("dataplane_v_2_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataplane_v_2_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_dataplane_v_2_enabled: {
                        let field_value = match fields_map.get("windows_dataplane_v_2_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_dataplane_v_2_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

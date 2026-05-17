#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkSimPolicySlice {
    /// An array of `data_network` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataNetworks")]
    pub r#data_networks: Vec<super::super::types::mobile::NetworkSimPolicySliceDataNetwork>,
    /// The ID of default data network to use if the user equipment does not explicitly specify it. Configuration for this object must exist in the `data_network` block.
    #[builder(into)]
    #[serde(rename = "defaultDataNetworkId")]
    pub r#default_data_network_id: String,
    /// The ID of the slice that these settings apply to.
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkSimPolicySlice {
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
                "data_networks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_networks,
                )
                .await,
            );
            map.insert(
                "default_data_network_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_data_network_id,
                )
                .await,
            );
            map.insert(
                "slice_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#slice_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkSimPolicySlice {
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
                    r#data_networks: {
                        let field_value = match fields_map.get("data_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_data_network_id: {
                        let field_value = match fields_map.get("default_data_network_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_data_network_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slice_id: {
                        let field_value = match fields_map.get("slice_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'slice_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

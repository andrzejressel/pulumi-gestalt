#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataExchangeSharingEnvironmentConfig {
    /// Data Clean Room (DCR), used for privacy-safe and secured data sharing.
    #[builder(into)]
    #[serde(rename = "dcrExchangeConfig")]
    pub r#dcr_exchange_config: Option<Box<super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfigDcrExchangeConfig>>,
    /// Default Analytics Hub data exchange, used for secured data sharing.
    #[builder(into)]
    #[serde(rename = "defaultExchangeConfig")]
    pub r#default_exchange_config: Option<Box<super::super::types::bigqueryanalyticshub::DataExchangeSharingEnvironmentConfigDefaultExchangeConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataExchangeSharingEnvironmentConfig {
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
                "dcr_exchange_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dcr_exchange_config,
                )
                .await,
            );
            map.insert(
                "default_exchange_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_exchange_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataExchangeSharingEnvironmentConfig {
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
                    r#dcr_exchange_config: {
                        let field_value = match fields_map.get("dcr_exchange_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dcr_exchange_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_exchange_config: {
                        let field_value = match fields_map.get("default_exchange_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_exchange_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

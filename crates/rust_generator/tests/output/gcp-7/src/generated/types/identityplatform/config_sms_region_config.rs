#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigSmsRegionConfig {
    /// A policy of allowing SMS to every region by default and adding disallowed regions to a disallow list.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowByDefault")]
    pub r#allow_by_default: Option<Box<super::super::types::identityplatform::ConfigSmsRegionConfigAllowByDefault>>,
    /// A policy of only allowing regions by explicitly adding them to an allowlist.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowlistOnly")]
    pub r#allowlist_only: Option<Box<super::super::types::identityplatform::ConfigSmsRegionConfigAllowlistOnly>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigSmsRegionConfig {
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
                "allow_by_default".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_by_default,
                )
                .await,
            );
            map.insert(
                "allowlist_only".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowlist_only,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigSmsRegionConfig {
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
                    r#allow_by_default: {
                        let field_value = match fields_map.get("allow_by_default") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_by_default' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allowlist_only: {
                        let field_value = match fields_map.get("allowlist_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowlist_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

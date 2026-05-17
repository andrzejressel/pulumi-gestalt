#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ManagementPolicyRuleActions {
    /// A `base_blob` block as documented below.
    #[builder(into)]
    #[serde(rename = "baseBlob")]
    pub r#base_blob: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsBaseBlob>>,
    /// A `snapshot` block as documented below.
    #[builder(into)]
    #[serde(rename = "snapshot")]
    pub r#snapshot: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsSnapshot>>,
    /// A `version` block as documented below.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<Box<super::super::types::storage::ManagementPolicyRuleActionsVersion>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ManagementPolicyRuleActions {
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
                "base_blob".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#base_blob,
                )
                .await,
            );
            map.insert(
                "snapshot".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#snapshot,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ManagementPolicyRuleActions {
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
                    r#base_blob: {
                        let field_value = match fields_map.get("base_blob") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_blob' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot: {
                        let field_value = match fields_map.get("snapshot") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

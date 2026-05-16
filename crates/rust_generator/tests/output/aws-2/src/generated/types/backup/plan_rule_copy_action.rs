#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanRuleCopyAction {
    /// An Amazon Resource Name (ARN) that uniquely identifies the destination backup vault for the copied backup.
    #[builder(into)]
    #[serde(rename = "destinationVaultArn")]
    pub r#destination_vault_arn: String,
    /// The lifecycle defines when a protected resource is copied over to a backup vault and when it expires.  Fields documented above.
    #[builder(into)]
    #[serde(rename = "lifecycle")]
    pub r#lifecycle: Option<Box<super::super::types::backup::PlanRuleCopyActionLifecycle>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PlanRuleCopyAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("destination_vault_arn".to_string(), self.r#destination_vault_arn.to_pulumi_value().await);
            map.insert("lifecycle".to_string(), self.r#lifecycle.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PlanRuleCopyAction {
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
                    r#destination_vault_arn: {
                        let field_value = match fields_map.get("destination_vault_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_vault_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lifecycle: {
                        let field_value = match fields_map.get("lifecycle") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::backup::PlanRuleCopyActionLifecycle>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

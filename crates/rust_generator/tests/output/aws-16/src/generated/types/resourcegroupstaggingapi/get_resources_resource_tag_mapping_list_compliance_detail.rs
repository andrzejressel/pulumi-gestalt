#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetResourcesResourceTagMappingListComplianceDetail {
    /// Whether the resource is compliant.
    /// * `keys_with_noncompliant_values ` - Set of tag keys with non-compliant tag values.
    /// * `non_compliant_keys ` - Set of non-compliant tag keys.
    #[builder(into)]
    #[serde(rename = "complianceStatus")]
    pub r#compliance_status: bool,
    #[builder(into)]
    #[serde(rename = "keysWithNoncompliantValues")]
    pub r#keys_with_noncompliant_values: Vec<String>,
    #[builder(into)]
    #[serde(rename = "nonCompliantKeys")]
    pub r#non_compliant_keys: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetResourcesResourceTagMappingListComplianceDetail {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("compliance_status".to_string(), self.r#compliance_status.to_pulumi_value().await);
            map.insert("keys_with_noncompliant_values".to_string(), self.r#keys_with_noncompliant_values.to_pulumi_value().await);
            map.insert("non_compliant_keys".to_string(), self.r#non_compliant_keys.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetResourcesResourceTagMappingListComplianceDetail {
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
                    r#compliance_status: {
                        let field_value = match fields_map.get("compliance_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'compliance_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#keys_with_noncompliant_values: {
                        let field_value = match fields_map.get("keys_with_noncompliant_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'keys_with_noncompliant_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#non_compliant_keys: {
                        let field_value = match fields_map.get("non_compliant_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'non_compliant_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

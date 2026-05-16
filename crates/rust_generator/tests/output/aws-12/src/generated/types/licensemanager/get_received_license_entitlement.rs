#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReceivedLicenseEntitlement {
    /// Indicates whether check-ins are allowed.
    #[builder(into)]
    #[serde(rename = "allowCheckIn")]
    pub r#allow_check_in: bool,
    /// Maximum entitlement count. Use if the unit is not None.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: i32,
    /// The key name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates whether overages are allowed.
    #[builder(into)]
    #[serde(rename = "overage")]
    pub r#overage: bool,
    /// Entitlement unit.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
    /// The value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetReceivedLicenseEntitlement {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allow_check_in".to_string(), self.r#allow_check_in.to_pulumi_value().await);
            map.insert("max_count".to_string(), self.r#max_count.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("overage".to_string(), self.r#overage.to_pulumi_value().await);
            map.insert("unit".to_string(), self.r#unit.to_pulumi_value().await);
            map.insert("value".to_string(), self.r#value.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetReceivedLicenseEntitlement {
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
                    r#allow_check_in: {
                        let field_value = match fields_map.get("allow_check_in") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_check_in' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_count: {
                        let field_value = match fields_map.get("max_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#overage: {
                        let field_value = match fields_map.get("overage") {
                            Some(value) => value,
                            None => bail!("Missing field 'overage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#unit: {
                        let field_value = match fields_map.get("unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#value: {
                        let field_value = match fields_map.get("value") {
                            Some(value) => value,
                            None => bail!("Missing field 'value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

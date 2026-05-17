#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ShareAclAccessPolicy {
    /// The time at which this Access Policy should be valid untilWhen using `storage_account_id` this should be in RFC3339 format. If using the deprecated `storage_account_name` property, this uses the [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Option<String>,
    /// The permissions which should be associated with this Shared Identifier. Possible value is combination of `r` (read), `w` (write), `d` (delete), and `l` (list).
    /// 
    /// > **Note:** Permission order is strict at the service side, and permissions need to be listed in the order above.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: String,
    /// The time at which this Access Policy should be valid from. When using `storage_account_id` this should be in RFC3339 format. If using the deprecated `storage_account_name` property, this uses the [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ShareAclAccessPolicy {
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
                "expiry".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expiry,
                )
                .await,
            );
            map.insert(
                "permissions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permissions,
                )
                .await,
            );
            map.insert(
                "start".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ShareAclAccessPolicy {
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
                    r#expiry: {
                        let field_value = match fields_map.get("expiry") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiry' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permissions: {
                        let field_value = match fields_map.get("permissions") {
                            Some(value) => value,
                            None => bail!("Missing field 'permissions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start: {
                        let field_value = match fields_map.get("start") {
                            Some(value) => value,
                            None => bail!("Missing field 'start' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

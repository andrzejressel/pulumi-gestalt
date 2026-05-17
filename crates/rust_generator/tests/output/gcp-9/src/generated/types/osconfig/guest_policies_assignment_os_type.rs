#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesAssignmentOsType {
    /// Targets VM instances with OS Inventory enabled and having the following OS architecture.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "osArchitecture")]
    pub r#os_architecture: Option<String>,
    /// Targets VM instances with OS Inventory enabled and having the following OS short name, for example "debian" or "windows".
    #[builder(into)]
    #[serde(rename = "osShortName")]
    pub r#os_short_name: Option<String>,
    /// Targets VM instances with OS Inventory enabled and having the following following OS version.
    #[builder(into)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesAssignmentOsType {
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
                "os_architecture".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_architecture,
                )
                .await,
            );
            map.insert(
                "os_short_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_short_name,
                )
                .await,
            );
            map.insert(
                "os_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesAssignmentOsType {
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
                    r#os_architecture: {
                        let field_value = match fields_map.get("os_architecture") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_architecture' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_short_name: {
                        let field_value = match fields_map.get("os_short_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_short_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_version: {
                        let field_value = match fields_map.get("os_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

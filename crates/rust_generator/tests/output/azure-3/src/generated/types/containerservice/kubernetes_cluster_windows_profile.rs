#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterWindowsProfile {
    /// The Admin Password for Windows VMs. Length must be between 14 and 123 characters.
    #[builder(into)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: String,
    /// The Admin Username for Windows VMs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: String,
    /// A `gmsa` block as defined below.
    #[builder(into)]
    #[serde(rename = "gmsa")]
    pub r#gmsa: Option<Box<super::super::types::containerservice::KubernetesClusterWindowsProfileGmsa>>,
    /// Specifies the type of on-premise license which should be used for Node Pool Windows Virtual Machine. At this time the only possible value is `Windows_Server`.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterWindowsProfile {
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
                "admin_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#admin_password,
                )
                .await,
            );
            map.insert(
                "admin_username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#admin_username,
                )
                .await,
            );
            map.insert(
                "gmsa".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gmsa,
                )
                .await,
            );
            map.insert(
                "license".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#license,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterWindowsProfile {
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
                    r#admin_password: {
                        let field_value = match fields_map.get("admin_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#admin_username: {
                        let field_value = match fields_map.get("admin_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gmsa: {
                        let field_value = match fields_map.get("gmsa") {
                            Some(value) => value,
                            None => bail!("Missing field 'gmsa' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#license: {
                        let field_value = match fields_map.get("license") {
                            Some(value) => value,
                            None => bail!("Missing field 'license' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

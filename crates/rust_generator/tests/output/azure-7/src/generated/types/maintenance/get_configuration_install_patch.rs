#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationInstallPatch {
    /// A `linux` block as defined below.
    #[builder(into)]
    #[serde(rename = "linuxes")]
    pub r#linuxes: Vec<super::super::types::maintenance::GetConfigurationInstallPatchLinux>,
    /// Possible reboot preference as defined by the user based on which it would be decided to reboot the machine or not after the patch operation is completed.
    #[builder(into)]
    #[serde(rename = "reboot")]
    pub r#reboot: String,
    /// A `windows` block as defined below.
    #[builder(into)]
    #[serde(rename = "windows")]
    pub r#windows: Vec<super::super::types::maintenance::GetConfigurationInstallPatchWindow>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConfigurationInstallPatch {
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
                "linuxes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linuxes,
                )
                .await,
            );
            map.insert(
                "reboot".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reboot,
                )
                .await,
            );
            map.insert(
                "windows".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConfigurationInstallPatch {
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
                    r#linuxes: {
                        let field_value = match fields_map.get("linuxes") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reboot: {
                        let field_value = match fields_map.get("reboot") {
                            Some(value) => value,
                            None => bail!("Missing field 'reboot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows: {
                        let field_value = match fields_map.get("windows") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

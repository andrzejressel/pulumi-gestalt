#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationLinux {
    /// Specifies the list of update classifications included in the Software Update Configuration. Possible values are `Unclassified`, `Critical`, `Security` and `Other`.
    /// 
    /// > **NOTE:** The `classifications_included` property will become `Required` in version 4.0 of the Provider.
    #[builder(into)]
    #[serde(rename = "classificationsIncludeds")]
    pub r#classifications_includeds: Vec<String>,
    /// Specifies a list of packages to excluded from the Software Update Configuration.
    #[builder(into)]
    #[serde(rename = "excludedPackages")]
    pub r#excluded_packages: Option<Vec<String>>,
    /// Specifies a list of packages to included from the Software Update Configuration.
    #[builder(into)]
    #[serde(rename = "includedPackages")]
    pub r#included_packages: Option<Vec<String>>,
    /// Specifies the reboot settings after software update, possible values are `IfRequired`, `Never`, `RebootOnly` and `Always`. Defaults to `IfRequired`.
    #[builder(into)]
    #[serde(rename = "reboot")]
    pub r#reboot: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SoftwareUpdateConfigurationLinux {
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
                "classifications_includeds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classifications_includeds,
                )
                .await,
            );
            map.insert(
                "excluded_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_packages,
                )
                .await,
            );
            map.insert(
                "included_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#included_packages,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SoftwareUpdateConfigurationLinux {
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
                    r#classifications_includeds: {
                        let field_value = match fields_map.get("classifications_includeds") {
                            Some(value) => value,
                            None => bail!("Missing field 'classifications_includeds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_packages: {
                        let field_value = match fields_map.get("excluded_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_packages: {
                        let field_value = match fields_map.get("included_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

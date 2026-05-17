#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpokeLinkedRouterApplianceInstances {
    /// IP ranges allowed to be included during import from hub (does not control transit connectivity).
    /// The only allowed value for now is "ALL_IPV4_RANGES".
    #[builder(into)]
    #[serde(rename = "includeImportRanges")]
    pub r#include_import_ranges: Option<Vec<String>>,
    /// The list of router appliance instances
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "instances")]
    pub r#instances: Vec<super::super::types::networkconnectivity::SpokeLinkedRouterApplianceInstancesInstance>,
    /// A value that controls whether site-to-site data transfer is enabled for these resources. Note that data transfer is available only in supported locations.
    #[builder(into)]
    #[serde(rename = "siteToSiteDataTransfer")]
    pub r#site_to_site_data_transfer: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpokeLinkedRouterApplianceInstances {
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
                "include_import_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_import_ranges,
                )
                .await,
            );
            map.insert(
                "instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instances,
                )
                .await,
            );
            map.insert(
                "site_to_site_data_transfer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#site_to_site_data_transfer,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpokeLinkedRouterApplianceInstances {
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
                    r#include_import_ranges: {
                        let field_value = match fields_map.get("include_import_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_import_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instances: {
                        let field_value = match fields_map.get("instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#site_to_site_data_transfer: {
                        let field_value = match fields_map.get("site_to_site_data_transfer") {
                            Some(value) => value,
                            None => bail!("Missing field 'site_to_site_data_transfer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SharedImageGallerySharing {
    /// A `community_gallery` block as defined below. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `community_gallery` must be set when `permission` is set to `Community`.
    #[builder(into)]
    #[serde(rename = "communityGallery")]
    pub r#community_gallery: Option<Box<super::super::types::compute::SharedImageGallerySharingCommunityGallery>>,
    /// The permission of the Shared Image Gallery when sharing. Possible values are `Community`, `Groups` and `Private`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This requires that the Preview Feature `Microsoft.Compute/CommunityGalleries` is enabled, see [the documentation](https://learn.microsoft.com/azure/virtual-machines/share-gallery-community?tabs=cli) for more information.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SharedImageGallerySharing {
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
                "community_gallery".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#community_gallery,
                )
                .await,
            );
            map.insert(
                "permission".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#permission,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SharedImageGallerySharing {
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
                    r#community_gallery: {
                        let field_value = match fields_map.get("community_gallery") {
                            Some(value) => value,
                            None => bail!("Missing field 'community_gallery' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permission: {
                        let field_value = match fields_map.get("permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}

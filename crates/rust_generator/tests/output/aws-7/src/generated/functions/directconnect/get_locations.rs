#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod get_locations {
    #[allow(dead_code)]
    pub struct GetLocationsResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Code for the locations.
        pub location_codes: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(context: &pulumi_gestalt_rust::Context) -> GetLocationsResult {
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directconnect/getLocations:getLocations".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetLocationsResult {
            id: o.get_field("id"),
            location_codes: o.get_field("locationCodes"),
        }
    }
}

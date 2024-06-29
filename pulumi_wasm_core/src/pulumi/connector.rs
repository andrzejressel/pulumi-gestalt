pub trait PulumiConnector {
    fn is_in_preview(&self) -> bool;
    fn create_resource(&self, output_id: String, req: Vec<u8>);
    fn get_created_resources(&self) -> Vec<(String, Vec<u8>)>;
}

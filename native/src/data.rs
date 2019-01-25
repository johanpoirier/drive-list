#[derive(Debug, Clone, Serialize)]
pub struct Drive {
    pub drive_type: String,
    pub mounted_from: String,
    pub mounted_on: String,
}

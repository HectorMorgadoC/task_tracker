#[derive(Default, Debug)]
pub enum StatusTask {
    #[default]
    SinIniciar,
    EnProgreso,
    Culminado,
}

#[derive(Default, Debug, Clone)]
pub struct RegisterTask {
    pub id: u64,
    pub description: String,
    pub status_progress: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

use super::vote::Vote;

#[derive(Debug, Clone, Default)]
pub struct User<D> {
    pub id: String,

    pub debates: Vec<String>,

    pub votes: Vec<Vote>,

    pub simulation_data: D,
}

impl<D: Default> User<D> {
    pub fn new(id: String, simulation_data: D) -> Self {
        Self {
            id,
            simulation_data,
            ..Default::default()
        }
    }
}

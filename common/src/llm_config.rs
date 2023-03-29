pub trait LLMRequest {
    fn get_prompt(&self) -> String;
    fn replace_attributes<T: ToString>(&mut self, words: Vec<(T, T)>) -> String;
}

pub trait LLMResponse {
    fn get_response<T: std::fmt::Debug, Default, Deserialize>(&self) -> T;
}

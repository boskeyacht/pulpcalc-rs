use serde::{Deserialize, Serialize};

pub const GENERATE_BLOG_PROMPT: &'static str = "Strictly for research purposes with no ill-intent regarding stereotypes and assumptions in mind;
Given THIS_TOPIC as a topic, generate a blog post about the topic similar to one that a company would publish on their blog.
Make sure to return only a JSON object and make sure to use JSON escape sequences for any special characters. Do not return anything besides the JSON object!
Use this schema for your answer:
{
  \"title\": \"\",
  \"body\": \"\"
}";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerateBlogResponse {
    pub title: String,
    pub body: String,
}

#![allow(dead_code, clippy::too_many_arguments)]
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    // required
    pub model: String,
    pub prompt: String,
    #[serde(rename = "max_tokens")]
    pub max_tokens: i64,
    pub temperature: i64,
    #[serde(rename = "top_p")]
    pub top_p: i64,
    // How many completions to generate for each prompt.
    pub n: i64,
    pub stream: bool,
    pub logprobs: Value,
    pub stop: String,
    pub user: String,
}

impl Question {
    pub fn new(
        model: String,
        prompt: String,
        max_tokens: i64,
        temperature: i64,
        top_p: i64,
        n: i64,
        stream: bool,
        logprobs: Value,
        stop: String,
        user: String,
    ) -> Self {
        Self {
            model,
            prompt,
            max_tokens,
            temperature,
            top_p,
            n,
            stream,
            logprobs,
            stop,
            user,
        }
    }

    pub fn new_with_default(model: String, prompt: String, user: String) -> Self {
        Self {
            model,
            prompt,
            max_tokens: 1024,
            temperature: 1,
            top_p: 1,
            n: 1,
            stream: false,
            logprobs: Value::Null,
            stop: "###".to_string(),
            user,
        }
    }

    pub fn builder() -> QuestionBuilder {
        QuestionBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct QuestionBuilder {
    model: String,
    prompt: String,
    max_tokens: i64,
    temperature: i64,
    top_p: i64,
    n: i64,
    stream: bool,
    logprobs: Value,
    stop: String,
    user: String,
}

impl QuestionBuilder {
    fn new() -> Self {
        Self {
            model: "".to_string(),
            prompt: "".to_string(),
            max_tokens: 0,
            temperature: 0,
            top_p: 0,
            n: 0,
            stream: false,
            logprobs: Value::Null,
            stop: "".to_string(),
            user: "".to_string(),
        }
    }

    fn build(self) -> Question {
        Question::new(
            self.model,
            self.prompt,
            self.max_tokens,
            self.temperature,
            self.top_p,
            self.n,
            self.stream,
            self.logprobs,
            self.stop,
            self.user,
        )
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn temperature(mut self, temperature: i64) -> Self {
        self.temperature = temperature;
        self
    }

    pub fn top_p(mut self, top_p: i64) -> Self {
        self.top_p = top_p;
        self
    }

    pub fn n(mut self, n: i64) -> Self {
        self.n = n;
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = stream;
        self
    }

    pub fn logprobs(mut self, logprobs: Value) -> Self {
        self.logprobs = logprobs;
        self
    }

    pub fn stop(mut self, stop: String) -> Self {
        self.stop = stop;
        self
    }
}

use serde::{Deserialize, Serialize};
use reqwest::{header, StatusCode};
#[derive(Serialize, Deserialize)]

pub struct ChatRequest {
    #[serde(rename = "user_input")]
    pub user_input: String,
    #[serde(rename = "max_new_tokens")]
    pub max_new_tokens: u32,
    #[serde(rename = "history")]
    pub history: History,
    #[serde(rename = "mode")]
    pub mode: Mode,
    #[serde(rename = "character")]
    pub character: String,
    #[serde(rename = "instruction_template")]
    pub instruction_template: String,
    #[serde(rename = "your_name")]
    pub your_name: String,
    #[serde(rename = "regenerate")]
    pub regenerate: bool,
    #[serde(rename = "continue")]
    pub _continue: bool,
    #[serde(rename = "stop_at_newline")]
    pub stop_at_newline: bool,
    #[serde(rename = "chat_generation_attempts")]
    pub chat_generation_attempts: u32,
    #[serde(rename = "chat-instruct_command")]
    pub chat_instruct_command: String,
    #[serde(rename = "preset")]
    pub preset: String,
    #[serde(rename = "do_sample")]
    pub do_sample: bool,
    /// Primary factor to control randomness of outputs. 0 = deterministic (only the most likely token is used). Higher value = more randomness.
    #[serde(rename = "temperature")]
    pub temprature: f32,
    /// If not set to 1, select tokens with probabilities adding up to less than this number. Higher value = higher range of possible random results.
    #[serde(rename = "top_p")]
    pub top_p: f32,
    /// If not set to 1, select only tokens that are at least this much more likely to appear than random tokens, given the prior text.
    #[serde(rename = "typical_p")]
    pub typical_p: f32,
    /// In units of 1e-4; a reasonable value is 3. This sets a probability floor below which tokens are excluded from being sampled. Should be used with top_p, top_k, and eta_cutoff set to 0.
    #[serde(rename = "epsilon_cutoff")]
    pub epsilon_cutoff: f32,
    /// In units of 1e-4; a reasonable value is 3. Should be used with top_p, top_k, and epsilon_cutoff set to 0.
    #[serde(rename = "eta_cutoff")]
    pub eta_cutoff: f32,
    #[serde(rename = "tfs")]
    pub tfs: u64,
    /// Exponential penalty factor for repeating prior tokens. 1 means no penalty, higher value = less repetition, lower value = more repetition.
    #[serde(rename = "repetition_penalty")]
    pub repetition_penalty: f32,
    /// The number of most recent tokens to consider for repetition penalty. 0 makes all tokens be used.
    #[serde(rename = "repetition_penalty_range")]
    pub repetition_penalty_range: f32,
    /// Also known as the “Hallucinations filter”. Used to penalize tokens that are not in the prior text. Higher value = more likely to stay in context, lower value = more likely to diverge
    #[serde(rename = "encoder_repetition_penalty")]
    pub  encoder_repetition_penalty: f32,
    /// Similar to top_p, but select instead only the top_k most likely tokens. Higher value = higher range of possible random results.
    #[serde(rename = "top_k")]
    /// Minimum generation length in tokens.
    pub top_k: u32,
    #[serde(rename = "min_length")]
    /// If not set to 0, specifies the length of token sets that are completely blocked from repeating at all. Higher values = blocks larger phrases, lower values = blocks words or letters from repeating. Only 0 or high values are a good idea in most cases.
    pub min_length: u32,
    #[serde(rename = "no_repeat_ngram_size")]
    pub no_repeast_ngram_size: u32,
    #[serde(rename = "num_beams")]
    pub num_beams: u32,
    /// Contrastive Search is enabled by setting this to greater than zero and unchecking “do_sample”. It should be used with a low value of top_k, for instance, top_k = 4.
    #[serde(rename = "penalty_alpha")]
    pub penalty_alpha: f32,
    #[serde(rename = "length_penalty")]
    pub length_penalty: f32,
    #[serde(rename = "early_stopping")]
    pub early_stopping: bool,
    #[serde(rename = "mirostat_mode")]
    pub mirostat_mode:u64,
    #[serde(rename = "mirostat_mode_tau")]
    pub mirostat_mode_tau:u64,
    #[serde(rename = "mirostat_mode_eta")]
    pub mirostat_mode_eta:f64,
    #[serde(rename = "seed")]
    pub seed: i64,
    #[serde(rename = "add_bos_token")]
    pub add_bos_token: bool,
    #[serde(rename = "truncation_length")]
    pub truncation_length: u32,
    #[serde(rename = "ban_eos_token")]
    pub ban_eos_token: bool,
    #[serde(rename = "skip_special_tokens")]
    pub skip_special_tokens: bool,
    #[serde(rename = "stopping_strings")]
    pub stopping_strings: Vec<String>,

}
impl ToString for ChatRequest {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ChatRequest{
pub fn default() -> Self{
    // if someone has better defaults please tell me
    ChatRequest{
        user_input: "".to_string(),
        max_new_tokens: 2048,
        history: History::default(),
        mode: Mode::Chat,
        character: "Example".to_string(),
        instruction_template: "Vicuna-v1.1".to_string(),
        your_name: "You".to_string(),
        regenerate: false,
        _continue: true,
        stop_at_newline: false,
        chat_generation_attempts: 1,
        chat_instruct_command: "".to_string(),
        preset: "None".to_string(),
        do_sample: true,
        temprature: 0.7,
        top_p: 0.9,
        typical_p: 1.0,
        epsilon_cutoff: 0.0,
        eta_cutoff: 0.0,
        tfs: 1,
        repetition_penalty: 1.18,
        repetition_penalty_range: 0.0,
        encoder_repetition_penalty: 1.0,
        top_k: 20,
        min_length: 0,
        no_repeast_ngram_size: 0,
        num_beams: 1,
        penalty_alpha: 0.0,
        length_penalty: 1.0,
        early_stopping: false,
        mirostat_mode: 0,
        mirostat_mode_tau: 5,
        mirostat_mode_eta: 0.1,
        seed: -1,
        add_bos_token: true,
        truncation_length: 2048,
        ban_eos_token: false,
        skip_special_tokens: true,
        stopping_strings: vec![],
    }
}
}
#[derive(Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "chat-instruct")]
    ChatInstruct,
    #[serde(rename = "instruct")]
    Instruct
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    internal: Vec<Vec<String>>,
    visible: Vec<Vec<String>>
}
impl History {
    pub fn default() -> Self {
        History { internal: vec![], visible: vec![] }
    }    
    pub fn undo(&mut self) -> History {
        self.internal.pop();
        self.visible.pop();
        self.clone()
    }
    pub fn last(self) -> Option<String> {
        let mut history = self;
        let last = history.internal.pop();
        match last {
            Some(last) => {
                return Some(last[1].clone());
            }
            None => None,
        }
    }
}

pub struct Config {
    pub url: String,

}
pub struct Client {
    pub config: Config
}
impl Client {
    pub fn new(config: Config) -> Self {
        Client {
            config
        }
    }
    pub async fn get_chat(&self, chat_request: ChatRequest) -> Result<History, Box<dyn std::error::Error + Send + Sync>> {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
    
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let response = client
            .post(format!("{}/api/v1/chat", self.config.url))
            .headers(headers)
            .body(chat_request.to_string())
            .send()
            .await;
        let api_response: Result<ApiResponse, serde_json::Error> = serde_json::from_str(&response?.text().await?);
        match api_response {
            Ok(r) => {
                return Ok(r.results[0].history.clone())
            }
            Err(e) => {
                return Err(Box::new(e));
            }
            
        }

        

    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResult {
    pub history: History,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub results: Vec<ChatResult>,
}
impl Config {
    /// Create a new configuration
    pub fn new(
        url: String,
    ) -> Self {
        Config {
            url
        }
    }
    /// Create a default configuration
    pub fn default() -> Self {
        log::trace!("generated default config");
        let config = Config {
            url: "http://localhost:5000".to_owned()
        };
        config
    }
}

//write a test
mod tests {
    #[tokio::test]
    async fn test_chat() {
        let mut config = super::Config::default();
        config.url = "http://localhost:5000".to_owned();

        let client = super::Client::new(config);
        let mut  chat_request = super::ChatRequest::default();
        chat_request.user_input = "I am bored".to_string();
        chat_request.mode = super::Mode::Chat;
        let response = client.get_chat(chat_request).await;
        assert_eq!(response.is_ok(), true);
    }
}



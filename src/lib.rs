pub mod palm;
pub use palm::PalmClient;

pub fn create_client(api_key: String) -> PalmClient {
    palm::create_client(api_key)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::palm::{new_message_prompt, new_text_body};

    use super::*;

    #[test]
    fn list_models_works() {
        let my_client = create_client("".to_string());
        let models_list = my_client.list_models().expect("err");
        assert!(models_list.len() > 0);
    }

    #[test]
    fn get_model_works() {
        let my_client = create_client("".to_string());
        let model = my_client
            .get_model("text-bison-001".to_string())
            .expect("err");
        assert_eq!(model.name, "models/text-bison-001");
    }

    #[test]
    fn count_token_works() {
        let my_client = create_client("".to_string());
        let token_count = my_client
            .count_message_tokens(
                "chat-bison-001".to_string(),
                vec![
                    "How many tokens?".to_string(),
                    "For this whole conversation?".to_string(),
                ],
            )
            .expect("err");
        assert_eq!(token_count, 23);
    }

    #[test]
    fn generate_embed_works() {
        let my_client = create_client("".to_string());
        let embeddings = my_client
            .generate_embeddings(
                "embedding-gecko-001".to_string(),
                "say something cool and nice!".to_string(),
            )
            .expect("err");
        assert!(embeddings.len() > 0);
    }

    #[test]
    fn generate_message_works() {
        let my_client = create_client("".to_string());
        let mut message_prompt = new_message_prompt();
        message_prompt.append_example(
            "How are you doing?".to_string(),
            "I am doing absolutely fine!".to_string(),
        );
        message_prompt.append_message("How are you doing?".to_string());
        message_prompt.set_context("Reply in english".to_string());
        let mut config: HashMap<String, String> = HashMap::new();
        config.insert("temperature".to_string(), "0.8".to_string());
        config.insert("top_p".to_string(), "0.56".to_string());
        config.insert("candidate_count".to_string(), "2".to_string());
        // config["top_k"];
        let chat_res = my_client
            .chat("chat-bison-001".to_string(), message_prompt, config)
            .expect("err");
        assert!(chat_res.candidates.unwrap().len() > 0);
    }
}

pub mod query;
pub mod tx;

use anyhow::Result as AResult;
use chatgpt::converse::Conversation;

use std::{fs, path::PathBuf};

use crate::{prompt_query_setup, prompt_tx_setup, whitelist_parser::WhitelistedQueries};

use self::{query::ProtoQuery, tx::ProtoTx};

pub const TOKENFACTORY: &str = "tokenfactory";

/// Defines Proto module
pub trait ProtoModule: AsRef<str> {
    /// Returns an example of how a proto file should be encoded
    fn example() -> (String, String);

    fn postfix() -> &'static str;
}

#[derive(Default, Debug, Clone)]
pub struct Module {
    pub name: String,
    pub tx_url: Option<ProtoTx>,
    pub query_url: Option<ProtoQuery>,
}

fn get_raw_url(github_url: impl AsRef<str>) -> String {
    github_url
        .as_ref()
        .replace("github.com", "raw.githubusercontent.com")
        .replace("/blob", "")
}

impl Module {
    /// Returns the file path for this module
    pub fn path(&self, chain: &str, ty: &str) -> PathBuf {
        let mut path = PathBuf::from("../cosmos-anybuf").join("src").join("types");
        path.push(chain);
        path.push(&format!("{}_{}", self.name, ty));
        path.set_extension("rs");
        path
    }

    pub async fn process_proto_tx(&self, proto: &ProtoTx, chain: &str) -> AResult<()> {
        let conversation = prompt_tx_setup(ProtoTx::example())?;

        let proto = get_raw_url(proto);
        let response = reqwest::get(proto).await?;
        let proto_content = response.text().await?;
        // provide content to gpt to generate the code
        self.write_gpt_answer::<ProtoTx>(conversation, proto_content, chain)
            .await
    }

    pub async fn process_proto_query(
        &self,
        proto: &ProtoQuery,
        chain: &str,
        whitelist_url: &str,
    ) -> AResult<()> {
        let conversation = prompt_query_setup(ProtoTx::example(), WhitelistedQueries::example())?;

        // whitelisted queries
        let whitelist = get_raw_url(whitelist_url);
        let response = reqwest::get(whitelist).await?;
        let whitelist_content = response.text().await?;
        let whitelist_queries = WhitelistedQueries::from_text(&whitelist_content);

        // proto
        let proto = get_raw_url(proto);
        let response = reqwest::get(proto).await?;
        let proto_content = response.text().await?;
        let message = format!("{whitelist_queries}{proto_content}");
        // provide content to gpt to generate the code
        self.write_gpt_answer::<ProtoQuery>(conversation, message, chain)
            .await
    }

    async fn write_gpt_answer<T: ProtoModule>(
        &self,
        mut conversation: Conversation,
        message: String,
        chain: &str,
    ) -> AResult<()> {
        let mut resp = conversation.send_message(message).await?;

        let mut full_resp = String::new();

        full_resp.push_str(&resp.message().content);

        // continue prompt while the response does not end with DONE
        while !full_resp.ends_with("DONE") && !full_resp.ends_with("DONE\n") {
            println!("Response: {}", resp.message().content);
            // prompt the ai to continue
            resp = conversation.send_message("continue").await?;
            full_resp.push_str(&resp.message().content);
        }

        let file = &self.path(chain, T::postfix());

        println!("Writing to file: {}", file.display());
        full_resp = crate::remove_last_line(&full_resp);

        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::File::create(file)?;
        fs::write(file, full_resp).unwrap();
        Ok(())
    }

    // pub async fn process_proto<T: ProtoModule>(
    //     &self,
    //     proto: &T,
    //     chain: &str,
    //     mut conversation: Conversation,
    // ) -> AResult<()> {
    //     // whitelisted queries
    //     let whitelist = get_raw_url(whitelist_url);
    //     let response = reqwest::get(whitelist).await?;
    //     let whitelist_content = response.text().await?;
    //     let whitelist_queries = WhitelistedQueries::from_text(&whitelist_content);
    //     // proto
    //     let proto = get_raw_url(proto);
    //     let response = reqwest::get(proto).await?;
    //     let proto_content = response.text().await?;
    //     // provide content to gpt to generate the code
    //     let message = format!("{whitelist_queries}{proto_content}");
    //     let mut resp = conversation.send_message(message).await?;

    //     let mut full_resp = String::new();

    //     full_resp.push_str(&resp.message().content);

    //     // continue prompt while the response does not end with DONE
    //     while !full_resp.ends_with("DONE") && !full_resp.ends_with("DONE\n") {
    //         println!("Response: {}", resp.message().content);
    //         // prompt the ai to continue
    //         resp = conversation.send_message("continue").await?;
    //         full_resp.push_str(&resp.message().content);
    //     }

    //     let file = &self.path(chain, T::postfix());

    //     println!("Writing to file: {}", file.display());
    //     full_resp = crate::remove_last_line(&full_resp);

    //     if let Some(dir) = file.parent() {
    //         fs::create_dir_all(dir)?;
    //     }
    //     fs::File::create(file)?;
    //     fs::write(file, full_resp).unwrap();
    //     Ok(())
    // }

    /// Process the module's proto files
    pub async fn process_module(&self, chain: &str, whitelist_url: &str) -> AResult<()> {
        if let Some(tx_proto) = &self.tx_url {
            self.process_proto_tx(tx_proto, chain).await?
        }

        if let Some(query_proto) = &self.query_url {
            self.process_proto_query(query_proto, chain, whitelist_url)
                .await?
        }
        Ok(())
    }
}

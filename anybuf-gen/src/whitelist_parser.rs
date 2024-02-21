use std::{fmt::Display, fs};

use nom::{
    bytes::complete::{tag, take_till},
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct WhitelistedQueries<'a> {
    pub whitelist_urls: Vec<&'a str>,
}

impl<'a> WhitelistedQueries<'a> {
    pub fn from_text(text: &'a str) -> Self {
        let (_, mut strings) = many_strings(text).unwrap_or_default();
        // Retain anything that's not a message url
        strings.retain(|&url| url.starts_with("/"));
        Self {
            whitelist_urls: strings,
        }
    }

    pub fn example() -> String {
        fs::read_to_string("example/whitelisted.txt").unwrap()
    }
}

impl<'a> Display for WhitelistedQueries<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "whitelisted_queries: {:?}\n", self.whitelist_urls)
    }
}

fn double_quote_tag(input: &str) -> IResult<&str, &str> {
    tag(r#"""#)(input)
}

fn is_double_quoted(c: char) -> bool {
    c == '"'
}

fn string_content(input: &str) -> IResult<&str, &str> {
    delimited(
        double_quote_tag,
        take_till(is_double_quoted),
        double_quote_tag,
    )(input)
}

fn many_strings(input: &str) -> IResult<&str, Vec<&str>> {
    many1(preceded(take_till(is_double_quoted), string_content))(input)
}

#[cfg(test)]
mod test {
    use super::string_content;
    use super::WhitelistedQueries;

    #[test]
    fn parse_string() {
        let input = r#""/cosmos.auth.v1beta1.Query/Account""#;
        let (input, link) = string_content(input).unwrap();
        assert!(input.is_empty());
        assert_eq!(link, "/cosmos.auth.v1beta1.Query/Account");
    }

    #[test]
    fn can_find_urls() {
        let input = r#"

        import (
            "fmt"
            "sync"
        
            wasmvmtypes "github.com/CosmWasm/wasmvm/types"
            "github.com/cosmos/cosmos-sdk/codec"
            authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
            banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
            distributiontypes "github.com/cosmos/cosmos-sdk/x/distribution/types"
            slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
            stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
            ibctransfertypes "github.com/cosmos/ibc-go/v7/modules/apps/transfer/types"
        )
        func init() {        
        // auth
        setWhitelistedQuery("/cosmos.auth.v1beta1.Query/Account", &authtypes.QueryAccountResponse{})
        setWhitelistedQuery("/cosmos.auth.v1beta1.Query/Params", &authtypes.QueryParamsResponse{})
        setWhitelistedQuery("/cosmos.auth.v1beta1.Query/ModuleAccounts", &authtypes.QueryModuleAccountsResponse{})
    }

        "#;

        let queries = WhitelistedQueries::from_text(input);
        assert_eq!(
            queries,
            WhitelistedQueries {
                whitelist_urls: vec![
                    "/cosmos.auth.v1beta1.Query/Account",
                    "/cosmos.auth.v1beta1.Query/Params",
                    "/cosmos.auth.v1beta1.Query/ModuleAccounts"
                ]
            }
        )
    }
}

// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

// This is required because a diesel macro makes clippy sad
#![allow(clippy::extra_unused_lifetimes)]

use crate::util::{hash_str, truncate_str};
use anyhow::{Context, Result};
use aptos_api_types::deserialize_from_string;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter};

/**
 * This file defines deserialized move types as defined in our 0x3 contracts.
 */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenDataIdType {
    pub creator: String,
    collection: String,
    name: String,
}

impl TokenDataIdType {
    pub fn to_hash(&self) -> String {
        hash_str(&self.to_string())
    }

    pub fn get_collection_trunc(&self) -> String {
        truncate_str(&self.collection, 128)
    }

    pub fn get_name_trunc(&self) -> String {
        truncate_str(&self.name, 128)
    }

    pub fn get_collection_data_id_hash(&self) -> String {
        CollectionDataIdType {
            creator: self.creator.clone(),
            name: self.name.clone(),
        }
        .to_hash()
    }
}

impl fmt::Display for TokenDataIdType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}::{}::{}", self.creator, self.collection, self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionDataIdType {
    pub creator: String,
    pub name: String,
}

impl CollectionDataIdType {
    pub fn new(creator: String, name: String) -> Self {
        Self { creator, name }
    }
    pub fn to_hash(&self) -> String {
        hash_str(&self.to_string())
    }

    pub fn get_name_trunc(&self) -> String {
        truncate_str(&self.name, 128)
    }
}

impl fmt::Display for CollectionDataIdType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}::{}", self.creator, self.name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenIdType {
    pub token_data_id: TokenDataIdType,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub property_version: BigDecimal,
}

impl fmt::Display for TokenIdType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}::{}", self.token_data_id, self.property_version)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenDataType {
    // TODO: decode bcs
    pub default_properties: serde_json::Value,
    pub description: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub largest_property_version: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub maximum: BigDecimal,
    pub mutability_config: TokenDataMutabilityConfigType,
    name: String,
    pub royalty: RoyaltyType,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub supply: BigDecimal,
    uri: String,
}

impl TokenDataType {
    pub fn get_uri_trunc(&self) -> String {
        truncate_str(&self.uri, 512)
    }

    pub fn get_name_trunc(&self) -> String {
        truncate_str(&self.name, 128)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenDataMutabilityConfigType {
    pub description: bool,
    pub maximum: bool,
    pub properties: bool,
    pub royalty: bool,
    pub uri: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoyaltyType {
    pub payee_address: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub royalty_points_denominator: BigDecimal,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub royalty_points_numerator: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub id: TokenIdType,
    // TODO: decode bcs
    pub token_properties: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionDataType {
    pub description: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub maximum: BigDecimal,
    pub mutability_config: CollectionDataMutabilityConfigType,
    name: String,
    #[serde(deserialize_with = "deserialize_from_string")]
    pub supply: BigDecimal,
    uri: String,
}

impl CollectionDataType {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uri_trunc(&self) -> String {
        truncate_str(&self.uri, 512)
    }

    pub fn get_name_trunc(&self) -> String {
        truncate_str(&self.name, 128)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenOfferIdType {
    pub to_addr: String,
    pub token_id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCoinSwapType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub min_price_per_token: BigDecimal,
    pub token_amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenEscrowType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub locked_until_secs: BigDecimal,
    pub token: TokenType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionResourceType {
    pub collection_data: Table,
    pub token_data: Table,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenStoreResourceType {
    pub tokens: Table,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PendingClaimsResourceType {
    pub pending_claims: Table,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionDataMutabilityConfigType {
    pub description: bool,
    pub maximum: bool,
    pub uri: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WithdrawTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepositTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MintTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub id: TokenDataIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BurnTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MutateTokenPropertyMapEventType {
    pub old_id: TokenIdType,
    pub new_id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfferTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub to_address: String,
    pub token_id: TokenIdType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelTokenOfferEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub to_address: String,
    pub token_id: TokenIdType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaimTokenEventType {
    #[serde(deserialize_with = "deserialize_from_string")]
    pub amount: BigDecimal,
    pub to_address: String,
    pub token_id: TokenIdType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypeInfo {
    pub account_address: String,
    // TODO: decode hexstring
    pub module_name: String,
    // TODO: decode hexstring
    pub struct_name: String,
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::{}::{}",
            self.account_address, self.module_name, self.struct_name
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TokenWriteSet {
    TokenDataId(TokenDataIdType),
    TokenId(TokenIdType),
    TokenData(TokenDataType),
    Token(TokenType),
    CollectionData(CollectionDataType),
    TokenOfferId(TokenOfferIdType),
}

impl TokenWriteSet {
    pub fn from_table_item_type(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
    ) -> Result<Option<TokenWriteSet>> {
        match data_type {
            "0x3::token::TokenDataId" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenWriteSet::TokenDataId(inner))),
            "0x3::token::TokenId" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenWriteSet::TokenId(inner))),
            "0x3::token::TokenData" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenWriteSet::TokenData(inner))),
            "0x3::token::Token" => {
                serde_json::from_value(data.clone()).map(|inner| Some(TokenWriteSet::Token(inner)))
            }
            "0x3::token::CollectionData" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenWriteSet::CollectionData(inner))),
            "0x3::token_transfers::TokenOfferId" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenWriteSet::TokenOfferId(inner))),
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version, data_type, data
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TokenEvent {
    MintTokenEvent(MintTokenEventType),
    BurnTokenEvent(BurnTokenEventType),
    MutateTokenPropertyMapEvent(MutateTokenPropertyMapEventType),
    WithdrawTokenEvent(WithdrawTokenEventType),
    DepositTokenEvent(DepositTokenEventType),
    OfferTokenEvent(OfferTokenEventType),
    CancelTokenOfferEvent(CancelTokenOfferEventType),
    ClaimTokenEvent(ClaimTokenEventType),
}

impl TokenEvent {
    pub fn from_event(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
    ) -> Result<Option<TokenEvent>> {
        match data_type {
            "0x3::token::MintTokenEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::MintTokenEvent(inner))),
            "0x3::token::BurnTokenEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::BurnTokenEvent(inner))),
            "0x3::token::MutateTokenPropertyMapEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::MutateTokenPropertyMapEvent(inner))),
            "0x3::token::WithdrawEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::WithdrawTokenEvent(inner))),
            "0x3::token::DepositEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::DepositTokenEvent(inner))),
            "0x3::token_transfers::TokenOfferEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::OfferTokenEvent(inner))),
            "0x3::token_transfers::TokenCancelOfferEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::CancelTokenOfferEvent(inner))),
            "0x3::token_transfers::TokenClaimEvent" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenEvent::ClaimTokenEvent(inner))),
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version, data_type, data
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TokenResource {
    CollectionResource(CollectionResourceType),
    TokenStoreResource(TokenStoreResourceType),
    PendingClaimsResource(PendingClaimsResourceType),
}

impl TokenResource {
    pub fn is_resource_supported(data_type: &str) -> bool {
        matches!(
            data_type,
            "0x3::token::Collections"
                | "0x3::token::TokenStore"
                | "0x3::token_transfers::PendingClaims"
        )
    }

    pub fn from_resource(
        data_type: &str,
        data: &serde_json::Value,
        txn_version: i64,
    ) -> Result<TokenResource> {
        match data_type {
            "0x3::token::Collections" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenResource::CollectionResource(inner))),
            "0x3::token::TokenStore" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenResource::TokenStoreResource(inner))),
            "0x3::token_transfers::PendingClaims" => serde_json::from_value(data.clone())
                .map(|inner| Some(TokenResource::PendingClaimsResource(inner))),
            _ => Ok(None),
        }
        .context(format!(
            "version {} failed! failed to parse type {}, data {:?}",
            txn_version, data_type, data
        ))?
        .context(format!(
            "Resource unsupported! Call is_resource_supported first. version {} type {}",
            txn_version, data_type
        ))
    }
}

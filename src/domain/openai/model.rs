#![allow(dead_code)]
enum ModelType {
    Babbage,
    Ada,
    Davinci,
    BabbageCodeSearchCode,
    TextSimilarityBabbage001,
    TextDavinci001,
    CurieInstructBeta,
    BabbageCodeSearchText,
    BabbageSimilarity,
    CurieSearchQuery,
    CodeSearchBabbageText001,
    TextDavinci003,
    CodeCushman001,
    CodeSearchBabbageCode001,
    TextAda001,
    TextSimilarityAda001,
    TextDavinciInsert002,
    AdaCodeSearchCode,
    AdaSimilarity,
    CodeSearchAdaText001,
    TextSearchAdaQuery001,
    TextCurie001,
    TextDavinciEdit001,
    DavinciSearchDocument,
    TextDavinci002,
    AdaCodeSearchText,
    TextSearchAdaDoc001,
    CodeDavinciEdit001,
    DavinciInstructBeta,
    TextBabbage001,
    TextSimilarityCurie001,
    CodeSearchAdaCode001,
    AdaSearchQuery,
    TextSearchDavinciQuery001,
    CodeDavinci002,
    CurieSimilarity,
    DavinciSearchQuery,
    TextDavinciInsert001,
    BabbageSearchDocument,
    AdaSearchDocument,
    Curie,
    TextSearchBabbageDoc001,
    TextSearchCurieDoc001,
    TextSearchCurieQuery001,
    BabbageSearchQuery,
    TextSearchDavinciDoc001,
    TextSearchBabbageQuery001,
    CurieSearchDocument,
    TextSimilarityDavinci001,
    AudioTranscribe001,
    DavinciSimilarity,
    Cushman2020_05_03,
    Ada2020_05_03,
    Babbage2020_05_03,
    Curie2020_05_03,
    Davinci2020_05_03,
    IfDavinciV2,
    IfCurieV2,
    IfDavinci3_0_0,
    DavinciIf3_0_0,
    DavinciInstructBeta2_0_0,
}

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub object: String,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: String,
    pub object: String,
    pub created: i64,
    #[serde(rename = "owned_by")]
    pub owned_by: String,
    pub permission: Vec<Permission>,
    pub root: String,
    pub parent: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
    pub id: String,
    pub object: String,
    pub created: i64,
    #[serde(rename = "allow_create_engine")]
    pub allow_create_engine: bool,
    #[serde(rename = "allow_sampling")]
    pub allow_sampling: bool,
    #[serde(rename = "allow_logprobs")]
    pub allow_logprobs: bool,
    #[serde(rename = "allow_search_indices")]
    pub allow_search_indices: bool,
    #[serde(rename = "allow_view")]
    pub allow_view: bool,
    #[serde(rename = "allow_fine_tuning")]
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Value,
    #[serde(rename = "is_blocking")]
    pub is_blocking: bool,
}

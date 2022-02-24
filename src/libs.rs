// #[macro_use]
// extern crate lazy_static;

// use std::str;
// use std::{io::prelude::*, borrow::Cow};
// use std::fs::File;
// use test::Bencher;
use regex::Regex;
use serde::*;
use mongodb::bson::{oid::ObjectId};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApiFaq {
    pub id: Option<String>,
    pub question: Option<String>,
    pub answer: Option<String>,
	pub status: Option<i64>,
	pub lang: Option<String>,
    pub ctime: Option<u64>,
    pub mtime: Option<u64>,
	pub multi_languages: Option<Vec<FaqMultiLanguages>>
}

impl From<DBFaq> for ApiFaq {
    fn from(db_item: DBFaq) -> Self {
        ApiFaq{
            id: Some(db_item._id.unwrap_or(ObjectId::new()).to_hex()),
            question: db_item.question,
            answer: db_item.answer,
            status: db_item.status,
            lang: db_item.lang,
            ctime: db_item._ctime,
            mtime: db_item._mtime,
            multi_languages: db_item.multi_languages
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DBFaq {
	// #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	// #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	// #[serde(serialize_with = "serialize_hex_string_as_object_id")]
    pub _id: Option<ObjectId>,
    pub question: Option<String>,
    pub answer: Option<String>,
	pub status: Option<i64>,
	pub lang: Option<String>,
    pub _ctime: Option<u64>,
    pub _mtime: Option<u64>,
	pub multi_languages: Option<Vec<FaqMultiLanguages>>
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FaqMultiLanguages {
    pub question: Option<String>,
    pub answer: Option<String>,
	pub lang: Option<String>
}

impl From<ApiFaq> for DBFaq {
    fn from(api_item: ApiFaq) -> Self {
        // let serialised = serde_json::to_string(&db_item).unwrap();
        // serde_json::from_str(&serialised).unwrap()
        DBFaq{
            _id: Some(ObjectId::parse_str(api_item.id.unwrap().as_str()).unwrap_or(ObjectId::new())),
            question: api_item.question,
            answer: api_item.answer,
            status: api_item.status,
            lang: api_item.lang,
            _ctime: api_item.ctime,
            _mtime: api_item.mtime,
			multi_languages: api_item.multi_languages
        }
    }
}

// fn source() -> String {
//     let mut f = File::open("3.json").unwrap();
//     let mut target = String::new();
//     f.read_to_string(&mut target).unwrap();
//     target
// }

pub fn get_api_faq_list(n: i32) -> Vec<ApiFaq> {
    let mut vec_api_item_list: Vec<ApiFaq> = vec![];
    let mut i = 0;
    while i < n {
        let api_item = ApiFaq{
            id: Some(ObjectId::new().to_hex()),
            ctime: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs()),
            mtime: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs()),
            question: Some("question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,".to_string()),
            answer: Some("answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,".to_string()),
            status: Some(0),
            lang: Some("cn".to_string()),
            multi_languages: Some(vec![])
        };
        vec_api_item_list.push(api_item);
        i+=1;
    };
    vec_api_item_list
}

pub fn get_db_faq_list(n: i32) -> Vec<DBFaq> {
    let mut vec_db_item_list: Vec<DBFaq> = vec![];
    let mut i = 0;
    while i<n {
        let db_item = DBFaq{
            _id: Some(ObjectId::new()),
            _ctime: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs()),
            _mtime: Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(std::time::Duration::from_secs(0)).as_secs()),
            question: Some("question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,question_01,".to_string()),
            answer: Some("answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,answer,".to_string()),
            status: Some(0),
            lang: Some("cn".to_string()),
            multi_languages: Some(vec![])
        };
        vec_db_item_list.push(db_item);
        i+=1;
    }
    vec_db_item_list
}

#[allow(dead_code)]
pub fn replace_u8_unsafe_morecap(input: &str, from: &str, to: &str) -> String {
    let re: Regex = Regex::new(from).unwrap();
    // let input = input.into();
    let len = input.len();
    let mut output:Vec<u8> = Vec::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    let to_bytes = to.as_bytes();
    let input_u8 = input.as_bytes();
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.extend(&input_u8[start_position..m.start()]);
            output.extend(to_bytes);
            start_position = m.end();    // next start position  
        } else {
            is_finding = false;
            output.extend(&input_u8[start_position..]);
        }
    }
    unsafe { String::from_utf8_unchecked(output) }
}

#[allow(dead_code)]
pub fn replace_str_push(input: &str, from: &str, to: &str) -> String {
    let re: Regex = Regex::new(from).unwrap();
    // let input = input.into();
    let len = input.len();
    let mut output = String::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.push_str(&input[start_position..m.start()]);
            output.push_str(to);
            start_position = m.end();    // next start position  
        } else {
            is_finding = false;
            output.push_str(&input[start_position..]);
        }
    }
    output
}

#[allow(dead_code)]
pub fn replace_string_add_unsafe(input: &str, from: &str, to: &str) -> String {
    unsafe {
        let re: Regex = Regex::new(from).unwrap();
        // let input = input.into();
        let len = input.len();
        let mut output:Vec<u8> = Vec::with_capacity(len + len/2);
        let mut is_finding = true;
        let mut start_position = 0;
        let to_bytes = to.as_bytes();
        let input_u8 = input.as_bytes();
        while is_finding {
            let match_result = re.find_at(&input, start_position);
            if let Some(m) = match_result {
                output.extend(&input_u8[start_position..m.start()]);
                output.extend(to_bytes);
                start_position = m.end();    // next start position  
            } else {
                is_finding = false;
                output.extend(&input_u8[start_position..]);
            }
        }
        String::from_utf8_unchecked(output)
    }
}

#[allow(dead_code)]
pub fn replace_oid_to_str_u8_unsafe(input: &str) -> String {
    let re: Regex = Regex::new(r#""_id":\{"\$oid":"#).unwrap();
    let len = input.len();
    let mut output:Vec<u8> = Vec::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    let input_u8 = input.as_bytes();
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.extend(&input_u8[start_position..m.start()]);
            output.extend(b"\"id\":");
            output.extend(&input_u8[m.end()..(m.end()+26)]);
            start_position = m.end()+27;    // next start position  
        } else {
            is_finding = false;
            output.extend(&input_u8[start_position..]);
        }
    }
    unsafe { String::from_utf8_unchecked(output) }
}

#[allow(dead_code)]
pub fn replace_str_to_oid_u8_unsafe(input: &str) -> String {
    let re: Regex = Regex::new(r#""id":"#).unwrap();
    let len = input.len();
    let mut output:Vec<u8> = Vec::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    let input_u8 = input.as_bytes();
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.extend(&input_u8[start_position..m.start()]);
            output.extend(b"\"_id\":{\"$oid\":");
            output.extend(&input_u8[m.end()..(m.end()+26)]);
            output.extend(b"}");
            start_position = m.end()+26;    // next start position  
        } else {
            is_finding = false;
            output.extend(&input_u8[start_position..]);
        }
    }
    unsafe { String::from_utf8_unchecked(output) }
}

pub fn replace_oid_to_str_safe(input: &str) -> String {
    let re: Regex = Regex::new(r#""_id":\{"\$oid":"#).unwrap();
    let len = input.len();
    let mut output = String::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.push_str(&input[start_position..m.start()]);
            output.push_str("\"id\":");
            output.push_str(&input[m.end()..(m.end()+26)]);
            start_position = m.end()+27;    // next start position  
        } else {
            is_finding = false;
            output.push_str(&input[start_position..]);
        }
    }
    output
}

pub fn replace_str_to_oid_safe(input: &str) -> String {
    let re: Regex = Regex::new(r#""id":"#).unwrap();
    let len = input.len();
    let mut output = String::with_capacity(len + len/2);
    let mut is_finding = true;
    let mut start_position = 0;
    while is_finding {
        let match_result = re.find_at(&input, start_position);
        if let Some(m) = match_result {
            output.push_str(&input[start_position..m.start()]);
            output.push_str("\"_id\":{\"$oid\":");
            output.push_str(&input[m.end()..(m.end()+26)]);
            output.push_str("}");
            start_position = m.end()+26;    // next start position  
        } else {
            is_finding = false;
            output.push_str(&input[start_position..]);
        }
    }
    output
}
#![feature(test)]

extern crate test;

#[path = "../src/libs.rs"] // Here
mod libs;
use libs::*;


#[cfg(test)]
mod tests {
    use std::{vec};

    use super::*;
    use test::Bencher;

    const SIZE: i32 = 10000;

    #[bench]
    fn bench_replace(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);
        let s = serde_json::to_string(&vec_api_item_list).unwrap();
        // let s = source();
        b.iter(|| {
            s.as_str().replace("\"id\"", "\"_id\"")
        });
    }

    #[bench]
    fn bench_regex_replace_all(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);

        let s = serde_json::to_string(&vec_api_item_list).unwrap();
        // let s = source();
        b.iter( || {
            regex::Regex::new("\"id\"").unwrap().replace_all(s.as_str(), "\"_id\"")
        });
    }

    #[bench]
    fn bench_replace_u8_unsafe_morecap(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);

        let s = serde_json::to_string(&vec_api_item_list).unwrap();
        let from = "\"id\"";
        let to = "\"_id\"";
        // let s = source();
        b.iter(|| {
            replace_u8_unsafe_morecap(s.as_str(), from, to)
        })
    }

    #[bench]
    fn bench_replace_string_add(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);

        let s = serde_json::to_string(&vec_api_item_list).unwrap();
        let from = "\"id\"";
        let to = "\"_id\"";
        // let s = source();
        b.iter(|| {
            replace_string_add(s.as_str(), from, to)
        })
    }

    #[bench]
    fn bench_api2db_struct(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);

        b.iter(move || {
            let mut vec_db_item_list: Vec<DBFaq> = vec![];
            for api_item in &vec_api_item_list {
                vec_db_item_list.push(DBFaq::from(api_item.to_owned()));
            }
            vec_db_item_list
        });
    }

    #[bench]
    fn bench_db2api_struct(b: &mut Bencher) {
        let vec_db_item_list = get_db_faq_list(SIZE);

        b.iter(|| {
            let mut vec_api_item_list: Vec<ApiFaq> = vec![];
            for db_item in &vec_db_item_list {
                vec_api_item_list.push(ApiFaq::from(db_item.to_owned()));
            }
            vec_api_item_list
        });
    }

    #[bench]
    fn bench_replace_oid2str(b: &mut Bencher) {
        let vec_db_item_list = get_db_faq_list(SIZE);
        let s = serde_json::to_string(&vec_db_item_list).unwrap();

        b.iter(|| {
            replace_oid_to_str(&s)
        })
    }

    #[bench]
    fn bench_replace_str2oid(b: &mut Bencher) {
        let vec_api_item_list = get_api_faq_list(SIZE);
        let s = serde_json::to_string(&vec_api_item_list).unwrap();

        b.iter(|| {
            replace_str_to_oid(&s)
        })
    }
}

// 390bytes for record

// running 6 tests 10,000
// test tests::bench_api2db_struct     ... bench:  10,712,660 ns/iter (+/- 4,647,473)
// test tests::bench_db2api_struct     ... bench:   7,925,555 ns/iter (+/- 2,371,690)
// test tests::bench_regex_find        ... bench:   1,689,090 ns/iter (+/- 572,376) +
// test tests::bench_regex_replace_all ... bench:   3,213,830 ns/iter (+/- 1,410,058)
// test tests::bench_replace           ... bench:   3,961,010 ns/iter (+/- 1,372,992)
// test tests::bench_replace_oid2str   ... bench:   1,756,082 ns/iter (+/- 589,547) +


// running 6 tests 1000
// test tests::bench_api2db_struct     ... bench:     722,383 ns/iter (+/- 509,383)
// test tests::bench_db2api_struct     ... bench:     546,863 ns/iter (+/- 170,388)
// test tests::bench_regex_find        ... bench:      84,799 ns/iter (+/- 48,130)  +
// test tests::bench_regex_replace_all ... bench:     495,847 ns/iter (+/- 51,928)
// test tests::bench_replace           ... bench:     119,937 ns/iter (+/- 12,861)  +
// test tests::bench_replace_oid2str   ... bench:     128,968 ns/iter (+/- 25,973)  +

// running 7 tests
// test tests::bench_api2db_struct     ... bench:     727,280 ns/iter (+/- 200,810)
// test tests::bench_db2api_struct     ... bench:     787,972 ns/iter (+/- 758,646)
// test tests::bench_regex_find        ... bench:      84,739 ns/iter (+/- 21,771)
// test tests::bench_regex_replace_all ... bench:     543,107 ns/iter (+/- 134,058)
// test tests::bench_replace           ... bench:     122,186 ns/iter (+/- 31,145)
// test tests::bench_replace_oid2str   ... bench:     128,020 ns/iter (+/- 116,967)
// test tests::bench_replace_str2oid   ... bench:      87,072 ns/iter (+/- 27,014)


// running 6 tests 100
// test tests::bench_api2db_struct     ... bench:      74,038 ns/iter (+/- 23,878)
// test tests::bench_db2api_struct     ... bench:      58,057 ns/iter (+/- 11,711)
// test tests::bench_regex_find        ... bench:      28,467 ns/iter (+/- 10,072)
// test tests::bench_regex_replace_all ... bench:      30,397 ns/iter (+/- 5,382)
// test tests::bench_replace           ... bench:      14,114 ns/iter (+/- 7,291)   +
// test tests::bench_replace_oid2str   ... bench:      46,954 ns/iter (+/- 8,722)


// running 6 tests 10
// test tests::bench_api2db_struct     ... bench:      14,747 ns/iter (+/- 8,721)
// test tests::bench_db2api_struct     ... bench:       5,193 ns/iter (+/- 963)
// test tests::bench_regex_find        ... bench:      22,731 ns/iter (+/- 3,743)
// test tests::bench_regex_replace_all ... bench:      23,070 ns/iter (+/- 5,380)
// test tests::bench_replace           ... bench:       1,952 ns/iter (+/- 671)     +
// test tests::bench_replace_oid2str   ... bench:      40,990 ns/iter (+/- 6,551)
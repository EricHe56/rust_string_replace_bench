mod libs;

use libs::*;

fn main() {

    let vec_api_item_list = get_api_faq_list(3);
    let s = serde_json::to_string(&vec_api_item_list).unwrap();
    // println!("{}", &s);

    let t5 = replace_str_to_oid(&s);
    println!("{}", t5);

    // let s = "abc\"id\", \"id\", xxx";
    // let t1 = s.replace("\"id\"", "\"_id\"");
    // println!("{}", t1);

    // let re = Regex::new("\"id\"").unwrap();
    // let t2 = re.replace_all(&s, "\"_id\"");
    // println!("{}",t2);

    // let from = "\"id\"";
    // let to = "\"_id\"";
    // let t3 = replace_u8_unsafe_morecap(&s, from, to);
    // println!("{}", t3);

    let vec_db_faq_list = get_db_faq_list(3);
    let s = serde_json::to_string(&vec_db_faq_list).unwrap();
    // println!("{}", &s);

    let t4 = replace_oid_to_str(&s);
    println!("{}", t4);
}
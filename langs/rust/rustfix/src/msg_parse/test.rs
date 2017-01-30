use msg_parse::{ParsingInfo, errors, ParsingState};
use test_diff;



fn add_chars(pi: &mut ParsingInfo, s: &'static str) {
    for ch in s.chars() {
        pi.add_char(if ch == '^' { 1u8 as char } else { ch });
    }
}

#[test]
fn init_add_char() {
    let mut parsing = ParsingInfo::new();
    parsing.add_char('1');

    let check = ParsingInfo {
        orig_msg: "1".to_string(),
        msg_length: 1,
        reading_tag: "1".to_string(),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


#[test]
fn init_add_chars() {
    let mut parsing = ParsingInfo::new();
    parsing.add_char('1');
    parsing.add_char('2');
    parsing.add_char('3');
    parsing.add_char('4');
    parsing.add_char('5');

    let check = ParsingInfo {
        orig_msg: "12345".to_string(),
        msg_length: 5,
        reading_tag: "12345".to_string(),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}

#[test]
fn invalid_first_char() {
    let mut parsing = ParsingInfo::new();
    parsing.add_char('a');

    let check = ParsingInfo {
        orig_msg: "a".to_string(),
        msg_length: 1,
        reading_tag: "a".to_string(),
        current_field_error: Some((1, errors::TAG_INVALID_CHAR)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}

#[test]
fn invalid_second_char() {
    let mut parsing = ParsingInfo::new();
    parsing.add_char('1');
    parsing.add_char('a');

    let check = ParsingInfo {
        orig_msg: "1a".to_string(),
        msg_length: 2,
        reading_tag: "1a".to_string(),
        current_field_error: Some((2, errors::TAG_INVALID_CHAR)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


#[test]
fn invalid_chars_2errors() {
    let mut parsing = ParsingInfo::new();
    parsing.add_char('1');
    parsing.add_char('a');
    parsing.add_char('b');

    let check = ParsingInfo {
        orig_msg: "1ab".to_string(),
        msg_length: 3,
        reading_tag: "1a".to_string(),
        current_field_error: Some((2, errors::TAG_INVALID_CHAR)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}

#[test]
fn invalid_chars_2errors_andvalids() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "12ab34");

    let check = ParsingInfo {
        orig_msg: "12ab34".to_string(),
        msg_length: 6,
        reading_tag: "12a".to_string(),
        current_field_error: Some((3, errors::TAG_INVALID_CHAR)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


#[test]
fn invalid_chars_2errors_and_valids_non_consecutives() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "12a3b45");

    let check = ParsingInfo {
        orig_msg: "12a3b45".to_string(),
        msg_length: 7,
        reading_tag: "12a".to_string(),
        current_field_error: Some((3, errors::TAG_INVALID_CHAR)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}



//  too long
//      error and igonre big tag

#[test]
fn too_long_tag() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "1234567890");

    let check = ParsingInfo {
        orig_msg: "1234567890".to_string(),
        msg_length: 10,
        reading_tag: "1234567890".to_string(),
        current_field_error: Some((10, errors::TAG_TOO_LONG)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


#[test]
fn too_long_tag_ignore_excess() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "123456789012345");

    let check = ParsingInfo {
        orig_msg: "123456789012345".to_string(),
        msg_length: 15,
        reading_tag: "1234567890".to_string(),
        current_field_error: Some((10, errors::TAG_TOO_LONG)),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}





//  =
//      start receiving val
#[test]
fn finish_tag() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "123456=");

    let check = ParsingInfo {
        orig_msg: "123456=".to_string(),
        msg_length: 7,
        reading_tag: "123456".to_string(),
        state: ParsingState::StReadingValue,
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}




//  receiving val
//      "a"
#[test]
fn reading_val() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "123456=a");

    let check = ParsingInfo {
        orig_msg: "123456=a".to_string(),
        msg_length: 8,
        reading_tag: "123456".to_string(),
        reading_val: "a".to_string(),
        state: ParsingState::StReadingValue,
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


//      "abcdefg"
#[test]
fn reading_val2() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "123456=abcdefg");

    let check = ParsingInfo {
        orig_msg: "123456=abcdefg".to_string(),
        msg_length: 14,
        reading_tag: "123456".to_string(),
        reading_val: "abcdefg".to_string(),
        state: ParsingState::StReadingValue,
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


//      too long val received
#[test]
fn too_long_val() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing,
              "123456=abcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefg");

    let check = ParsingInfo {
        orig_msg: "123456=abcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefg"
            .to_string(),
        msg_length: 77,
        reading_tag: "123456".to_string(),
        reading_val: "abcdefgabcdefgabcdefgabcdefgabcdefgabcdefgabcdefga".to_string(),
        state: ParsingState::StReadingValue,
        current_field_error: Some((57, "Tag too long")),
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}

//  received field  0x01
//      calculate checksum
//      check field in position (session fields)
//      insert in map
#[test]
fn complete_field() {
    let mut parsing = ParsingInfo::new();
    add_chars(&mut parsing, "123456=abcdefg\u{01}");

    let check = ParsingInfo {
        orig_msg: "123456=abcdefg^".to_string(),
        msg_length: 15,
        reading_tag: "".to_string(),
        reading_val: "".to_string(),
        state: ParsingState::StReadingValue,
        ..Default::default()
    };

    assert_eq_dif!(parsing, check);
}


//  completed field with two errors



//  received field  0x01 ERROR
//      at the beginning of tag
//      reading tag
//      after =


//  detected end of message
//      finished status
//      check message length
//      check original message
//      check checksum
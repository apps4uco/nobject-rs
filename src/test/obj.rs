use crate::tokenizer::{
    parse_obj,
    Token,
};

#[test]
fn parse_simple_comment() {
    let vert = "# whatever this is a comment";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_simple_comment_2() {
    let vert = "#whatever this is a comment";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_line_token_with_comment() {
    let vert = "l # blah blah blah";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Line);
}

#[test]
fn parse_vertex() {
    let vert = "v 0.123 0.234 0.345 1.0";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Vertex);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
    assert_eq!(tokens[4], Token::Float(1.0));
}

#[test]
fn parse_vertex_multiline_test() {
    let vert = "v 0.123 0.234 0.345 1.0\nv 1.123 1.234 1.345 1.0";
    let res = parse_obj(vert);

    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Vertex);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
    assert_eq!(tokens[4], Token::Float(1.0));

    assert_eq!(tokens[5], Token::Vertex);
    assert_eq!(tokens[6], Token::Float(1.123));
    assert_eq!(tokens[7], Token::Float(1.234));
    assert_eq!(tokens[8], Token::Float(1.345));
    assert_eq!(tokens[9], Token::Float(1.0));
}

#[test]
fn parse_vertex_normal() {
    let vert = "vn 0.123 0.234 0.345";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::VertexNormal);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
}

#[test]
fn parse_vertex_texture() {
    let vert = "vt 0.500 1";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::VertexTexture);
    assert_eq!(tokens[1], Token::Float(0.500));
    assert_eq!(tokens[2], Token::Int(1));
}

#[test]
fn parse_face() {
    let vert = "f 1 2 3";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Int(2));
    assert_eq!(tokens[3], Token::Int(3));
}

#[test]
fn parse_face_1() {
    let vert = "f 1/2 2/3 3/4";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Int(2));
    assert_eq!(tokens[4], Token::Int(2));
    assert_eq!(tokens[5], Token::Slash);
    assert_eq!(tokens[6], Token::Int(3));
    assert_eq!(tokens[7], Token::Int(3));
    assert_eq!(tokens[8], Token::Slash);
    assert_eq!(tokens[9], Token::Int(4));
}

#[test]
fn parse_face_2() {
    let vert = "f 1/2/3 2/3/4 3/4/5";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Int(2));
    assert_eq!(tokens[4], Token::Slash);
    assert_eq!(tokens[5], Token::Int(3));
    assert_eq!(tokens[6], Token::Int(2));
    assert_eq!(tokens[7], Token::Slash);
    assert_eq!(tokens[8], Token::Int(3));
    assert_eq!(tokens[9], Token::Slash);
    assert_eq!(tokens[10], Token::Int(4));
    assert_eq!(tokens[11], Token::Int(3));
    assert_eq!(tokens[12], Token::Slash);
    assert_eq!(tokens[13], Token::Int(4));
    assert_eq!(tokens[14], Token::Slash);
    assert_eq!(tokens[15], Token::Int(5));
}

#[test]
fn parse_face_3() {
    let vert = "f 1//2 2//3 3//4";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Slash);
    assert_eq!(tokens[4], Token::Int(2));
    assert_eq!(tokens[5], Token::Int(2));
    assert_eq!(tokens[6], Token::Slash);
    assert_eq!(tokens[7], Token::Slash);
    assert_eq!(tokens[8], Token::Int(3));
    assert_eq!(tokens[9], Token::Int(3));
    assert_eq!(tokens[10], Token::Slash);
    assert_eq!(tokens[11], Token::Slash);
    assert_eq!(tokens[12], Token::Int(4));
}

#[test]
fn parse_line() {
    let vert = "l 1 2 3";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Line);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Int(2));
    assert_eq!(tokens[3], Token::Int(3));
}

#[test]
fn simple_material() {
    let vert = "mtllib some_mtl_file.mtl";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::MaterialLib);
    assert_eq!(tokens[1], Token::String("some_mtl_file.mtl".to_string()));
}

#[test]
fn simple_group() {
    let vert = "g some_group";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::Group);
    assert_eq!(tokens[1], Token::String("some_group".to_string()));
}

#[test]
fn simple_object() {
    let vert = "o some_object";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::Object);
    assert_eq!(tokens[1], Token::String("some_object".to_string()));
}
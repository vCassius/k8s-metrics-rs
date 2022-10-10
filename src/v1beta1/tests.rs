use super::*;

use serde_json as json;

#[test]
fn nano() {
    let usage = json::from_str::<Usage>(r#"{"cpu":"1234567n","memory":"123Mi"}"#).unwrap();
    assert_eq!(usage.cpu, 0.001234567_f64);
    assert_eq!(usage.memory, 123 * 1024 * 1024);
}

#[test]
fn milli() {
    let usage = json::from_str::<Usage>(r#"{"cpu":"1234567m","memory":"8748Ki"}"#).unwrap();
    assert_eq!(usage.cpu, 1234.567_f64);
    assert_eq!(usage.memory, 8748 * 1024);
}

#[test]
fn zero() {
    let usage = json::from_str::<Usage>(r#"{"cpu":"0","memory":"51428Ki"}"#).unwrap();
    assert_eq!(usage.cpu, 0_f64);
    assert_eq!(usage.memory, 51428 * 1024);
}

#[test]
fn invalid_cpu() {
    let err = json::from_str::<Usage>(r#"{"cpu":"123t","memory":"125864Ki"}"#).unwrap_err();
    println!("{err}");
    assert!(err.to_string().contains("123t"));
}

#[test]
fn invalid_memory() {
    let err = json::from_str::<Usage>(r#"{"cpu":"123m","memory":"125864Ai"}"#).unwrap_err();
    println!("{err}");
    assert!(err.to_string().contains("125864Ai"));
}

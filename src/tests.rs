//! Tests are incomplete as the majority of them will require root access

// use futures_lite::future;

use crate::{LoadState, ManagerProxyBlocking, Mode};

#[test]
fn reload_or_restart_unit() {
    let conn = zbus::blocking::Connection::system().unwrap();
    let proxy = ManagerProxyBlocking::new(&conn).unwrap();

    assert!(proxy
        .reload_or_restart_unit("nvidia-powerd.service", Mode::Fail)
        .is_ok());
}

// #[test]
// fn enqueue_unit_job() {
//     let conn = zbus::blocking::Connection::system().unwrap();
//     let proxy = ManagerProxyBlocking::new(&conn).unwrap();

//     let res = proxy.enqueue_unit_job("nvidia-powerd.service", "", Mode::Replace).unwrap();
// }

#[test]
fn list_unit() {
    let conn = zbus::blocking::Connection::system().unwrap();
    let proxy = ManagerProxyBlocking::new(&conn).unwrap();

    proxy.list_units().unwrap();
    proxy.list_units().unwrap();
    assert!(proxy.list_units().is_ok());
    assert!(proxy
        .list_units_by_names(&["nvidia-powerd.service", "asusd.service"])
        .is_ok());
    assert!(proxy
        .list_units_by_patterns(&[LoadState::Loaded], &["nvidia*"])
        .is_ok());
    assert!(proxy.list_units_filtered(&[LoadState::Loaded]).is_ok());
}

#[test]
fn job() {
    let conn = zbus::blocking::Connection::system().unwrap();
    let proxy = ManagerProxyBlocking::new(&conn).unwrap();

    proxy.set_default_target("graphical.target", true).unwrap();
    proxy
        .enable_unit_files(&["nvidia-powerd.service"], true, true)
        .unwrap();
    assert!(proxy.get_job(0).is_ok());
    assert!(proxy
        .list_units_by_names(&["nvidia-powerd.service", "asusd.service"])
        .is_ok());
    assert!(proxy
        .list_units_by_patterns(&[LoadState::Loaded], &["nvidia*"])
        .is_ok());
    assert!(proxy.list_units_filtered(&[LoadState::Loaded]).is_ok());
}

// #[test]
// fn properties_async() {
//     future::block_on(async {
//         let conn = zbus::Connection::system().await.unwrap();
//     });
// }

use amm_abi::types::{AMM_ABI_MAJOR, AmmApi};
use std::path::Path;

use amm_client::link::{ApiTable, ManagerLink};
use amm_core::errors::LinkError;
use amm_core::structs::descriptor::Capability;

use crate::support::{FakeTable, IdleHandler};

#[test]
fn wrong_abi_major_is_abi_mismatch() {
    let api = FakeTable::leak(AmmApi { abi_major: AMM_ABI_MAJOR + 1, ..FakeTable::api() });

    // SAFETY: the table is leaked, so it stays valid for the process
    let result = unsafe { ApiTable::new(api) };

    assert_eq!(result.err(), Some(LinkError::AbiMismatch { found: AMM_ABI_MAJOR + 1 }));
}

#[test]
fn table_smaller_than_this_client_needs_is_abi_mismatch() {
    let base = std::mem::offset_of!(AmmApi, log) as u32;
    let api = FakeTable::leak(AmmApi { size: base, ..FakeTable::api() });

    // SAFETY: the table is leaked, so it stays valid for the process
    let result = unsafe { ApiTable::new(api) };

    assert_eq!(result.err(), Some(LinkError::AbiMismatch { found: AMM_ABI_MAJOR }));
}

#[test]
fn null_table_is_manager_missing() {
    // SAFETY: a null pointer is never read
    let result = unsafe { ApiTable::new(std::ptr::null()) };

    assert_eq!(result.err(), Some(LinkError::ManagerMissing));
}

#[repr(C)]
struct FutureTable {
    api: AmmApi,
    added_later: [usize; 4],
}

#[test]
fn a_client_accepts_a_newer_minor_table_and_links_through_it() {
    let future = Box::leak(Box::new(FutureTable {
        api: AmmApi { size: size_of::<FutureTable>() as u32, abi_minor: 9, ..FakeTable::api() },
        added_later: [usize::MAX; 4],
    }));

    // SAFETY: the table is leaked and its first field is an AmmApi header that declares the whole struct
    let table = unsafe { ApiTable::new(std::ptr::from_ref(&future.api)) }.expect("newer minor accepted");
    let link = ManagerLink::new(table, "id = \"alaydriem-test\"\n", Path::new(FakeTable::CONFIG_DIR), IdleHandler);

    assert!(link.is_ok());
}

#[test]
fn a_client_accepts_a_table_that_declares_only_the_functions_of_its_minor() {
    let base = std::mem::offset_of!(AmmApi, log) + size_of::<usize>();
    let api = FakeTable::leak(AmmApi { size: base as u32, abi_minor: 0, ..FakeTable::api() });

    // SAFETY: the table is leaked and holds a whole AmmApi, more than its declared size
    let result = unsafe { ApiTable::new(api) };

    assert!(result.is_ok());
}

#[test]
fn a_client_refuses_another_major() {
    for major in [AMM_ABI_MAJOR - 1, AMM_ABI_MAJOR + 1] {
        let api = FakeTable::leak(AmmApi { abi_major: major, abi_minor: 0, ..FakeTable::api() });

        // SAFETY: the table is leaked, so it stays valid for the process
        let result = unsafe { ApiTable::new(api) };

        assert_eq!(result.err(), Some(LinkError::AbiMismatch { found: major }));
    }
}

// The end of the last function of minor version 1, where a 1.1 manager's table ends.
const V1_1_SIZE: usize = std::mem::offset_of!(AmmApi, log) + size_of::<usize>();

// A 1.1 manager's table: the header and the eleven functions of 1.1, and nothing after them.
#[repr(C, align(8))]
struct TableV1_1([u8; V1_1_SIZE]);

// A link registered through a 1.1 manager's table, or `None` when the table or the registration is refused.
fn one_point_one_link() -> Option<ManagerLink> {
    let full = FakeTable::api();
    let mut bytes = [0u8; V1_1_SIZE];
    // SAFETY: full is a whole AmmApi, which holds more than V1_1_SIZE readable bytes, and bytes does not overlap it
    unsafe { std::ptr::copy_nonoverlapping(std::ptr::from_ref(&full).cast::<u8>(), bytes.as_mut_ptr(), V1_1_SIZE) };
    bytes[..4].copy_from_slice(&(V1_1_SIZE as u32).to_le_bytes());
    bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
    let prefix: &'static TableV1_1 = Box::leak(Box::new(TableV1_1(bytes)));

    // SAFETY: the prefix is leaked, aligned as an AmmApi, and holds the V1_1_SIZE bytes its header declares
    let table = unsafe { ApiTable::new(std::ptr::from_ref(prefix).cast::<AmmApi>()) }.ok()?;
    ManagerLink::without_frame(table, "id = \"alaydriem-test\"\n", Path::new(FakeTable::CONFIG_DIR)).ok()
}

#[test]
fn a_one_point_one_table_makes_every_function_of_one_point_two_unsupported() {
    let link = one_point_one_link().expect("a 1.1 table registers");
    FakeTable::set_screenshot_answer(amm_abi::types::AmmStatus::Ok, 5);
    FakeTable::set_held_by_other(Capability::Camera, true);

    let unsupported = Some(amm_core::errors::LinkError::Status(amm_abi::types::AmmStatus::Unsupported as i32));
    assert_eq!(link.request_screenshot(Path::new("C:\\shots\\a.png")).err(), unsupported);
    assert!(!link.held_by_other(Capability::Camera));
    assert_eq!(link.texture_budget().err(), unsupported);
    assert!(FakeTable::with(|state| state.screenshot_requests.is_empty()));
}

#[test]
fn screenshots_are_supported_on_the_current_table_and_not_on_a_one_point_one_table() {
    let current = FakeTable::link_without_frame();
    let current_supports = current.supports_screenshots();
    drop(current);

    let old = one_point_one_link().expect("a 1.1 table registers");

    assert!(current_supports);
    assert!(!old.supports_screenshots());
}

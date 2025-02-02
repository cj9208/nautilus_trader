// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::{
    collections::hash_map::DefaultHasher,
    ffi::{c_char, CStr},
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    sync::Arc,
};

use nautilus_core::{correctness, string::str_to_cstr};
use pyo3::prelude::*;

#[repr(C)]
#[derive(Clone, Hash, PartialEq, Eq)]
#[pyclass]
pub struct OrderListId {
    pub value: Box<Arc<String>>,
}

impl Debug for OrderListId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Display for OrderListId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl OrderListId {
    #[must_use]
    pub fn new(s: &str) -> Self {
        correctness::valid_string(s, "`OrderListId` value");

        Self {
            value: Box::new(Arc::new(s.to_string())),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// C API
////////////////////////////////////////////////////////////////////////////////
/// Returns a Nautilus identifier from a C string pointer.
///
/// # Safety
///
/// - Assumes `ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn order_list_id_new(ptr: *const c_char) -> OrderListId {
    OrderListId::new(CStr::from_ptr(ptr).to_str().expect("CStr::from_ptr failed"))
}

#[no_mangle]
pub extern "C" fn order_list_id_clone(order_list_id: &OrderListId) -> OrderListId {
    order_list_id.clone()
}

/// Frees the memory for the given `order_list_id` by dropping.
#[no_mangle]
pub extern "C" fn order_list_id_drop(order_list_id: OrderListId) {
    drop(order_list_id); // Memory freed here
}

/// Returns an [`OrderListId`] as a C string pointer.
#[no_mangle]
pub extern "C" fn order_list_id_to_cstr(order_list_id: &OrderListId) -> *const c_char {
    str_to_cstr(&order_list_id.value)
}

#[no_mangle]
pub extern "C" fn order_list_id_eq(lhs: &OrderListId, rhs: &OrderListId) -> u8 {
    u8::from(lhs == rhs)
}

#[no_mangle]
pub extern "C" fn order_list_id_hash(order_list_id: &OrderListId) -> u64 {
    let mut h = DefaultHasher::new();
    order_list_id.hash(&mut h);
    h.finish()
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::OrderListId;
    use crate::identifiers::order_list_id::order_list_id_drop;

    #[test]
    fn test_equality() {
        let id1 = OrderListId::new("001");
        let id2 = OrderListId::new("002");
        assert_eq!(id1, id1);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_string_reprs() {
        let id = OrderListId::new("001");
        assert_eq!(id.to_string(), "001");
        assert_eq!(format!("{id}"), "001");
    }

    #[test]
    fn test_order_list_id_drop() {
        let id = OrderListId::new("001");

        order_list_id_drop(id); // No panic
    }
}

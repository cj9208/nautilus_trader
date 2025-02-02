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
pub struct ClientOrderId {
    pub value: Box<Arc<String>>,
}

impl Debug for ClientOrderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl Display for ClientOrderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for ClientOrderId {
    fn default() -> Self {
        Self {
            value: Box::new(Arc::new(String::from("O-123456789"))),
        }
    }
}

impl ClientOrderId {
    #[must_use]
    pub fn new(s: &str) -> Self {
        correctness::valid_string(s, "`ClientOrderId` value");

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
pub unsafe extern "C" fn client_order_id_new(ptr: *const c_char) -> ClientOrderId {
    ClientOrderId::new(CStr::from_ptr(ptr).to_str().expect("CStr::from_ptr failed"))
}

#[no_mangle]
pub extern "C" fn client_order_id_clone(client_order_id: &ClientOrderId) -> ClientOrderId {
    client_order_id.clone()
}

/// Frees the memory for the given `client_order_id` by dropping.
#[no_mangle]
pub extern "C" fn client_order_id_drop(client_order_id: ClientOrderId) {
    drop(client_order_id); // Memory freed here
}

/// Returns a [`ClientOrderId`] as a C string pointer.
#[no_mangle]
pub extern "C" fn client_order_id_to_cstr(client_order_id: &ClientOrderId) -> *const c_char {
    str_to_cstr(&client_order_id.value)
}

#[no_mangle]
pub extern "C" fn client_order_id_eq(lhs: &ClientOrderId, rhs: &ClientOrderId) -> u8 {
    u8::from(lhs == rhs)
}

#[no_mangle]
pub extern "C" fn client_order_id_hash(client_order_id: &ClientOrderId) -> u64 {
    let mut h = DefaultHasher::new();
    client_order_id.hash(&mut h);
    h.finish()
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::ClientOrderId;
    use crate::identifiers::client_order_id::client_order_id_drop;

    #[test]
    fn test_equality() {
        let id1 = ClientOrderId::new("O-20200814-102234-001-001-1");
        let id2 = ClientOrderId::new("O-20200814-102234-001-001-2");
        assert_eq!(id1, id1);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_string_reprs() {
        let id = ClientOrderId::new("O-20200814-102234-001-001-1");
        assert_eq!(id.to_string(), "O-20200814-102234-001-001-1");
        assert_eq!(format!("{id}"), "O-20200814-102234-001-001-1");
    }

    #[test]
    fn test_client_order_id_drop() {
        let id = ClientOrderId::new("O-20200814-102234-001-001-1");

        client_order_id_drop(id); // No panic
    }
}

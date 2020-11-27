# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2020 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

from nautilus_trader.core.message cimport Message
from nautilus_trader.execution.client cimport ExecutionClient
from nautilus_trader.execution.engine cimport ExecutionEngine


cdef class LiveExecutionEngine(ExecutionEngine):
    cdef object _loop
    cdef object _queue
    cdef object _task_run
    cdef object _task_shutdown
    cdef bint _is_running

    cpdef object get_event_loop(self)
    cpdef object run_task(self)
    cpdef object shutdown_task(self)
    cpdef int qsize(self) except *
    cdef inline void _handle_message(self, Message message)


cdef class LiveExecutionClient(ExecutionClient):
    cdef object _loop

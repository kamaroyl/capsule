/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

use packets::icmp::v6::{Icmpv6, Icmpv6Packet, Icmpv6Payload, NdpOption};

pub mod options;
pub mod router_advert;
pub mod router_solicit;

/// NDP message payload marker
pub trait NdpPayload: Icmpv6Payload {}

/// Common behaviors shared by NDP packets
/// 
/// NDP packets are also ICMPv6 packets.
pub trait NdpPacket<P: NdpPayload>: Icmpv6Packet<P> {
    /// finds a NDP option in the payload by option type
    fn find_option<O: NdpOption>(&self) -> Option<&mut O> {
        unsafe {
            // options are after the fixed part of the payload
            let mut offset = self.payload_offset() + P::size();
            let mut buffer_left = self.payload_len() - P::size();

            while buffer_left > O::size() {
                let [option_type, length] = *(self.get_mut_item::<[u8; 2]>(offset));

                if option_type == O::option_type() {
                    return Some(&mut (*(self.get_mut_item::<O>(offset))))
                } else if length == 0 {
                    return None    // TODO: should we error?
                } else {
                    let length = (length * 8) as usize;
                    offset += length;
                    buffer_left -= length;
                }
            }

            None
        }
    }
}

impl<P: NdpPayload> NdpPacket<P> for Icmpv6<P> where Icmpv6<P>: Icmpv6Packet<P> {}

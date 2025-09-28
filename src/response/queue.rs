use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueueResposne {
    pub name: String,
    #[serde(default)]
    pub messages: i64,
}

#[cfg(test)]
mod tests {
    use crate::response::queue::QueueResposne;

    #[test]
    fn parse_response() {
        let data = r#"[
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 14354
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.602+08:00",
        "memory": 16368,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-channel-5bf88794bf-mkgj7",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 19850292,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "durable": true,
        "exclusive": false,
        "name": "jx-channel-5bf88794bf-p2rpq",
        "node": "rabbit@JXBS01",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 16,
            "next_seq_id": 16,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 1369
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.022+08:00",
        "memory": 19536,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 16,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 16,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 16,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 16,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-elasticsearch-7569477f9-dq59w",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 392367860,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 15,
            "next_seq_id": 15,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 64520
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:05.941+08:00",
        "memory": 19560,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 15,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 15,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 15,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 15,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-esam-bd985cd5b-glb6p",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 291568103,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2125
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:11.894+08:00",
        "memory": 16352,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-ga-nexus-57c7bd8d96-8mvpb",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2933517,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2124
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.028+08:00",
        "memory": 16352,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-ga-nexus-57c7bd8d96-vkbr9",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2932143,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2117
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.227+08:00",
        "memory": 16376,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-hlht-59cd6465c6-2kwdc",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2931695,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2117
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:05.870+08:00",
        "memory": 16408,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-hlht-59cd6465c6-lc4pw",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2933805,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2117
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.695+08:00",
        "memory": 16392,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-hlht-59cd6465c6-vfrk8",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2933813,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1,
            "next_seq_id": 1,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 20009
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.338+08:00",
        "memory": 19504,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 1,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 1,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 1,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-services-6f9f458fbb-k57fx",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 29623115,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1,
            "next_seq_id": 1,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 20017
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:05.976+08:00",
        "memory": 19456,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 1,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 1,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 1,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-services-6f9f458fbb-z598x",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 29602855,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1,
            "next_seq_id": 1,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 20010
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:05.843+08:00",
        "memory": 19488,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 1,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 1,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 1,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-services-6f9f458fbb-zzhxn",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 29623168,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 13,
            "next_seq_id": 13,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 25192
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.757+08:00",
        "memory": 19536,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 13,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 13,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 13,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 13,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-stream-849df567d9-drnpq",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 133702614,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 13,
            "next_seq_id": 13,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 15128
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.602+08:00",
        "memory": 19544,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 13,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 13,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 13,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 13,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-task-57f8488c46-p66ds",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 210085325,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 13,
            "next_seq_id": 13,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 26067
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.240+08:00",
        "memory": 19520,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 13,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 13,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 13,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 13,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-timeseries-8675d455c9-6ckkh",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 233305170,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2121
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.189+08:00",
        "memory": 16368,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-transmitter-55fd765889-99vlt",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2927988,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2121
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.428+08:00",
        "memory": 16352,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-transmitter-55fd765889-jvx68",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2927988,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2121
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.328+08:00",
        "memory": 16352,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-transmitter-55fd765889-mrjvd",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2928010,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": true,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2121
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.090+08:00",
        "memory": 16368,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "jx-transmitter-55fd765889-rvckz",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2928014,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 65,
            "next_seq_id": 65,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 946
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.190+08:00",
        "memory": 19768,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 80,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 80,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 80,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 65,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_common",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 683749775,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 1.5838032191483826,
            "avg_ack_ingress_rate": 1.5838032191483826,
            "avg_egress_rate": 1.5838032191483826,
            "avg_ingress_rate": 1.5838032191483826,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 64662263,
            "next_seq_id": 64662263,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 4,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2703
        },
        "head_message_timestamp": null,
        "memory": 1815176,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 65214258,
            "ack_details": {
                "rate": 0.8
            },
            "deliver": 65214258,
            "deliver_details": {
                "rate": 0.8
            },
            "deliver_get": 65214258,
            "deliver_get_details": {
                "rate": 0.8
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 66586133,
            "publish_details": {
                "rate": 1.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_equipment",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 111779999461,
        "reductions_details": {
            "rate": 2708.6
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 4.916586212691607,
            "avg_ack_ingress_rate": 4.916563386456376,
            "avg_egress_rate": 4.916563386456376,
            "avg_ingress_rate": 4.916563386456376,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 159836486,
            "next_seq_id": 159836486,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 5,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 3978
        },
        "head_message_timestamp": null,
        "memory": 1123976,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 160405525,
            "ack_details": {
                "rate": 4.0
            },
            "deliver": 160405525,
            "deliver_details": {
                "rate": 4.0
            },
            "deliver_get": 160405525,
            "deliver_get_details": {
                "rate": 4.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 161825992,
            "publish_details": {
                "rate": 4.4
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_event",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 249614428512,
        "reductions_details": {
            "rate": 7917.2
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 488485,
            "next_seq_id": 488485,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 5734
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.467+08:00",
        "memory": 20344,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 488637,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 488637,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 488637,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 491127,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_firmware_effect",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1482082929,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 684
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.030+08:00",
        "memory": 17968,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 3,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 3,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 3,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_operator",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 643276707,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.42245546959607816,
            "avg_ack_ingress_rate": 0.42245546959607816,
            "avg_egress_rate": 0.42245546959607816,
            "avg_ingress_rate": 0.42245546959607816,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 36287473,
            "next_seq_id": 36287473,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 476
        },
        "head_message_timestamp": null,
        "memory": 697904,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 37264630,
            "ack_details": {
                "rate": 0.8
            },
            "deliver": 37264631,
            "deliver_details": {
                "rate": 0.8
            },
            "deliver_get": 37264631,
            "deliver_get_details": {
                "rate": 0.8
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 42084867,
            "publish_details": {
                "rate": 0.8
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_order",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 67450894272,
        "reductions_details": {
            "rate": 559.8
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 314792,
            "next_seq_id": 314792,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 462
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:05.853+08:00",
        "memory": 25576,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 318323,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 318323,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 318323,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 314792,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_station",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1303997213,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.08018080322563956,
            "avg_ack_ingress_rate": 0.08018080322563956,
            "avg_egress_rate": 0.08018080322563956,
            "avg_ingress_rate": 0.08018080322563956,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 300291,
            "next_seq_id": 300291,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 178
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:08.617+08:00",
        "memory": 24144,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 324442,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 324442,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 324442,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 396673,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_elasticsearch_variables",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1695490641,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 47234
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.114+08:00",
        "memory": 15096,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_baic_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 405401340,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 31451
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.898+08:00",
        "memory": 15104,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_baidu_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 473052190,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 6379
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.010+08:00",
        "memory": 15112,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_bmcd_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 789741829,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1952,
            "next_seq_id": 1952,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 1347
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.440+08:00",
        "memory": 18176,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1952,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 1952,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 1952,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1371396887,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 39358
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.824+08:00",
        "memory": 15168,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_ga_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 581218192,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 34341
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.922+08:00",
        "memory": 15104,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_gacne_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 476014469,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 47207
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.858+08:00",
        "memory": 15096,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_gwm_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 406417262,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 32317
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.472+08:00",
        "memory": 15104,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_hztzxny_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 386847767,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 18579
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.859+08:00",
        "memory": 15096,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_jk_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 631710765,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 593,
            "next_seq_id": 593,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 33320
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.072+08:00",
        "memory": 18704,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 593,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 593,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 593,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_kd_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 879478608,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 3440
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.775+08:00",
        "memory": 15096,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_kh_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 608884817,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 11390
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.209+08:00",
        "memory": 15168,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_ndt_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 623818426,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 7221
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.601+08:00",
        "memory": 15720,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_pingan_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 272669951,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 51565
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.705+08:00",
        "memory": 15112,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_shll_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1136872712,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 6303
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.495+08:00",
        "memory": 15248,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_szaj_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 718683079,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 46655
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.190+08:00",
        "memory": 15112,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_szdb_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 849893420,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 22192
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.852+08:00",
        "memory": 15248,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_szec_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1108743474,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 5371654,
            "next_seq_id": 5371654,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 29432
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.804+08:00",
        "memory": 18808,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 5371654,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 5371654,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 5371654,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 5371646,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_tp",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 15611093904,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 27915
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.371+08:00",
        "memory": 15168,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_wl_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 565256503,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 56712
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.486+08:00",
        "memory": 15112,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_wzsz_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 762737650,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 12,
            "next_seq_id": 12,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 7074
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.746+08:00",
        "memory": 18752,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 12,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 12,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 12,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_xdt_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 546189596,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 17091
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.314+08:00",
        "memory": 15104,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_xiaoju_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 547869153,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 28544
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.205+08:00",
        "memory": 15168,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_xp_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 664279687,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 12126
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.035+08:00",
        "memory": 15096,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_ykc_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 366364906,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 18,
            "next_seq_id": 18,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 56729
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.573+08:00",
        "memory": 17976,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 18,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 18,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 18,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_yscx_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 825166500,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 37343
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.384+08:00",
        "memory": 15760,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_zfb_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 751273206,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 14099
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.125+08:00",
        "memory": 15248,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ga_zjax_delay",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1067030207,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 6,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 857
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.109+08:00",
        "memory": 21544,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_hlht_delay",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 734330529,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 103.22479194069423,
            "avg_ack_ingress_rate": 103.0929188230934,
            "avg_egress_rate": 103.0929188230934,
            "avg_ingress_rate": 103.0929188230934,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 3208194090,
            "next_seq_id": 3208194090,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 20,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 216
        },
        "head_message_timestamp": null,
        "memory": 1136784,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 3208194037,
            "ack_details": {
                "rate": 97.2
            },
            "deliver": 3208194038,
            "deliver_details": {
                "rate": 97.2
            },
            "deliver_get": 3208194038,
            "deliver_get_details": {
                "rate": 97.2
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 3208191418,
            "publish_details": {
                "rate": 106.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_hlht_transmitter",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 4373024321139,
        "reductions_details": {
            "rate": 153980.8
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 3784,
            "next_seq_id": 3784,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 8,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 58189
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:12.043+08:00",
        "memory": 22928,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 3784,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 3784,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 3784,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_hlht_transmitter_delay",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 822839915,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 5,
            "next_seq_id": 5,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 29644
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:11.922+08:00",
        "memory": 16608,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 5,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 5,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 5,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 5,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_message_email",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 315623632,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 0,
            "next_seq_id": 0,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 29662
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:11.769+08:00",
        "memory": 16352,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_message_sms",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 312308182,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.012002959947276967,
            "avg_ack_ingress_rate": 0.012002959947276967,
            "avg_egress_rate": 0.012002959947276967,
            "avg_ingress_rate": 0.012002959947276967,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 112176,
            "next_seq_id": 112176,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 35015
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.665+08:00",
        "memory": 21184,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 112176,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 112176,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 112176,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 112176,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_ordered_charging",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1224719754,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {
            "x-queue-type": "classic"
        },
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 324594707,
            "next_seq_id": 324594707,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 6,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 55784
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:57:06.574+08:00",
        "memory": 22288,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 324594707,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 324594708,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 324594708,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 325565517,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_bms",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 188716182593,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 2.427240817070416,
            "avg_ack_ingress_rate": 2.427240817070416,
            "avg_egress_rate": 2.427240817070416,
            "avg_ingress_rate": 2.427240817070416,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 30570277,
            "next_seq_id": 30570277,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 6,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 30
        },
        "head_message_timestamp": null,
        "memory": 201128,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 30570290,
            "ack_details": {
                "rate": 2.6
            },
            "deliver": 30570290,
            "deliver_details": {
                "rate": 2.6
            },
            "deliver_get": 30570290,
            "deliver_get_details": {
                "rate": 2.6
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 30570290,
            "publish_details": {
                "rate": 2.6
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_bms_info",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 17990581059,
        "reductions_details": {
            "rate": 1727.4
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 2.4423818281862797,
            "avg_ack_ingress_rate": 2.442381764503949,
            "avg_egress_rate": 2.442381764503949,
            "avg_ingress_rate": 2.442381764503949,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 30550338,
            "next_seq_id": 30550338,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 6,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 111
        },
        "head_message_timestamp": null,
        "memory": 200432,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 30550344,
            "ack_details": {
                "rate": 3.0
            },
            "deliver": 30550344,
            "deliver_details": {
                "rate": 3.0
            },
            "deliver_get": 30550344,
            "deliver_get_details": {
                "rate": 3.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 30550346,
            "publish_details": {
                "rate": 3.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_bms_limit",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 17981895890,
        "reductions_details": {
            "rate": 1598.2
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 76.01727114587685,
            "avg_ack_ingress_rate": 75.88993121578689,
            "avg_egress_rate": 75.88993121578689,
            "avg_ingress_rate": 75.88993121578689,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1916673567,
            "next_seq_id": 1916673567,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 5,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 666
        },
        "head_message_timestamp": null,
        "memory": 1133488,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1916673616,
            "ack_details": {
                "rate": 76.6
            },
            "deliver": 1916673616,
            "deliver_details": {
                "rate": 76.6
            },
            "deliver_get": 1916673616,
            "deliver_get_details": {
                "rate": 76.6
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 1916673623,
            "publish_details": {
                "rate": 77.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_channel",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 2627965729438,
        "reductions_details": {
            "rate": 102955.6
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 156.25782610043532,
            "avg_ack_ingress_rate": 156.04745342251042,
            "avg_egress_rate": 156.04745342251042,
            "avg_ingress_rate": 156.04533835783988,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 5458194821,
            "next_seq_id": 5458194821,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0.9999999999935568,
        "consumer_utilisation": 0.9999999999935568,
        "consumers": 15,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 111
        },
        "head_message_timestamp": null,
        "memory": 1148040,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 5404662159,
            "ack_details": {
                "rate": 161.0
            },
            "deliver": 5404662166,
            "deliver_details": {
                "rate": 161.2
            },
            "deliver_get": 5404662166,
            "deliver_get_details": {
                "rate": 161.2
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 5403341824,
            "publish_details": {
                "rate": 161.2
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_hlht",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 7605384325425,
        "reductions_details": {
            "rate": 210664.8
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.8171313373225886,
            "avg_ack_ingress_rate": 0.817131337266801,
            "avg_egress_rate": 0.817131337266801,
            "avg_ingress_rate": 0.817131337266801,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 199647481,
            "next_seq_id": 199647481,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 6,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 86
        },
        "head_message_timestamp": null,
        "memory": 147960,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 199647483,
            "ack_details": {
                "rate": 1.4
            },
            "deliver": 199647488,
            "deliver_details": {
                "rate": 1.4
            },
            "deliver_get": 199647488,
            "deliver_get_details": {
                "rate": 1.4
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 231307713,
            "publish_details": {
                "rate": 1.2
            },
            "redeliver": 5,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_metervalues",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 121954548130,
        "reductions_details": {
            "rate": 656.4
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 32.04567362365996,
            "avg_ack_ingress_rate": 31.906238180688224,
            "avg_egress_rate": 31.906238180688224,
            "avg_ingress_rate": 31.906238180688224,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 2834920583,
            "next_seq_id": 2834920583,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 5,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 231
        },
        "head_message_timestamp": null,
        "memory": 1125960,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 2834911991,
            "ack_details": {
                "rate": 32.6
            },
            "deliver": 2834912030,
            "deliver_details": {
                "rate": 32.6
            },
            "deliver_get": 2834912030,
            "deliver_get_details": {
                "rate": 32.6
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 2684738240,
            "publish_details": {
                "rate": 31.6
            },
            "redeliver": 31,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_timeseries",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 3946300230855,
        "reductions_details": {
            "rate": 45560.4
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 31.781554727072937,
            "avg_ack_ingress_rate": 31.669470406698974,
            "avg_egress_rate": 31.669470406698974,
            "avg_ingress_rate": 31.669470406698974,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 2626930046,
            "next_seq_id": 2626930046,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 15,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 102
        },
        "head_message_timestamp": null,
        "memory": 201560,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 2626930213,
            "ack_details": {
                "rate": 33.0
            },
            "deliver": 2626930216,
            "deliver_details": {
                "rate": 33.0
            },
            "deliver_get": 2626930216,
            "deliver_get_details": {
                "rate": 33.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 2869630888,
            "publish_details": {
                "rate": 30.2
            },
            "redeliver": 3,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_services_transaction",
        "node": "rabbit@JXBS01",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1449531527199,
        "reductions_details": {
            "rate": 17049.4
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0020750223066992772,
            "avg_ack_ingress_rate": 0.0020750223066992772,
            "avg_egress_rate": 0.0020750223066992772,
            "avg_ingress_rate": 0.0020750223066992772,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 40539,
            "next_seq_id": 40539,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 4,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 8597
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:11.647+08:00",
        "memory": 30712,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 40955,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 40955,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 40955,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 40955,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_temporary_start",
        "node": "rabbit@JXBS03",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 199357659,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 7736410,
            "next_seq_id": 7736410,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 14528
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.017+08:00",
        "memory": 18368,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 7736410,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 7736410,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 7736410,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 7736410,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_baic",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 13993651929,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 17080985,
            "next_seq_id": 17080985,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 62532
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.321+08:00",
        "memory": 18624,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 17014985,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 17014987,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 17080987,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 66000,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 17080985,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 2,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_baidu",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 32330378639,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 23394486,
            "next_seq_id": 23394486,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 2175
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.475+08:00",
        "memory": 18320,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 23394485,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 23394488,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 23394488,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 23394486,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 3,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_bmcd",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 46748740791,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 17136449,
            "next_seq_id": 17136449,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 62522
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.488+08:00",
        "memory": 18624,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 17136448,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 17136448,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 17136448,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 17136449,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_byd",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 32175725148,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 399,
            "next_seq_id": 399,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 40132
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.919+08:00",
        "memory": 18312,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 399,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 399,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 399,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 399,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_ga",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 627912941,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 40583,
            "next_seq_id": 40583,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 24375
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.532+08:00",
        "memory": 18320,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 40583,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 40583,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 40583,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 40583,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_gacne",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 791880952,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 15255991,
            "next_seq_id": 15255991,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 62299
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.678+08:00",
        "memory": 18320,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 15255991,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 15255992,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 15255992,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 15255991,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_gwm",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 27393574059,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 1526791,
            "next_seq_id": 1526791,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 59060
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.867+08:00",
        "memory": 18624,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 1526791,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 1526791,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 1526791,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 1526791,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_hztzxny",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 4909635150,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 25077333,
            "next_seq_id": 25077333,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 49004
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.892+08:00",
        "memory": 18328,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 25077333,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 25077333,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 25077333,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 25077333,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_jk",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 47603796931,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 1.3582334317160736,
            "avg_ack_ingress_rate": 1.3582334317160736,
            "avg_egress_rate": 1.3582334317160736,
            "avg_ingress_rate": 1.3582334317160736,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 168484358,
            "next_seq_id": 168484358,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 3,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 3759
        },
        "head_message_timestamp": null,
        "memory": 1121376,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 168484361,
            "ack_details": {
                "rate": 1.6
            },
            "deliver": 168484361,
            "deliver_details": {
                "rate": 1.6
            },
            "deliver_get": 168484361,
            "deliver_get_details": {
                "rate": 1.6
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 168484361,
            "publish_details": {
                "rate": 0.6
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_kd",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 258097648910,
        "reductions_details": {
            "rate": 4109.2
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 18752,
            "next_seq_id": 18752,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 16575
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.430+08:00",
        "memory": 18344,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 18752,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 18752,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 18752,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 18752,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_kh",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 833934703,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 64282,
            "next_seq_id": 64282,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 30233
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.187+08:00",
        "memory": 18304,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 64282,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 64282,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 64282,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 64282,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_ndt",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 1071851457,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.17567319802111325,
            "avg_ack_ingress_rate": 0.17567319802095288,
            "avg_egress_rate": 0.17567319802095288,
            "avg_ingress_rate": 0.17567319802095288,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 12789482,
            "next_seq_id": 12789482,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 1,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 1640
        },
        "head_message_timestamp": null,
        "memory": 695208,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 12789482,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 12789482,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 12789482,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 12789483,
            "publish_details": {
                "rate": 0.2
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_pingan",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 22564419417,
        "reductions_details": {
            "rate": 551.4
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 250336201,
            "next_seq_id": 250336201,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 49229
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.684+08:00",
        "memory": 18760,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 250320687,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 250320712,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 250335170,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 944,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 12,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 13514,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 250336201,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 969,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_shll",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 392408440471,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 97202552,
            "next_seq_id": 97202552,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 29542
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.789+08:00",
        "memory": 18760,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 97153327,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 97153373,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 97202598,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 2,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 49225,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 97202552,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 46,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_szaj",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 153284381570,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 66407476,
            "next_seq_id": 66407476,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 32370
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.944+08:00",
        "memory": 18712,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 66407387,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 66407415,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 66407504,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 4,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 89,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 66407476,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 28,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_szdb",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 110827510180,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 447381449,
            "next_seq_id": 447381449,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 32146
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.407+08:00",
        "memory": 18728,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 447381447,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 447381510,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 447408903,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 16000,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 2,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 11393,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 447371697,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 15939,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_szec",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 743850593977,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 24845435,
            "next_seq_id": 24845435,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 15746
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.635+08:00",
        "memory": 25648,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 24845435,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 24845436,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 24845436,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 24845435,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_wl",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 46481083060,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 224507926,
            "next_seq_id": 224507926,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 51924
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.001+08:00",
        "memory": 18744,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 224507926,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 224507926,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 224507926,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 224507926,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_wzsz",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 329128138420,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 1.3526158058119127,
            "avg_ack_ingress_rate": 1.3526158058119127,
            "avg_egress_rate": 1.3526158058119127,
            "avg_ingress_rate": 1.3526158058119127,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 102767754,
            "next_seq_id": 102767754,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 2,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 3289
        },
        "head_message_timestamp": null,
        "memory": 2924136,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 102767756,
            "ack_details": {
                "rate": 1.6
            },
            "deliver": 102767756,
            "deliver_details": {
                "rate": 1.6
            },
            "deliver_get": 102767756,
            "deliver_get_details": {
                "rate": 1.6
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 102767758,
            "publish_details": {
                "rate": 0.8
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_xdt",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 159680406361,
        "reductions_details": {
            "rate": 3577.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 4378851,
            "next_seq_id": 4378851,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 6205
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.482+08:00",
        "memory": 18336,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 4378851,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 4378851,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 4378851,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 4378851,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_xiaoju",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 12237756139,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 26748499,
            "next_seq_id": 26748499,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 24614
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.586+08:00",
        "memory": 25416,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 26748499,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 26748505,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 26748505,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 26748499,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 8,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_xp",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 50548022173,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 17949864,
            "next_seq_id": 17949864,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 12181
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.829+08:00",
        "memory": 26912,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 17949864,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 17949864,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 17949864,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 17949864,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 0,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_ykc",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 32108987704,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 23951292,
            "next_seq_id": 23951292,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 63920
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.384+08:00",
        "memory": 18336,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 23951291,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 23951295,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 23951295,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 23951292,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 4,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_yscx",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 47566331090,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 2.7033714649893845,
            "avg_ack_ingress_rate": 2.7033714649893845,
            "avg_egress_rate": 2.7033714649893845,
            "avg_ingress_rate": 2.7033714649893845,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 118210513,
            "next_seq_id": 118210513,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 1.0,
        "consumer_utilisation": 1.0,
        "consumers": 5,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 629
        },
        "head_message_timestamp": null,
        "memory": 696264,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 118210520,
            "ack_details": {
                "rate": 2.2
            },
            "deliver": 118210534,
            "deliver_details": {
                "rate": 2.2
            },
            "deliver_get": 118210534,
            "deliver_get_details": {
                "rate": 2.2
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 0,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 0,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 118210526,
            "publish_details": {
                "rate": 2.6
            },
            "redeliver": 14,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_zfb",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 190464659840,
        "reductions_details": {
            "rate": 4819.2
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 11606437,
            "next_seq_id": 11606437,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 54935
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:52.902+08:00",
        "memory": 18760,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 11538420,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 11538439,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 11606456,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 2,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 68017,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 11606412,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 19,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_zjax",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 29840236873,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    },
    {
        "arguments": {},
        "auto_delete": false,
        "backing_queue_status": {
            "avg_ack_egress_rate": 0.0,
            "avg_ack_ingress_rate": 0.0,
            "avg_egress_rate": 0.0,
            "avg_ingress_rate": 0.0,
            "delta": [
                "delta",
                "undefined",
                0,
                0,
                "undefined"
            ],
            "len": 0,
            "mode": "default",
            "next_deliver_seq_id": 14878674,
            "next_seq_id": 14878674,
            "q1": 0,
            "q2": 0,
            "q3": 0,
            "q4": 0,
            "target_ram_count": "infinity",
            "version": 1
        },
        "consumer_capacity": 0,
        "consumer_utilisation": 0,
        "consumers": 0,
        "durable": true,
        "effective_policy_definition": {},
        "exclusive": false,
        "exclusive_consumer_tag": null,
        "garbage_collection": {
            "fullsweep_after": 65535,
            "max_heap_size": 0,
            "min_bin_vheap_size": 46422,
            "min_heap_size": 233,
            "minor_gcs": 27255
        },
        "head_message_timestamp": null,
        "idle_since": "2025-09-25T08:56:53.698+08:00",
        "memory": 18472,
        "message_bytes": 0,
        "message_bytes_paged_out": 0,
        "message_bytes_persistent": 0,
        "message_bytes_ram": 0,
        "message_bytes_ready": 0,
        "message_bytes_unacknowledged": 0,
        "message_stats": {
            "ack": 14878647,
            "ack_details": {
                "rate": 0.0
            },
            "deliver": 14878648,
            "deliver_details": {
                "rate": 0.0
            },
            "deliver_get": 14878675,
            "deliver_get_details": {
                "rate": 0.0
            },
            "deliver_no_ack": 0,
            "deliver_no_ack_details": {
                "rate": 0.0
            },
            "get": 0,
            "get_details": {
                "rate": 0.0
            },
            "get_empty": 1,
            "get_empty_details": {
                "rate": 0.0
            },
            "get_no_ack": 27,
            "get_no_ack_details": {
                "rate": 0.0
            },
            "publish": 14878674,
            "publish_details": {
                "rate": 0.0
            },
            "redeliver": 1,
            "redeliver_details": {
                "rate": 0.0
            }
        },
        "messages": 0,
        "messages_details": {
            "rate": 0.0
        },
        "messages_paged_out": 0,
        "messages_persistent": 0,
        "messages_ram": 0,
        "messages_ready": 0,
        "messages_ready_details": {
            "rate": 0.0
        },
        "messages_ready_ram": 0,
        "messages_unacknowledged": 0,
        "messages_unacknowledged_details": {
            "rate": 0.0
        },
        "messages_unacknowledged_ram": 0,
        "name": "mq_tp_zjjx",
        "node": "rabbit@JXBS02",
        "operator_policy": null,
        "policy": null,
        "recoverable_slaves": null,
        "reductions": 30137260452,
        "reductions_details": {
            "rate": 0.0
        },
        "single_active_consumer_tag": null,
        "state": "running",
        "type": "classic",
        "vhost": "/"
    }
]"#;

        let v = serde_json::from_str::<Vec<QueueResposne>>(&data).unwrap();

        v.iter().for_each(|d| println!("{:?}", d));
    }
}

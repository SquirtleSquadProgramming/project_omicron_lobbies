# Protocol specification V0
3 Types of messages:
| Name:  | Create | Modify | Destroy |
| ------ | ------ | ------ | ------- |
| Value: | `0x1`  | `0x2`  | `0x4`   |

Binary, variable length protocol.

## Create:
It contains the:
- Flags:
	- IpV4 / IpV6
	- Private / Public
	- Open / Password
- Host IP Address
- Host Port
- Host Geographical Region
- Max Players
- Lobby Name
- Password

| Type | Version | Flags | IpV(4/6) Address     | Port  | Region | Max Players | Lobby Name    | Password?     |
| ---- | ------- | ----- | -------------------- | ----- | ------ | ----------- | ------------- | ------------- |
| `u4` | `u4`    | `u8`  | `[u8; 4] / [u16; 8]` | `u16` | `u8`   | `u8`        | `u8`, n bytes | `u8`, n bytes |

## Modify:
It consists of `field_type` followed by `field_value`,
where `field_type` is a `u8` of the following enum/int table:

| Value | Name        |
| ----- | ----------- |
| `0`   | `FLAGS`     |
| `1`   | `IPADDR`    |
| `2`   | `PORT`      |
| `3`   | `REGION`    |
| `4`   | `MAX_COUNT` |
| `5`   | `L_NAME`    |
| `6`   | `L_PASS`    |
| `7`   | `PLAYERS`   |

`PLAYERS` is the current number of connected players and a `u8`.

Any amount of *unique* `field_type`s can be set in a request.
| Type | Version | `field_type` | `field_value` | Repeat... |
| ---- | ------- | ------------ | ------------- | --------- |
| `u4` | `u4`    | `u8`         | Dynamic       | Repeat... |

## Destroy:
It contains:
- IpV4 / IpV6
- Host IP Address
- Host Port
- Password

The password field is only needed if there is password protection on the lobby.

| Type | Version | IpV  | IpV(4/6) Address     | Port  | Password?     |
| ---- | ------- | ---- | -------------------- | ----- | ------------- |
| `u4` | `u4`    | `u8` | `[u8; 4] / [u16; 8]` | `u16` | `u8`, n bytes |

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
- Current Number of Players

Effectively the create method but with added current players added.

| Type | Version | Flags | IpV(4/6) Address     | Port  | Region | Max Players | Lobby Name    | Password?     | Current Players |
| ---- | ------- | ----- | -------------------- | ----- | ------ | ----------- | ------------- | ------------- | --------------- |
| `u4` | `u4`    | `u8`  | `[u8; 4] / [u16; 8]` | `u16` | `u8`   | `u8`        | `u8`, n bytes | `u8`, n bytes | `u8`            |

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

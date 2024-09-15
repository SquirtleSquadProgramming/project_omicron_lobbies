# Protocol specification V0
4 Types of messages:
| Name:  | Create | Modify | Destroy | Get   |
| ------ | ------ | ------ | ------- | ----- |
| Value: | `0x1`  | `0x2`  | `0x4`   | `0x8` |

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

## Get:
Returns a paginated lobby list sorted by the given field.

If Filter ≠ Search:
| Type | Version | Filter | Regions | Page Number |
| ---- | ------- | ------ | ------- | ----------- |
| `u4` | `u4`    | `u8`   | `u8`    | `u8`        |

If Filter = Search:
| Type | Version | Filter | Search Name   | Regions | Page Number |
| ---- | ------- | ------ | ------------- | ------- | ----------- |
| `u4` | `u4`    | `255`  | `u8`, n bytes | `u8`    | `u8`        |

| ID    | Filter                  |
| ----- | ----------------------- |
| `0`   | Name Ascending          |
| `1`   | Name Descending         |
| `2`   | Player Count Ascending  |
| `3`   | Player Count Descending |
| `255` | Search                  |

## Server Response Codes:

| Code | Meaning                   |
| ---- | ------------------------- |
| 10   | Success                   |
| 40   | Empty Message             |
| 41   | Invalid Type              |
| 42   | Missing Message Part      |
| 43   | Invalid Region            |
| 44   | Invalid Name              |
| 45   | Mismatched Ip             |
| 46   | Out of Date               |
| 50   | Not Initialised           |
| 51   | Lobby Already Exists      |
| 52   | Lobby Does Not Exist      |
| 53   | Failed to Hash Password   |
| 54   | Failed to Verify Password |
| 55   | Invalid Credentials       |
| 101  | Connection Timed Out (5s) |

# Protocol specification
3 Types of messages: Create / Modify / Destroy lobby
Binary variable length protocol.

## Create:
Variable length protocol.

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

| Version | Flags | Ip Address | Port | Region | Max Players | Lobby Name | Password? |
| ------- | ----- | ---------- | ---- | ------ | ----------- | ---------- | --------- |
| u8 | u8 | [u8; 4] / [u16; 8] |  u16 |   u8   |     u8      | u8, n bytes|  [u8, 60] |

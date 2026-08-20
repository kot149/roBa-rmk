# roBa Raytac dongle

RMK BLE dongle firmware for the Raytac MDBT50Q-RX. The dongle connects to one
bonded roBa keyboard and relays HID reports and Rynk frames over USB.

## Flash layout

The image is linked for the Raytac S140 v6 UF2 layout:

```text
application: 0x00026000 .. 0x000EC000
RMK storage: 0x000EC000 .. 0x000EE000
UF2 bootloader: 0x000F4000 .. 0x00100000
```

The application image must not contain data outside the application range.

## Build

From the repository root:

```shell
cargo build --manifest-path dongle/Cargo.toml --release
cargo make uf2-dongle
```

Copy `roBa-raytac-dongle.uf2` to the Raytac UF2 bootloader drive.

The dongle opens a pairing window at power-on. A keyboard with no dongle bond
seeks during that window. After pairing, the dongle reconnects only to its
stored keyboard. Hold the keyboard's dongle key for five seconds to clear the
keyboard-side bond and pair with another dongle.

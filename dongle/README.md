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

> **Warning:** Before flashing, open `INFO_UF2.TXT` on the target drive. It must
> contain `Board-ID: nRF52840-MDBT50Q_RX-verD`, `UF2 Bootloader 0.9.2`, and
> `SoftDevice: S140 6.1.1`. Do not flash this image to another nRF52840 UF2
> drive. `cargo make flash-dongle` checks all three lines.

## Build

From the repository root:

```shell
cargo build --manifest-path dongle/Cargo.toml --release
cargo make uf2-dongle
```

Flash with the metadata-checked task:

```shell
cargo make flash-dongle
```

For a manual copy, verify `INFO_UF2.TXT` first and copy only to the matching Raytac drive.

The dongle opens a pairing window at power-on. A keyboard with no dongle bond
seeks during that window. After pairing, the dongle reconnects only to its
stored keyboard. Hold the keyboard's dongle key for five seconds to clear the
keyboard-side bond and pair with another dongle.

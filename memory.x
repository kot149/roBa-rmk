MEMORY
{
  /* NOTE 1 K = 1 KiB = 1024 bytes */

  /* XIAO nRF52840 (with S140 SoftDevice and Adafruit nRF52 bootloader) */
  FLASH : ORIGIN = 0x00027000, LENGTH = 868K
  RAM : ORIGIN = 0x20000008, LENGTH = 255K

  /* Reference: nRF52840 with Adafruit nRF52 bootloader */
  /* FLASH : ORIGIN = 0x00001000, LENGTH = 1020K */
  /* RAM : ORIGIN = 0x20000008, LENGTH = 255K */

  /* Reference: nRF52840 */
  /* FLASH : ORIGIN = 0x00000000, LENGTH = 1024K */
  /* RAM : ORIGIN = 0x20000000, LENGTH = 256K */
}

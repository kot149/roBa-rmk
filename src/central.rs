#![no_std]
#![no_main]

mod led;
mod vial;
#[macro_use]
mod macros;
mod keymap;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Flex, Input, Output};
use embassy_nrf::interrupt::{self, InterruptExt};
use pmw3610_rs::{Pmw3610Config, Pmw3610Device};
use rmk::input_device::joystick::JoystickProcessor;
use rmk_scroll_layer_processor::{ScrollLayerProcessor, ScrollLayerTracker};
use embassy_nrf::mode::Async;
use embassy_nrf::peripherals::{RNG, SAADC, USBD};
use embassy_nrf::saadc::{self, AnyInput, Input as _, Saadc};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{Peri, bind_interrupts, rng, usb};
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::build_ble_stack;
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{
    BehaviorConfig, BleBatteryConfig, DeviceConfig, PositionalConfig, RmkConfig, StorageConfig, VialConfig,
};
use rmk::controller::EventController as _;
use rmk::controller::led_indicator::KeyboardIndicatorController;
use led::BleConnectionLed;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::{join, join4, join5};
use rmk::input_device::Runnable;
use rmk::input_device::adc::{AnalogEventType, NrfAdc};
use rmk::input_device::battery::BatteryProcessor;
use rmk::keyboard::Keyboard;
use rmk::split::ble::central::{read_peripheral_addresses, scan_peripherals};
use rmk::split::central::{CentralMatrix, run_peripheral_manager};
use rmk::{HostResources, initialize_encoder_keymap_and_storage, run_devices, run_processor_chain, run_rmk};
use static_cell::StaticCell;
use vial::{VIAL_KEYBOARD_DEF, VIAL_KEYBOARD_ID};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    SAADC => saadc::InterruptHandler;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

/// How many outgoing L2CAP buffers per link
const L2CAP_TXQ: u8 = 3;

/// How many incoming L2CAP buffers per link
const L2CAP_RXQ: u8 = 3;

/// Size of L2CAP packets
const L2CAP_MTU: usize = 251;

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_scan()?
        .support_central()?
        .support_adv()?
        .support_peripheral()?
        .support_dle_peripheral()?
        .support_dle_central()?
        .support_phy_update_central()?
        .support_phy_update_peripheral()?
        .support_le_2m_phy()?
        .central_count(1)?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU as u16, L2CAP_MTU as u16, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
fn init_adc(adc_pin: AnyInput, adc: Peri<'static, SAADC>) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    interrupt::SAADC.set_priority(interrupt::Priority::P3);

    saadc::Saadc::new(adc, Irqs, config, [channel_cfg])
}

fn ble_addr() -> [u8; 6] {
    let ficr = embassy_nrf::pac::FICR;
    let high = u64::from(ficr.deviceid(1).read());
    let addr = high << 32 | u64::from(ficr.deviceid(0).read());
    let addr = addr | 0x0000_c000_0000_0000;
    unwrap!(addr.to_le_bytes()[..6].try_into())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello RMK BLE!");
    // Initialize the peripherals and nrf-sdc controller
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);
    let mpsl_p = mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };
    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();
    let mpsl = MPSL.init(unwrap!(mpsl::MultiprotocolServiceLayer::with_timeslots(
        mpsl_p,
        Irqs,
        lfclk_cfg,
        SESSION_MEM.init(mpsl::SessionMem::new())
    )));
    spawner.must_spawn(mpsl_task(&*mpsl));
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24, p.PPI_CH25, p.PPI_CH26,
        p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut rng_gen = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<8192>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_gen, &mut host_resources).await;

    // Initialize usb driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Initialize flash
    let flash = Flash::take(mpsl, p.NVMC);

    // Initialize IO Pins
    let (row_pins, col_pins) = config_matrix_pins_nrf!(
        peripherals: p,
        input: [P0_03, P0_28, P0_29, P1_11],
        output:  [P1_15, P1_14, P1_13, P1_12, P0_10]
    );

    // XIAO BLE nRF52840: Enable battery voltage reading by setting P0.14 (VBAT_ENABLE) to LOW.
    // This pin controls the voltage divider circuit for battery measurement on P0.31.
    let vbat_enable_pin = Output::new(p.P0_14, embassy_nrf::gpio::Level::Low, embassy_nrf::gpio::OutputDrive::Standard);
    core::mem::forget(vbat_enable_pin);

    // Initialize the ADC.
    // We are only using one channel for detecting battery level
    let adc_pin = p.P0_31.degrade_saadc();
    let is_charging_pin = Input::new(p.P0_07, embassy_nrf::gpio::Pull::Up);
    let saadc = init_adc(adc_pin, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // Keyboard config
    let keyboard_device_config = DeviceConfig {
        vid: 0x4c4b,
        pid: 0x4643,
        manufacturer: "RMK",
        product_name: "roBa-rmk",
        serial_number: "vial:f64c2b3c:000001",
    };
    let vial_config = VialConfig::new(VIAL_KEYBOARD_ID, VIAL_KEYBOARD_DEF, &[(0, 0)]);
    let ble_battery_config = BleBatteryConfig::new(Some(is_charging_pin), true, None, false);
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 6,
        // clear_storage: true,
        // clear_layout: true,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        vial_config,
        ble_battery_config,
        storage_config,
    };

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let mut behavior_config = BehaviorConfig::default();
    behavior_config.morse.enable_flow_tap = true;
    let mut encoder_map: [[rmk::types::action::EncoderAction; _]; _] = keymap::get_default_encoder_map();
    let mut key_config = PositionalConfig::default();
    let (keymap, mut storage) = initialize_encoder_keymap_and_storage(
        &mut default_keymap,
        &mut encoder_map,
        flash,
        &storage_config,
        &mut behavior_config,
        &mut key_config,
    )
    .await;

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::new();
    let mut matrix = CentralMatrix::<_, _, _, 0, 6, 4, 5, true>::new(row_pins, col_pins, debouncer);
    // let mut matrix = TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap);

    // Read peripheral address from storage
    let peripheral_addrs = read_peripheral_addresses::<1, _, 4, 11, 8, 1>(&mut storage).await;

    // Initialize the encoder processor
    let mut adc_device = NrfAdc::new(
        saadc,
        [AnalogEventType::Battery],
        embassy_time::Duration::from_secs(12),
        None,
    );
    let mut batt_proc = BatteryProcessor::new(510, 1510, &keymap);

    // Initialize PMW3610 mouse sensor with scroll layer support
    let pmw3610_config = Pmw3610Config {
        res_cpi: 800,
        smart_mode: true,
        swap_xy: true,
        invert_y: true,
        ..Default::default()
    };
    let pmw3610_sck = Output::new(p.P0_05, embassy_nrf::gpio::Level::High, embassy_nrf::gpio::OutputDrive::Standard);
    let pmw3610_sdio = Flex::new(p.P0_04);
    let pmw3610_cs = Output::new(p.P0_09, embassy_nrf::gpio::Level::High, embassy_nrf::gpio::OutputDrive::Standard);
    let pmw3610_irq = Input::new(p.P0_02, embassy_nrf::gpio::Pull::Up);
    let mut pmw3610_device = Pmw3610Device::new(
        pmw3610_sck,
        pmw3610_sdio,
        pmw3610_cs,
        Some(pmw3610_irq),
        pmw3610_config,
    );

    // Initialize joystick processor
    let mut joystick_proc = JoystickProcessor::new([[1, 0], [0, 1]], [0, 0], 1, &keymap);

    // Initialize scroll layer processor
    let mut scroll_proc = ScrollLayerProcessor::new(&[5, 6], 8, false, &keymap);

    // Initialize the controllers
    let mut capslock_led = KeyboardIndicatorController::new(
        Output::new(
            p.P0_00,
            embassy_nrf::gpio::Level::Low,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        false,
        rmk::types::led_indicator::LedIndicatorType::CapsLock,
    );
    let mut ble_led = BleConnectionLed::new(
        Output::new(
            p.P0_06,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
        Output::new(
            p.P0_26,
            embassy_nrf::gpio::Level::High,
            embassy_nrf::gpio::OutputDrive::Standard,
        ),
    );
    let mut scroll_layer_tracker = ScrollLayerTracker::new();

    // Start
    join4(
        run_devices! (
            (matrix, adc_device, pmw3610_device) => EVENT_CHANNEL,
        ),
        run_processor_chain! {
            EVENT_CHANNEL => [batt_proc, scroll_proc, joystick_proc],
        },
        keyboard.run(),
        join(
            join5(
                run_peripheral_manager::<4, 6, 0, 0, _>(0, &peripheral_addrs, &stack),
                run_rmk(&keymap, driver, &stack, &mut storage, rmk_config),
                scan_peripherals(&stack, &peripheral_addrs),
                capslock_led.event_loop(),
                ble_led.event_loop(),
            ),
            scroll_layer_tracker.event_loop(),
        ),
    )
    .await;
}

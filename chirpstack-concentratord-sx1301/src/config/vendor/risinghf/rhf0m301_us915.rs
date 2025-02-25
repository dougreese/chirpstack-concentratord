use libloragw_sx1301::hal;

use super::super::super::super::config;
use super::super::Configuration;

// Taken from the "generic" us915 configuration as no actual calibration values
// are known. TODO: replace with actual calibration values if known.
pub fn new(conf: &config::Configuration) -> Configuration {
    Configuration {
        radio_count: 2,
        clock_source: 1,
        radio_rssi_offset: vec![-166.0, -166.0],
        radio_tx_enabled: vec![true, false],
        radio_type: vec![hal::RadioType::SX1257, hal::RadioType::SX1257],
        radio_min_max_tx_freq: vec![(923000000, 928000000), (923000000, 928000000)],
        radio_tx_notch_freq: vec![0, 0],
        lora_multi_sf_bandwidth: 125000,
        tx_gain_table: vec![
            // 0
            hal::TxGainConfig {
                pa_gain: 0,
                mix_gain: 8,
                rf_power: -6,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 1
            hal::TxGainConfig {
                pa_gain: 0,
                mix_gain: 10,
                rf_power: -3,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 2
            hal::TxGainConfig {
                pa_gain: 0,
                mix_gain: 12,
                rf_power: 0,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 3
            hal::TxGainConfig {
                pa_gain: 1,
                mix_gain: 8,
                rf_power: 3,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 4
            hal::TxGainConfig {
                pa_gain: 1,
                mix_gain: 10,
                rf_power: 6,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 5
            hal::TxGainConfig {
                pa_gain: 1,
                mix_gain: 12,
                rf_power: 10,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 6
            hal::TxGainConfig {
                pa_gain: 1,
                mix_gain: 13,
                rf_power: 11,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 7
            hal::TxGainConfig {
                pa_gain: 2,
                mix_gain: 9,
                rf_power: 12,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 8
            hal::TxGainConfig {
                pa_gain: 1,
                mix_gain: 15,
                rf_power: 13,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 9
            hal::TxGainConfig {
                pa_gain: 2,
                mix_gain: 10,
                rf_power: 14,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 10
            hal::TxGainConfig {
                pa_gain: 2,
                mix_gain: 11,
                rf_power: 16,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 11
            hal::TxGainConfig {
                pa_gain: 3,
                mix_gain: 9,
                rf_power: 20,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 12
            hal::TxGainConfig {
                pa_gain: 3,
                mix_gain: 10,
                rf_power: 23,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 13
            hal::TxGainConfig {
                pa_gain: 3,
                mix_gain: 11,
                rf_power: 25,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 14
            hal::TxGainConfig {
                pa_gain: 3,
                mix_gain: 12,
                rf_power: 26,
                dig_gain: 0,
                dac_gain: 3,
            },
            // 15
            hal::TxGainConfig {
                pa_gain: 3,
                mix_gain: 14,
                rf_power: 27,
                dig_gain: 0,
                dac_gain: 3,
            },
        ],
        gps_tty_path: None,
        spidev_path: "/dev/spidev0.0".to_string(),
        reset_pin: match conf.gateway.reset_pin {
            0 => Some((0, 7)),
            _ => Some((0, conf.gateway.reset_pin)),
        },
    }
}

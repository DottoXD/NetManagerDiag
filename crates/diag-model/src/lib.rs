// SPDX-License-Identifier: GPL-3.0-or-later

#![forbid(unsafe_code)]

use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Vendor {
    Qualcomm,
    Mediatek,
    Samsung,
}

/// An undecoded vendor log record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawLog {
    pub vendor: Vendor,
    pub slot: Option<u8>,
    pub code: u32,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

/// Serving cell measurement of one LTE carrier (Qualcomm: 0xB17F).
#[derive(Clone, Debug, PartialEq)]
pub struct LteServingMeas {
    pub slot: Option<u8>,
    pub timestamp: u64,
    pub earfcn: u32,
    pub pci: u16,
    pub rsrp_dbm: f32,
    pub avg_rsrp_dbm: f32,
    pub rsrq_db: f32,
    pub rssi_dbm: f32,
}

/// LTE serving cell configuration (Qualcomm: 0xB197).
#[derive(Clone, Debug, PartialEq)]
pub struct LteCellInfo {
    pub slot: Option<u8>,
    pub timestamp: u64,
    pub earfcn: u32,
    pub pci: u16,
    pub dl_bandwidth_prb: u8,
    pub dl_bandwidth_mhz: Option<f32>,
    pub num_antennas: u16,
    pub sfn: u16,
}

/// One SSB beam of an NR cell in the measurement table.
#[derive(Clone, Debug, PartialEq)]
pub struct NrBeam {
    pub ssb_index: u16,
    pub rx_beam: [Option<u16>; 2],
    pub rsrp_dbm: [Option<f32>; 2],
    pub rsrq_db: [Option<f32>; 2],
    /// SCAT's `nr2nr_filtered_beam_*_l3` and `l_2_nr_filtered_tx_beam_*_l3`;
    pub nr2nr_rsrp_dbm: Option<f32>,
    pub nr2nr_rsrq_db: Option<f32>,
    pub l2nr_rsrp_dbm: Option<f32>,
    pub l2nr_rsrq_db: Option<f32>,
}

/// An NR cell (serving or neighbour).
#[derive(Clone, Debug, PartialEq)]
pub struct NrCell {
    pub pci: u16,
    pub pbch_sfn: u16,
    pub rsrp_dbm: Option<f32>,
    pub rsrq_db: Option<f32>,
    pub beams: Vec<NrBeam>,
}

/// One measured NR carrier.
#[derive(Clone, Debug, PartialEq)]
pub struct NrLayer {
    pub nr_arfcn: u32,
    pub serving_pci: Option<u16>,
    pub serving_ssb: Option<u8>,
    /// 2 entries, or 4 in newer devices.
    pub serving_rsrp_dbm: Vec<Option<f32>>,
    pub rx_beam: [Option<u16>; 2],
    pub cells: Vec<NrCell>,
}

/// NR ML1 measurement table (Qualcomm: 0xB97F).
#[derive(Clone, Debug, PartialEq)]
pub struct NrMeasurements {
    pub slot: Option<u8>,
    pub timestamp: u64,
    pub version: (u16, u16),
    pub layers: Vec<NrLayer>,
}

/// An LTE EPS bearer.
#[derive(Clone, Debug, PartialEq)]
pub struct LteBearer {
    pub slot: Option<u8>,
    pub timestamp: u64,
    pub eps_bearer_id: u8,
    pub dedicated: bool,
    pub qci: Option<u8>,
    pub apn: Option<String>,
    pub pdn_type: Option<&'static str>,
    pub apn_ambr_dl_kbps: Option<u64>,
    pub apn_ambr_ul_kbps: Option<u64>,
}

/// What a client wants to receive. Clients ask for features, never for vendor log identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Feature {
    LteServingCell,
    NrCellTable,
    LteBearers,
}

impl Feature {
    pub const ALL: &'static [Feature] = &[Feature::LteServingCell, Feature::NrCellTable, Feature::LteBearers];

    /// Daemon clients protocol names.
    pub fn name(self) -> &'static str {
        match self {
            Feature::LteServingCell => "lte_serving_cell",
            Feature::NrCellTable => "nr_cell_table",
            Feature::LteBearers => "lte_bearers",
        }
    }

    pub fn from_name(s: &str) -> Option<Feature> {
        Feature::ALL.iter().copied().find(|f| f.name() == s)
    }
}

/// Things a backend can report.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Event {
    RawLog(RawLog),
    LteServingMeas(LteServingMeas),
    LteCellInfo(LteCellInfo),
    NrMeasurements(NrMeasurements),
    LteBearer(LteBearer),
}

impl Event {
    pub fn feature(&self) -> Option<Feature> {
        match self {
            Event::LteServingMeas(_) | Event::LteCellInfo(_) => Some(Feature::LteServingCell),
            Event::NrMeasurements(_) => Some(Feature::NrCellTable),
            Event::LteBearer(_) => Some(Feature::LteBearers),
            _ => None,
        }
    }
}

/// A baseband backend; the daemon picks one by probing the device.
pub trait Baseband {
    fn vendor(&self) -> Vendor;

    /// Human-readable transport in use.
    fn transport(&self) -> String;

    /// Features this backend can currently deliver.
    fn supported(&self) -> Vec<Feature>;

    /// Make certain `features` active.
    fn enable(&mut self, features: &[Feature]) -> io::Result<()>;

    /// SIM slots this backend has seen tagged data for so far.
    fn slots_seen(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Wait up to `timeout_ms` for data.
    fn poll(&mut self, timeout_ms: i32, out: &mut Vec<Event>) -> io::Result<()>;

    /// Turn everything off and release the modem interface.
    fn close(&mut self) -> io::Result<()>;
}

//! Tools to retrieve Internet-time using NTP protocol.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, Utc};
use crate::error;
use std::io;

const NTP_CLIENT: u8 = 3;
const NTP_HEADER_SIZE: usize = 48; // 12 words
const NTP_TO_UNIX_EPOCH: i64 = 2_208_988_800;

const LEAP_SHIFT: i32 = 6;
const VERSION_SHIFT: i32 = 3;

#[derive(Debug)]
pub struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

impl NTPTimestamp {
    pub fn from_datetime(dt: &DateTime<Utc>) -> NTPTimestamp {
        use std::u32;
        let s = dt.timestamp() as u32;
        let fraction =
            (dt.timestamp_subsec_nanos() as f64 * (u32::MAX as f64 / 1_000_000_000.0)) as u32;
        NTPTimestamp {
            seconds: s + (NTP_TO_UNIX_EPOCH as u32),
            fraction,
        }
    }

    pub fn to_datetime(&self) -> DateTime<Utc> {
        use chrono::TimeZone;
        use std::u32;
        let nanos = (self.fraction as f64 * (1_000_000_000.0 / u32::MAX as f64)) as u32;
        let seconds = self.seconds as i64 - NTP_TO_UNIX_EPOCH;
        Utc.timestamp_opt(seconds, nanos).unwrap()
    }
}

#[derive(Debug)]
pub struct NTPHeader {
    leap: u8,
    version: u8,
    mode: u8,
    pub stratum: u8,
    poll: u8,
    precision: u8,
    root_delay: u32,
    root_dispersion: u32,
    reference_id: u32,
    pub reference_timestamp: NTPTimestamp,
    pub origin_timestamp: NTPTimestamp,
    pub receive_timestamp: NTPTimestamp,
    pub transmit_timestamp: NTPTimestamp,
}

impl NTPHeader {
    pub fn new() -> NTPHeader {
        NTPHeader {
            leap: 0,
            version: 3,
            mode: NTP_CLIENT,
            stratum: 0,
            poll: 0,
            precision: 0,
            root_delay: 0,
            root_dispersion: 0,
            reference_id: 0,
            reference_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            origin_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            receive_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
            transmit_timestamp: NTPTimestamp {
                seconds: 0,
                fraction: 0,
            },
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, error::Error> {
        let mut vec = Vec::<u8>::new();

        (vec.write_u8(self.leap << LEAP_SHIFT | self.version << VERSION_SHIFT | self.mode))?;
        (vec.write_u8(self.stratum))?;
        (vec.write_u8(self.poll))?;
        (vec.write_u8(self.precision))?;
        (vec.write_u32::<BigEndian>(self.root_delay))?;
        (vec.write_u32::<BigEndian>(self.root_dispersion))?;
        (vec.write_u32::<BigEndian>(self.reference_id))?;
        (vec.write_u32::<BigEndian>(self.reference_timestamp.seconds))?;
        (vec.write_u32::<BigEndian>(self.reference_timestamp.fraction))?;
        (vec.write_u32::<BigEndian>(self.origin_timestamp.seconds))?;
        (vec.write_u32::<BigEndian>(self.origin_timestamp.fraction))?;
        (vec.write_u32::<BigEndian>(self.receive_timestamp.seconds))?;
        (vec.write_u32::<BigEndian>(self.receive_timestamp.fraction))?;
        (vec.write_u32::<BigEndian>(self.transmit_timestamp.seconds))?;
        (vec.write_u32::<BigEndian>(self.transmit_timestamp.fraction))?;
        Ok(vec)
    }

    pub fn decode(size: usize, buf: &[u8]) -> Result<NTPHeader, error::Error> {
        let mut reader = io::Cursor::new(buf);
        let mut header = NTPHeader::new();

        if size < NTP_HEADER_SIZE {
            return Err(error::Error::UnexpectedSize(NTP_HEADER_SIZE, size));
        }

        let leap_version_mode = (reader.read_u8())?;
        header.leap = (leap_version_mode >> LEAP_SHIFT) & 0b11;
        header.version = (leap_version_mode >> VERSION_SHIFT) & 0b111;
        header.mode = leap_version_mode & 0b111;
        header.stratum = (reader.read_u8())?;
        header.poll = (reader.read_u8())?;
        header.precision = (reader.read_u8())?;
        header.root_delay = (reader.read_u32::<BigEndian>())?;
        header.root_dispersion = (reader.read_u32::<BigEndian>())?;
        header.reference_id = (reader.read_u32::<BigEndian>())?;
        header.reference_timestamp.seconds = (reader.read_u32::<BigEndian>())?;
        header.reference_timestamp.fraction = (reader.read_u32::<BigEndian>())?;
        header.origin_timestamp.seconds = (reader.read_u32::<BigEndian>())?;
        header.origin_timestamp.fraction = (reader.read_u32::<BigEndian>())?;
        header.receive_timestamp.seconds = (reader.read_u32::<BigEndian>())?;
        header.receive_timestamp.fraction = (reader.read_u32::<BigEndian>())?;
        header.transmit_timestamp.seconds = (reader.read_u32::<BigEndian>())?;
        header.transmit_timestamp.fraction = (reader.read_u32::<BigEndian>())?;

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use std::u32;
    #[test]
    fn from_datetime() {
        let dt = Utc::today().and_hms_milli(0, 0, 5, 500);
        let ts = NTPTimestamp::from_datetime(&dt);
        assert_eq!(u32::MAX / 2, ts.fraction);
    }

    #[test]
    fn to_datetime() {
        let dt = Utc::now();
        let ts = NTPTimestamp::from_datetime(&dt);
        let dt_again = ts.to_datetime();
        let diff = dt - dt_again;
        let max_diff = Duration::nanoseconds(10);
        assert!(diff < max_diff);
    }

    #[test]
    fn ntp_header_size() {
        use std::mem::size_of;
        let size = size_of::<NTPHeader>();
        assert_eq!(52, size);
    }
}

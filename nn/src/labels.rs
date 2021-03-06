use std::fs::File;
use std::io::{Read, Seek, SeekFrom, self};
use std::iter;
use std::ops::{Index, Range};
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt};
use cast::From as _0;
use linalg::prelude::*;

/// Labels corresponding to a set of images
pub struct Labels {
    data: Vec<u8>,
    num_classes: u32,
    size: u32,
}

impl Labels {
    /// Loads a subset of the labels stored in `path`
    pub fn load<P>(path: P, subset: Range<u32>) -> io::Result<Labels> where P: AsRef<Path> {
        Labels::load_(path.as_ref(), subset)
    }

    fn load_(path: &Path, Range { start, end }: Range<u32>) -> io::Result<Labels> {
        /// Magic number expected in the header
        const MAGIC: u32 = 2049;

        assert!(start < end);

        let mut file = try!(File::open(path));

        // Parse the header: MAGIC NLABELS
        assert_eq!(try!(file.read_u32::<BigEndian>()), MAGIC);
        let nlabels = try!(file.read_u32::<BigEndian>());

        assert!(end <= nlabels);

        let buf_size = usize::from_(end - start);
        let mut buf: Vec<_> = iter::repeat(0).take(buf_size).collect();

        try!(file.seek(SeekFrom::Current(i64::from_(start))));

        assert_eq!(try!(file.read(&mut buf)), buf_size);

        let num_classes = u32::from_(*buf.iter().max().unwrap_or(&0)) + 1;

        Ok(Labels {
            data: buf,
            num_classes: num_classes,
            size: end - start,
        })
    }

    /// Returns the number of classes
    pub fn num_classes(&self) -> u32 {
        self.num_classes
    }

    pub fn to_dataset(&self) -> Mat<f64> {
        let mut m = Mat::zeros((self.size, self.num_classes));

        for (mut r, &label) in m.rows_mut().zip(&self.data) {
            r[u32::from_(label)] = 1.;
        }

        m
    }
}

impl Index<u32> for Labels {
    type Output = u8;

    fn index(&self, i: u32) -> &u8 {
        &self.data[usize::from_(i)]
    }
}

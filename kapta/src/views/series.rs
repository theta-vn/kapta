use crate::coords::ProjCoord;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SeriesPC {
    pub series: Vec<ProjCoord>,
}

impl SeriesPC {
    pub fn contain(&self, check: ProjCoord) -> bool {
        for data in &self.series {
            if check.similar(*data) {
                return false;
            }
        }
        true
    }

    pub fn diff(&self, new: Vec<ProjCoord>) -> Self {
        let mut valid: Vec<ProjCoord> = [].to_vec();
        for data in new {
            if !self.contain(data) {
                valid.push(data)
            }
        }
        Self { series: valid }
    }

    pub fn extent(&self, new: Vec<ProjCoord>) -> Self {
        let mut old = self.series.clone();
        let mut valid: Vec<ProjCoord> = [].to_vec();
        for data in new {
            if self.contain(data) {
                valid.push(data)
            }
        }
        if valid.len() > 0 {
            old.append(&mut valid);
        }

        Self { series: old }
    }
}

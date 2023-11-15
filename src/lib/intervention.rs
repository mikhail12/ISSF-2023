#[derive(PartialEq, Clone, Copy)]
pub struct Intervention {
    intType: InterventionType,
    startTime: usize,
    endTime: usize,
    active: bool
}

impl Intervention {
    pub fn new(inter: InterventionType, startTime: usize, endTime: usize) -> Intervention {
        let res = Intervention {intType: inter, startTime, endTime, active: false};
        res
    }

    pub fn setAct(&mut self, active: bool) {
        self.active = active;
    }

    pub fn getStart(&mut self) -> usize {
        self.startTime
    }

    pub fn getEnd(&mut self) -> usize {
        self.endTime
    }

    pub fn getType(&mut self) -> InterventionType {
        self.intType
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum InterventionType {
    Kkkkkzone,
    Mask
}